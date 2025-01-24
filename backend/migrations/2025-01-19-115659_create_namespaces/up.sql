-- migration file for creating the 'namespace' table

CREATE EXTENSION IF NOT EXISTS pgcrypto;


CREATE TABLE IF NOT EXISTS "namespaces" (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),  
    "name" VARCHAR NOT NULL,
    "created_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    "updated_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);
