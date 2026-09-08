-- Reverting cannot restore the contacts, mails or bounce_logs that up.sql cleared. It only
-- restores the shape of the table: global email uniqueness, and no tenant column.
CREATE INDEX IF NOT EXISTS idx_contacts_email_lower ON contacts (lower(email));

DROP INDEX IF EXISTS idx_contacts_namespace_email_lower;
DROP INDEX IF EXISTS idx_contacts_namespace_id;

ALTER TABLE contacts DROP CONSTRAINT IF EXISTS contacts_namespace_id_email_key;
ALTER TABLE contacts DROP COLUMN IF EXISTS namespace_id;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint WHERE conname = 'contacts_email_key'
    ) THEN
        ALTER TABLE contacts ADD CONSTRAINT contacts_email_key UNIQUE (email);
    END IF;
END $$;
