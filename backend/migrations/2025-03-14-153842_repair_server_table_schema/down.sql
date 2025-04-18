ALTER TABLE servers DROP COLUMN IF EXISTS aws_credentials;

ALTER TABLE servers ALTER COLUMN server_type TYPE VARCHAR(255) USING server_type::TEXT;

DROP TYPE IF EXISTS server_type;
