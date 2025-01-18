/*
  Warnings:

  - The primary key for the `BounceLog` table will be changed. If it partially fails, the table could be left without primary key constraint.
  - The primary key for the `Campaign` table will be changed. If it partially fails, the table could be left without primary key constraint.
  - The primary key for the `CampaignSender` table will be changed. If it partially fails, the table could be left without primary key constraint.
  - The primary key for the `Contact` table will be changed. If it partially fails, the table could be left without primary key constraint.
  - The primary key for the `List` table will be changed. If it partially fails, the table could be left without primary key constraint.
  - The primary key for the `ListContact` table will be changed. If it partially fails, the table could be left without primary key constraint.
  - The primary key for the `Mail` table will be changed. If it partially fails, the table could be left without primary key constraint.
  - The primary key for the `Namespace` table will be changed. If it partially fails, the table could be left without primary key constraint.
  - The primary key for the `Server` table will be changed. If it partially fails, the table could be left without primary key constraint.
  - The primary key for the `Template` table will be changed. If it partially fails, the table could be left without primary key constraint.

*/
-- DropForeignKey
ALTER TABLE "BounceLog" DROP CONSTRAINT "BounceLog_campaign_id_fkey";

-- DropForeignKey
ALTER TABLE "BounceLog" DROP CONSTRAINT "BounceLog_contact_id_fkey";

-- DropForeignKey
ALTER TABLE "Campaign" DROP CONSTRAINT "Campaign_campaign_sender_id_fkey";

-- DropForeignKey
ALTER TABLE "Campaign" DROP CONSTRAINT "Campaign_namespace_id_fkey";

-- DropForeignKey
ALTER TABLE "Campaign" DROP CONSTRAINT "Campaign_template_id_fkey";

-- DropForeignKey
ALTER TABLE "CampaignSender" DROP CONSTRAINT "CampaignSender_server_id_fkey";

-- DropForeignKey
ALTER TABLE "List" DROP CONSTRAINT "List_namespace_id_fkey";

-- DropForeignKey
ALTER TABLE "ListContact" DROP CONSTRAINT "ListContact_contact_id_fkey";

-- DropForeignKey
ALTER TABLE "ListContact" DROP CONSTRAINT "ListContact_list_id_fkey";

-- DropForeignKey
ALTER TABLE "Mail" DROP CONSTRAINT "Mail_campaign_id_fkey";

-- DropForeignKey
ALTER TABLE "Mail" DROP CONSTRAINT "Mail_contact_id_fkey";

-- DropForeignKey
ALTER TABLE "Mail" DROP CONSTRAINT "Mail_template_id_fkey";

-- DropForeignKey
ALTER TABLE "Server" DROP CONSTRAINT "Server_namespace_id_fkey";

-- DropForeignKey
ALTER TABLE "Template" DROP CONSTRAINT "Template_namespace_id_fkey";

-- AlterTable
ALTER TABLE "BounceLog" DROP CONSTRAINT "BounceLog_pkey",
ALTER COLUMN "id" DROP DEFAULT,
ALTER COLUMN "id" SET DATA TYPE TEXT,
ALTER COLUMN "campaign_id" SET DATA TYPE TEXT,
ALTER COLUMN "contact_id" SET DATA TYPE TEXT,
ADD CONSTRAINT "BounceLog_pkey" PRIMARY KEY ("id");
DROP SEQUENCE "BounceLog_id_seq";

-- AlterTable
ALTER TABLE "Campaign" DROP CONSTRAINT "Campaign_pkey",
ALTER COLUMN "id" DROP DEFAULT,
ALTER COLUMN "id" SET DATA TYPE TEXT,
ALTER COLUMN "campaign_sender_id" SET DATA TYPE TEXT,
ALTER COLUMN "namespace_id" SET DATA TYPE TEXT,
ALTER COLUMN "template_id" SET DATA TYPE TEXT,
ADD CONSTRAINT "Campaign_pkey" PRIMARY KEY ("id");
DROP SEQUENCE "Campaign_id_seq";

-- AlterTable
ALTER TABLE "CampaignSender" DROP CONSTRAINT "CampaignSender_pkey",
ALTER COLUMN "id" DROP DEFAULT,
ALTER COLUMN "id" SET DATA TYPE TEXT,
ALTER COLUMN "server_id" SET DATA TYPE TEXT,
ADD CONSTRAINT "CampaignSender_pkey" PRIMARY KEY ("id");
DROP SEQUENCE "CampaignSender_id_seq";

-- AlterTable
ALTER TABLE "Contact" DROP CONSTRAINT "Contact_pkey",
ALTER COLUMN "id" DROP DEFAULT,
ALTER COLUMN "id" SET DATA TYPE TEXT,
ADD CONSTRAINT "Contact_pkey" PRIMARY KEY ("id");
DROP SEQUENCE "Contact_id_seq";

-- AlterTable
ALTER TABLE "List" DROP CONSTRAINT "List_pkey",
ALTER COLUMN "id" DROP DEFAULT,
ALTER COLUMN "id" SET DATA TYPE TEXT,
ALTER COLUMN "namespace_id" SET DATA TYPE TEXT,
ADD CONSTRAINT "List_pkey" PRIMARY KEY ("id");
DROP SEQUENCE "List_id_seq";

-- AlterTable
ALTER TABLE "ListContact" DROP CONSTRAINT "ListContact_pkey",
ALTER COLUMN "contact_id" SET DATA TYPE TEXT,
ALTER COLUMN "list_id" SET DATA TYPE TEXT,
ADD CONSTRAINT "ListContact_pkey" PRIMARY KEY ("list_id", "contact_id");

-- AlterTable
ALTER TABLE "Mail" DROP CONSTRAINT "Mail_pkey",
ALTER COLUMN "id" DROP DEFAULT,
ALTER COLUMN "id" SET DATA TYPE TEXT,
ALTER COLUMN "campaign_id" SET DATA TYPE TEXT,
ALTER COLUMN "contact_id" SET DATA TYPE TEXT,
ALTER COLUMN "template_id" SET DATA TYPE TEXT,
ADD CONSTRAINT "Mail_pkey" PRIMARY KEY ("id");
DROP SEQUENCE "Mail_id_seq";

-- AlterTable
ALTER TABLE "Namespace" DROP CONSTRAINT "Namespace_pkey",
ALTER COLUMN "id" DROP DEFAULT,
ALTER COLUMN "id" SET DATA TYPE TEXT,
ADD CONSTRAINT "Namespace_pkey" PRIMARY KEY ("id");
DROP SEQUENCE "Namespace_id_seq";

-- AlterTable
ALTER TABLE "Server" DROP CONSTRAINT "Server_pkey",
ALTER COLUMN "id" DROP DEFAULT,
ALTER COLUMN "id" SET DATA TYPE TEXT,
ALTER COLUMN "namespace_id" SET DATA TYPE TEXT,
ADD CONSTRAINT "Server_pkey" PRIMARY KEY ("id");
DROP SEQUENCE "Server_id_seq";

-- AlterTable
ALTER TABLE "Template" DROP CONSTRAINT "Template_pkey",
ALTER COLUMN "id" DROP DEFAULT,
ALTER COLUMN "id" SET DATA TYPE TEXT,
ALTER COLUMN "namespace_id" SET DATA TYPE TEXT,
ADD CONSTRAINT "Template_pkey" PRIMARY KEY ("id");
DROP SEQUENCE "Template_id_seq";

-- AddForeignKey
ALTER TABLE "Template" ADD CONSTRAINT "Template_namespace_id_fkey" FOREIGN KEY ("namespace_id") REFERENCES "Namespace"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Campaign" ADD CONSTRAINT "Campaign_template_id_fkey" FOREIGN KEY ("template_id") REFERENCES "Template"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Campaign" ADD CONSTRAINT "Campaign_namespace_id_fkey" FOREIGN KEY ("namespace_id") REFERENCES "Namespace"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Campaign" ADD CONSTRAINT "Campaign_campaign_sender_id_fkey" FOREIGN KEY ("campaign_sender_id") REFERENCES "CampaignSender"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "CampaignSender" ADD CONSTRAINT "CampaignSender_server_id_fkey" FOREIGN KEY ("server_id") REFERENCES "Server"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Server" ADD CONSTRAINT "Server_namespace_id_fkey" FOREIGN KEY ("namespace_id") REFERENCES "Namespace"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "List" ADD CONSTRAINT "List_namespace_id_fkey" FOREIGN KEY ("namespace_id") REFERENCES "Namespace"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Mail" ADD CONSTRAINT "Mail_contact_id_fkey" FOREIGN KEY ("contact_id") REFERENCES "Contact"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Mail" ADD CONSTRAINT "Mail_template_id_fkey" FOREIGN KEY ("template_id") REFERENCES "Template"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Mail" ADD CONSTRAINT "Mail_campaign_id_fkey" FOREIGN KEY ("campaign_id") REFERENCES "Campaign"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "ListContact" ADD CONSTRAINT "ListContact_list_id_fkey" FOREIGN KEY ("list_id") REFERENCES "List"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "ListContact" ADD CONSTRAINT "ListContact_contact_id_fkey" FOREIGN KEY ("contact_id") REFERENCES "Contact"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "BounceLog" ADD CONSTRAINT "BounceLog_contact_id_fkey" FOREIGN KEY ("contact_id") REFERENCES "Contact"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "BounceLog" ADD CONSTRAINT "BounceLog_campaign_id_fkey" FOREIGN KEY ("campaign_id") REFERENCES "Campaign"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
