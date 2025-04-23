-- Down Migration: Remove "default_from_email" column from the servers table
ALTER TABLE "servers"
DROP COLUMN "default_from_email";
