CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE "templates" (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    "namespace_id" UUID NOT NULL,
    "name" VARCHAR NOT NULL,
    "template_data" JSONB NOT NULL,
    "content_plaintext" TEXT,
    "content_html" TEXT NOT NULL,
    "created_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    "updated_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- Adding foreign key relationship to the 'namespace' table
ALTER TABLE "templates"
    ADD CONSTRAINT "template_namespace_id_fkey" FOREIGN KEY ("namespace_id")
    REFERENCES "namespaces"("id") ON DELETE CASCADE;