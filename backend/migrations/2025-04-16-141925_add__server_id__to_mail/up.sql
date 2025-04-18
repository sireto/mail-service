-- Up Migration: Add "server_id" column to the mails table
ALTER TABLE "mails"
ADD COLUMN "server_id" UUID;