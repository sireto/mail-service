-- M7: give contacts a tenant.
--
-- The contacts table had no namespace_id and email was globally UNIQUE, so two namespaces
-- could not hold the same subscriber and every contact was visible to every namespace. No
-- query filter can fix that; it needs the column.
--
-- DESTRUCTIVE. Existing contacts are removed rather than assigned to a namespace, as
-- decided by the maintainer. Note what that takes with it: contacts is the parent of three
-- ON DELETE CASCADE relationships, so this also clears
--   * list_contacts  (every list membership)
--   * mails          (the entire send history, and therefore all analytics)
--   * bounce_logs
-- There is no recovery path other than a database backup. Take one before deploying.
--
-- Clearing the table first is also what lets namespace_id be NOT NULL with no backfill and
-- no placeholder default, which is the point: a nullable tenant column is a tenant column
-- that gets forgotten.

TRUNCATE TABLE contacts CASCADE;

ALTER TABLE contacts
ADD COLUMN IF NOT EXISTS namespace_id UUID NOT NULL
    REFERENCES namespaces(id) ON DELETE CASCADE;

-- Replace global uniqueness on email with per-namespace uniqueness. The old constraint name
-- is whatever Postgres generated for the inline UNIQUE in the original CREATE TABLE, so look
-- it up rather than guessing at it.
DO $$
DECLARE
    existing_constraint text;
BEGIN
    SELECT con.conname INTO existing_constraint
    FROM pg_constraint con
    JOIN pg_class rel ON rel.oid = con.conrelid
    WHERE rel.relname = 'contacts'
      AND con.contype = 'u'
      AND con.conkey = ARRAY[(
          SELECT attnum FROM pg_attribute
          WHERE attrelid = rel.oid AND attname = 'email'
      )]::smallint[];

    IF existing_constraint IS NOT NULL THEN
        EXECUTE format('ALTER TABLE contacts DROP CONSTRAINT %I', existing_constraint);
    END IF;
END $$;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint WHERE conname = 'contacts_namespace_id_email_key'
    ) THEN
        ALTER TABLE contacts
        ADD CONSTRAINT contacts_namespace_id_email_key UNIQUE (namespace_id, email);
    END IF;
END $$;

CREATE INDEX IF NOT EXISTS idx_contacts_namespace_id ON contacts (namespace_id);
-- Contact search is ILIKE on three columns and is now always scoped to one namespace.
CREATE INDEX IF NOT EXISTS idx_contacts_namespace_email_lower ON contacts (namespace_id, lower(email));

-- The standalone lower(email) index from the previous migration is redundant now that every
-- lookup carries a namespace.
DROP INDEX IF EXISTS idx_contacts_email_lower;
