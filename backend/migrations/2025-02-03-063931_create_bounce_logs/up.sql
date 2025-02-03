CREATE TABLE "bounce_logs" (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
    "contact_id" UUID REFERENCES contacts(id) ON DELETE CASCADE NOT NULL,
    "campaign_id" UUID REFERENCES campaigns(id) ON DELETE SET NULL,
    "at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    "kind" TEXT NOT NULL,
    "reason" TEXT NOT NULL
);