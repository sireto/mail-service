-- Migration: Add queuing and rate-limiting fields to mails table

ALTER TABLE mails
ADD COLUMN "scheduled_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
ADD COLUMN "attempts" INTEGER DEFAULT 0 NOT NULL,
ADD COLUMN "last_error" TEXT;