-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE "contacts" (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
    "first_name" VARCHAR NOT NULL,
    "last_name" VARCHAR NOT NULL,
    "email" VARCHAR NOT NULL UNIQUE,
    "attribute" JSONB, 
    "created_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    "updated_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);