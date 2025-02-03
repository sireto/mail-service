-- Your SQL goes here
CREATE TABLE "servers" (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
    "host" VARCHAR NOT NULL,
    "smtp_username" VARCHAR NOT NULL,
    "smtp_password" VARCHAR NOT NULL,
    "namespace_id" UUID REFERENCES namespaces(id) ON DELETE CASCADE NOT NULL,
    "created_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    "updated_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);