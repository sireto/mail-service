-- Indexes, the missing mails.server_id foreign key, and a rate_limit range constraint.
--
-- Every statement is guarded so the migration is safe to apply to an existing database
-- that may already contain rows violating the new rules.

-- M9: the mail worker polls `WHERE status = 'queued'` every few seconds and joins
-- campaigns, campaign_senders, contacts and bounce_logs. None of those columns were
-- indexed, so each poll was a sequential scan of the whole mails table.
CREATE INDEX IF NOT EXISTS idx_mails_status ON mails (status);
CREATE INDEX IF NOT EXISTS idx_mails_status_sent_at ON mails (status, sent_at);
CREATE INDEX IF NOT EXISTS idx_mails_contact_id ON mails (contact_id);
CREATE INDEX IF NOT EXISTS idx_mails_campaign_id ON mails (campaign_id);
CREATE INDEX IF NOT EXISTS idx_mails_server_id ON mails (server_id);
CREATE INDEX IF NOT EXISTS idx_mails_sent_at ON mails (sent_at DESC);

CREATE INDEX IF NOT EXISTS idx_bounce_logs_mail_id ON bounce_logs (mail_id);
CREATE INDEX IF NOT EXISTS idx_bounce_logs_contact_id ON bounce_logs (contact_id);
CREATE INDEX IF NOT EXISTS idx_bounce_logs_campaign_id ON bounce_logs (campaign_id);

CREATE INDEX IF NOT EXISTS idx_list_contacts_contact_id ON list_contacts (contact_id);
CREATE INDEX IF NOT EXISTS idx_campaign_lists_list_id ON campaign_lists (list_id);

CREATE INDEX IF NOT EXISTS idx_servers_namespace_id ON servers (namespace_id);
CREATE INDEX IF NOT EXISTS idx_lists_namespace_id ON lists (namespace_id);
CREATE INDEX IF NOT EXISTS idx_templates_namespace_id ON templates (namespace_id);
CREATE INDEX IF NOT EXISTS idx_campaigns_namespace_id ON campaigns (namespace_id);
CREATE INDEX IF NOT EXISTS idx_campaign_senders_server_id ON campaign_senders (server_id);

-- Case-insensitive contact search (the API uses ILIKE on these three columns).
CREATE INDEX IF NOT EXISTS idx_contacts_email_lower ON contacts (lower(email));

-- M10: mails.server_id was added as a bare UUID with no foreign key, unlike every other
-- relation in the schema. Null out any row that already points at a deleted server so the
-- constraint can be added, then match the ON DELETE SET NULL behaviour of template_id and
-- campaign_id.
UPDATE mails
SET server_id = NULL
WHERE server_id IS NOT NULL
  AND server_id NOT IN (SELECT id FROM servers);

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint WHERE conname = 'mails_server_id_fkey'
    ) THEN
        ALTER TABLE mails
        ADD CONSTRAINT mails_server_id_fkey
        FOREIGN KEY (server_id) REFERENCES servers(id) ON DELETE SET NULL;
    END IF;
END $$;

-- M11: rate_limit is read straight into a u32 quota. A negative value wrapped to an
-- enormous number and silently disabled rate limiting, which is the worst possible
-- failure mode for a misconfigured value. The application also clamps, but the database
-- should not be able to hold a value that cannot mean anything.
UPDATE servers SET rate_limit = 1 WHERE rate_limit < 1;
UPDATE servers SET rate_limit = 10000 WHERE rate_limit > 10000;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint WHERE conname = 'servers_rate_limit_range'
    ) THEN
        ALTER TABLE servers
        ADD CONSTRAINT servers_rate_limit_range
        CHECK (rate_limit >= 1 AND rate_limit <= 10000);
    END IF;
END $$;


-- Reconcile pre-existing mail statuses before the retry worker is enabled.
--
-- This is a deployment safety step, not a schema change. Until now nothing in the codebase
-- ever wrote "sent": process_mails marked a mail "submitted" and send_single_email marked
-- it "submitted" again on success, so every successfully delivered mail is sitting at
-- "submitted" permanently. process_submitted_mails, which retries anything "submitted",
-- older than 30 minutes and under the attempt cap, was implemented but never spawned.
--
-- Spawning it (which this release does) without this step would make the first tick after
-- deployment re-send the entire mail history to real recipients. A "submitted" row older
-- than the deployment is not distinguishable from a delivered one, and the overwhelming
-- majority did deliver, so they are settled as "sent": mislabelling a handful of old
-- failures is far cheaper than re-sending live email.
--
-- Anything genuinely in flight is younger than the retry worker's 30 minute staleness
-- window, so this deliberately leaves recent rows alone for the worker to handle.
UPDATE mails
SET status = 'sent'
WHERE status = 'submitted'
  AND sent_at < CURRENT_TIMESTAMP - INTERVAL '30 minutes';

-- Belt and braces: cap attempts on anything still "submitted" and already old, so a row
-- this statement somehow misses cannot be retried either.
UPDATE mails
SET attempts = 3
WHERE status = 'submitted'
  AND attempts < 3
  AND sent_at < CURRENT_TIMESTAMP - INTERVAL '30 minutes';
