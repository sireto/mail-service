-- Rollback: Remove queuing and rate-limiting fields from mails table

ALTER TABLE mails
DROP COLUMN scheduled_at,
DROP COLUMN attempts,
DROP COLUMN last_error;