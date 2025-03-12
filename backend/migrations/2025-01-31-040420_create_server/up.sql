CREATE TYPE tls_type AS ENUM ('STARTTLS', 'SSL/TLS', 'NONE');
CREATE TYPE server_type AS ENUM ('SMTP', 'AWS');

CREATE TABLE "servers" (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
    "active" BOOLEAN DEFAULT TRUE NOT NULL,  
    "host" VARCHAR NOT NULL,
    "smtp_username" VARCHAR NOT NULL,
    "smtp_password" VARCHAR NOT NULL,
    "namespace_id" UUID REFERENCES namespaces(id) ON DELETE CASCADE NOT NULL,
    "tls_type" tls_type NOT NULL DEFAULT 'STARTTLS',
    "port" SMALLINT NOT NULL DEFAULT 25,
    "server_type" server_type NOT NULL DEFAULT 'SMTP',
    "aws_credentials" JSONB DEFAULT '{}',
    "created_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    "updated_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL, 
    CONSTRAINT check_smtp_fields CHECK (
        (server_type != 'SMTP') OR 
        (host IS NOT NULL AND host != '')
    ),
    CONSTRAINT check_aws_fields CHECK (
        (server_type != 'AWS') OR
        (aws_credentials ? 'access_key_id' AND aws_credentials ? 'secret_access_key' AND aws_credentials ? 'region')
    )
);
