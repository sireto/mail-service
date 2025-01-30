
CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE "lists" (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(), 
    "namespace_id" UUID NOT NULL, 
    "name" VARCHAR NOT NULL, 
    "description" VARCHAR, 
    "created_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL, 
    "updated_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL, 

    CONSTRAINT "lists_namespace_id_fkey" FOREIGN KEY ("namespace_id") REFERENCES "namespaces"("id") ON DELETE CASCADE
);