-- migration file for creating the 'namespace' table

CREATE EXTENSION IF NOT EXISTS pgcrypto;


CREATE TABLE IF NOT EXISTS "namespaces" (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),  
    "name" VARCHAR NOT NULL,
    "created_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    "updated_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);

INSERT INTO "namespaces" (id, name)
VALUES ('e3bda5cf-760e-43ea-8e9a-c2c3c5f95b82', 'Namespace 01');