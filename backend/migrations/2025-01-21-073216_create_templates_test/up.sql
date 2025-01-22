
CREATE TABLE IF NOT EXISTS "Template" (
  "id" UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  "namespace_id" UUID NOT NULL,
  "name" TEXT NOT NULL,
  "template_data" JSONB NOT NULL,
  "content_plaintext" TEXT,
  "content_html" TEXT NOT NULL,
  "created_at" TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
  "updated_at" TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
  CONSTRAINT "Template_updated_at_check" CHECK ("updated_at" >= "created_at"),
  CONSTRAINT "fk_namespace" FOREIGN KEY ("namespace_id") REFERENCES "Namespace" ("id") ON DELETE CASCADE
);

