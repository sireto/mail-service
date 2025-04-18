-- Down Migration: Remove "server_id" column from the mails table
ALTER TABLE "mails"
DROP COLUMN "server_id";
