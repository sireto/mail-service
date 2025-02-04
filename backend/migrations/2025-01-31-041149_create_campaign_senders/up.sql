-- Your SQL goes here
CREATE TABLE "campaign_senders" (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
    "server_id" UUID REFERENCES servers(id) ON DELETE CASCADE NOT NULL,
    "from_name" VARCHAR NOT NULL,
    "from_email" VARCHAR NOT NULL,
    "created_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    "updated_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);
