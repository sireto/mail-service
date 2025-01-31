CREATE TABLE "campaigns" (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    "campaign_name" VARCHAR NOT NULL,
    "template_id" UUID NOT NULL REFERENCES templateS(id) ON DELETE CASCADE,
    "namespace_id" UUID NOT NULL REFERENCES namespaceS(id) ON DELETE CASCADE,
    "status" VARCHAR NOT NULL,
    "campaign_senders" UUID REFERENCES campaign_senders(id) ON DELETE SET NULL,
    "scheduled_at" TIMESTAMPTZ NOT NULL,
    "created_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    "updated_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);