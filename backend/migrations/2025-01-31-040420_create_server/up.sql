-- Your SQL goes here
CREATE TYPE tls_type AS ENUM ('STARTTLS', 'SSL/TLS', 'NONE');

CREATE TABLE "servers" (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
    "host" VARCHAR NOT NULL,
    "smtp_username" VARCHAR NOT NULL,
    "smtp_password" VARCHAR NOT NULL,
    "namespace_id" UUID REFERENCES namespaces(id) ON DELETE CASCADE NOT NULL,
    "tls_type" tls_type NOT NULL DEFAULT 'STARTTLS',
    "port" SMALLINT NOT NULL DEFAULT 25,
    "created_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    "updated_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);
