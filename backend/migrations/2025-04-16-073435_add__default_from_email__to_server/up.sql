-- Up Migration: Add "default_from_email" column to the servers table
ALTER TABLE "servers"
ADD COLUMN "default_from_email" VARCHAR NOT NULL DEFAULT '';