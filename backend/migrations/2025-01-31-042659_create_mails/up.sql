-- Create enum for the status of the mail...
-- CREATE TYPE "mail_status_enum" AS ENUM ('draft', 'pending', 'sent', 'failed');

CREATE TABLE "mails" (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
    "mail_message" TEXT NOT NULL,
    "contact_id" UUID REFERENCES contacts(id) ON DELETE CASCADE NOT NULL,
    "template_id" UUID REFERENCES templates(id) ON DELETE SET NULL,
    "campaign_id" UUID REFERENCES campaigns(id) ON DELETE SET NULL,
    "sent_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    "status" TEXT NOT NULL
);