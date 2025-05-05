-- Up Migration: add the rate_limit column with a default value
ALTER TABLE "servers"
ADD COLUMN "rate_limit" INTEGER NOT NULL DEFAULT 60;