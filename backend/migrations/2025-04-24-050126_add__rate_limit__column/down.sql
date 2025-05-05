-- Down Migration: remove the rate_limit column
ALTER TABLE "servers"
DROP COLUMN IF EXISTS "rate_limit";