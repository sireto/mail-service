-- Your SQL goes here
CREATE TABLE "campaign_senders" (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    "server_id" UUID REFERENCES servers(id) ON DELETE CASCADE,
    "from_name" VARCHAR,
    "from_email" VARCHAR NOT NULL,
    "created_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);