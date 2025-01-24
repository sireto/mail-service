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


INSERT INTO "templates" (namespace_id, name, template_data, content_plaintext, content_html, created_at, updated_at)
VALUES (
    'e3bda5cf-760e-43ea-8e9a-c2c3c5f95b82', 
    'Welcome Email 001', 
    '{"key1": "value1", "key2": "value2"}', 
    'Hello, welcome to our service!', 
    '<h1>Hello</h1><p>Welcome to our service!</p>', 
    '2025-01-24 05:15:21.356911+00', 
    '2025-01-24 05:15:21.356911+00'
);
