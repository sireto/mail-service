/*
  Warnings:

  - You are about to drop the column `bounceReason` on the `BounceLog` table. All the data in the column will be lost.
  - You are about to drop the column `bounceType` on the `BounceLog` table. All the data in the column will be lost.
  - You are about to drop the column `bouncedAt` on the `BounceLog` table. All the data in the column will be lost.
  - You are about to drop the column `campaignId` on the `BounceLog` table. All the data in the column will be lost.
  - You are about to drop the column `contactId` on the `BounceLog` table. All the data in the column will be lost.
  - You are about to drop the column `campaignName` on the `Campaign` table. All the data in the column will be lost.
  - You are about to drop the column `campaignSenderId` on the `Campaign` table. All the data in the column will be lost.
  - You are about to drop the column `createdAt` on the `Campaign` table. All the data in the column will be lost.
  - You are about to drop the column `namespaceId` on the `Campaign` table. All the data in the column will be lost.
  - You are about to drop the column `scheduledAt` on the `Campaign` table. All the data in the column will be lost.
  - You are about to drop the column `templateId` on the `Campaign` table. All the data in the column will be lost.
  - You are about to drop the column `updatedAt` on the `Campaign` table. All the data in the column will be lost.
  - You are about to drop the column `createdAt` on the `CampaignSender` table. All the data in the column will be lost.
  - You are about to drop the column `fromEmail` on the `CampaignSender` table. All the data in the column will be lost.
  - You are about to drop the column `fromName` on the `CampaignSender` table. All the data in the column will be lost.
  - You are about to drop the column `serverId` on the `CampaignSender` table. All the data in the column will be lost.
  - You are about to drop the column `updatedAt` on the `CampaignSender` table. All the data in the column will be lost.
  - You are about to drop the column `createdAt` on the `Contact` table. All the data in the column will be lost.
  - You are about to drop the column `firstName` on the `Contact` table. All the data in the column will be lost.
  - You are about to drop the column `lastName` on the `Contact` table. All the data in the column will be lost.
  - You are about to drop the column `updatedAt` on the `Contact` table. All the data in the column will be lost.
  - You are about to drop the column `createdAt` on the `List` table. All the data in the column will be lost.
  - You are about to drop the column `namespaceId` on the `List` table. All the data in the column will be lost.
  - You are about to drop the column `updatedAt` on the `List` table. All the data in the column will be lost.
  - The primary key for the `ListContact` table will be changed. If it partially fails, the table could be left without primary key constraint.
  - You are about to drop the column `contactId` on the `ListContact` table. All the data in the column will be lost.
  - You are about to drop the column `createdAt` on the `ListContact` table. All the data in the column will be lost.
  - You are about to drop the column `listId` on the `ListContact` table. All the data in the column will be lost.
  - You are about to drop the column `updatedAt` on the `ListContact` table. All the data in the column will be lost.
  - You are about to drop the column `campaignId` on the `Mail` table. All the data in the column will be lost.
  - You are about to drop the column `contactId` on the `Mail` table. All the data in the column will be lost.
  - You are about to drop the column `sentAt` on the `Mail` table. All the data in the column will be lost.
  - You are about to drop the column `templateId` on the `Mail` table. All the data in the column will be lost.
  - You are about to drop the column `createdAt` on the `Namespace` table. All the data in the column will be lost.
  - You are about to drop the column `updatedAt` on the `Namespace` table. All the data in the column will be lost.
  - You are about to drop the column `createdAt` on the `Server` table. All the data in the column will be lost.
  - You are about to drop the column `namespaceId` on the `Server` table. All the data in the column will be lost.
  - You are about to drop the column `smtpPassword` on the `Server` table. All the data in the column will be lost.
  - You are about to drop the column `smtpUsername` on the `Server` table. All the data in the column will be lost.
  - You are about to drop the column `updatedAt` on the `Server` table. All the data in the column will be lost.
  - You are about to drop the column `contentHtml` on the `Template` table. All the data in the column will be lost.
  - You are about to drop the column `contentPlaintext` on the `Template` table. All the data in the column will be lost.
  - You are about to drop the column `createdAt` on the `Template` table. All the data in the column will be lost.
  - You are about to drop the column `namespaceId` on the `Template` table. All the data in the column will be lost.
  - You are about to drop the column `templateData` on the `Template` table. All the data in the column will be lost.
  - You are about to drop the column `templateName` on the `Template` table. All the data in the column will be lost.
  - You are about to drop the column `updatedAt` on the `Template` table. All the data in the column will be lost.
  - Added the required column `bounce_reason` to the `BounceLog` table without a default value. This is not possible if the table is not empty.
  - Added the required column `bounce_type` to the `BounceLog` table without a default value. This is not possible if the table is not empty.
  - Added the required column `bounced_at` to the `BounceLog` table without a default value. This is not possible if the table is not empty.
  - Added the required column `campaign_id` to the `BounceLog` table without a default value. This is not possible if the table is not empty.
  - Added the required column `contact_id` to the `BounceLog` table without a default value. This is not possible if the table is not empty.
  - Added the required column `campaign_sender_id` to the `Campaign` table without a default value. This is not possible if the table is not empty.
  - Added the required column `name` to the `Campaign` table without a default value. This is not possible if the table is not empty.
  - Added the required column `namespace_id` to the `Campaign` table without a default value. This is not possible if the table is not empty.
  - Added the required column `scheduled_at` to the `Campaign` table without a default value. This is not possible if the table is not empty.
  - Added the required column `template_id` to the `Campaign` table without a default value. This is not possible if the table is not empty.
  - Added the required column `updated_at` to the `Campaign` table without a default value. This is not possible if the table is not empty.
  - Added the required column `from_email` to the `CampaignSender` table without a default value. This is not possible if the table is not empty.
  - Added the required column `from_name` to the `CampaignSender` table without a default value. This is not possible if the table is not empty.
  - Added the required column `server_id` to the `CampaignSender` table without a default value. This is not possible if the table is not empty.
  - Added the required column `updated_at` to the `CampaignSender` table without a default value. This is not possible if the table is not empty.
  - Added the required column `first_name` to the `Contact` table without a default value. This is not possible if the table is not empty.
  - Added the required column `last_name` to the `Contact` table without a default value. This is not possible if the table is not empty.
  - Added the required column `updated_at` to the `Contact` table without a default value. This is not possible if the table is not empty.
  - Added the required column `namespace_id` to the `List` table without a default value. This is not possible if the table is not empty.
  - Added the required column `updated_at` to the `List` table without a default value. This is not possible if the table is not empty.
  - Added the required column `contact_id` to the `ListContact` table without a default value. This is not possible if the table is not empty.
  - Added the required column `list_id` to the `ListContact` table without a default value. This is not possible if the table is not empty.
  - Added the required column `updated_at` to the `ListContact` table without a default value. This is not possible if the table is not empty.
  - Added the required column `campaign_id` to the `Mail` table without a default value. This is not possible if the table is not empty.
  - Added the required column `contact_id` to the `Mail` table without a default value. This is not possible if the table is not empty.
  - Added the required column `sent_at` to the `Mail` table without a default value. This is not possible if the table is not empty.
  - Added the required column `template_id` to the `Mail` table without a default value. This is not possible if the table is not empty.
  - Added the required column `updated_at` to the `Namespace` table without a default value. This is not possible if the table is not empty.
  - Added the required column `namespace_id` to the `Server` table without a default value. This is not possible if the table is not empty.
  - Added the required column `smtp_password` to the `Server` table without a default value. This is not possible if the table is not empty.
  - Added the required column `smtp_username` to the `Server` table without a default value. This is not possible if the table is not empty.
  - Added the required column `updated_at` to the `Server` table without a default value. This is not possible if the table is not empty.
  - Added the required column `namespace_id` to the `Template` table without a default value. This is not possible if the table is not empty.
  - Added the required column `template_data` to the `Template` table without a default value. This is not possible if the table is not empty.
  - Added the required column `template_name` to the `Template` table without a default value. This is not possible if the table is not empty.
  - Added the required column `updated_at` to the `Template` table without a default value. This is not possible if the table is not empty.

*/
-- DropForeignKey
ALTER TABLE "BounceLog" DROP CONSTRAINT "BounceLog_campaignId_fkey";

-- DropForeignKey
ALTER TABLE "BounceLog" DROP CONSTRAINT "BounceLog_contactId_fkey";

-- DropForeignKey
ALTER TABLE "Campaign" DROP CONSTRAINT "Campaign_campaignSenderId_fkey";

-- DropForeignKey
ALTER TABLE "Campaign" DROP CONSTRAINT "Campaign_namespaceId_fkey";

-- DropForeignKey
ALTER TABLE "Campaign" DROP CONSTRAINT "Campaign_templateId_fkey";

-- DropForeignKey
ALTER TABLE "CampaignSender" DROP CONSTRAINT "CampaignSender_serverId_fkey";

-- DropForeignKey
ALTER TABLE "List" DROP CONSTRAINT "List_namespaceId_fkey";

-- DropForeignKey
ALTER TABLE "ListContact" DROP CONSTRAINT "ListContact_contactId_fkey";

-- DropForeignKey
ALTER TABLE "ListContact" DROP CONSTRAINT "ListContact_listId_fkey";

-- DropForeignKey
ALTER TABLE "Mail" DROP CONSTRAINT "Mail_campaignId_fkey";

-- DropForeignKey
ALTER TABLE "Mail" DROP CONSTRAINT "Mail_contactId_fkey";

-- DropForeignKey
ALTER TABLE "Mail" DROP CONSTRAINT "Mail_templateId_fkey";

-- DropForeignKey
ALTER TABLE "Server" DROP CONSTRAINT "Server_namespaceId_fkey";

-- DropForeignKey
ALTER TABLE "Template" DROP CONSTRAINT "Template_namespaceId_fkey";

-- AlterTable
ALTER TABLE "BounceLog" DROP COLUMN "bounceReason",
DROP COLUMN "bounceType",
DROP COLUMN "bouncedAt",
DROP COLUMN "campaignId",
DROP COLUMN "contactId",
ADD COLUMN     "bounce_reason" TEXT NOT NULL,
ADD COLUMN     "bounce_type" TEXT NOT NULL,
ADD COLUMN     "bounced_at" TIMESTAMP(3) NOT NULL,
ADD COLUMN     "campaign_id" INTEGER NOT NULL,
ADD COLUMN     "contact_id" INTEGER NOT NULL;

-- AlterTable
ALTER TABLE "Campaign" DROP COLUMN "campaignName",
DROP COLUMN "campaignSenderId",
DROP COLUMN "createdAt",
DROP COLUMN "namespaceId",
DROP COLUMN "scheduledAt",
DROP COLUMN "templateId",
DROP COLUMN "updatedAt",
ADD COLUMN     "campaign_sender_id" INTEGER NOT NULL,
ADD COLUMN     "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
ADD COLUMN     "name" TEXT NOT NULL,
ADD COLUMN     "namespace_id" INTEGER NOT NULL,
ADD COLUMN     "scheduled_at" TIMESTAMP(3) NOT NULL,
ADD COLUMN     "template_id" INTEGER NOT NULL,
ADD COLUMN     "updated_at" TIMESTAMP(3) NOT NULL;

-- AlterTable
ALTER TABLE "CampaignSender" DROP COLUMN "createdAt",
DROP COLUMN "fromEmail",
DROP COLUMN "fromName",
DROP COLUMN "serverId",
DROP COLUMN "updatedAt",
ADD COLUMN     "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
ADD COLUMN     "from_email" TEXT NOT NULL,
ADD COLUMN     "from_name" TEXT NOT NULL,
ADD COLUMN     "server_id" INTEGER NOT NULL,
ADD COLUMN     "updated_at" TIMESTAMP(3) NOT NULL;

-- AlterTable
ALTER TABLE "Contact" DROP COLUMN "createdAt",
DROP COLUMN "firstName",
DROP COLUMN "lastName",
DROP COLUMN "updatedAt",
ADD COLUMN     "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
ADD COLUMN     "first_name" TEXT NOT NULL,
ADD COLUMN     "last_name" TEXT NOT NULL,
ADD COLUMN     "updated_at" TIMESTAMP(3) NOT NULL;

-- AlterTable
ALTER TABLE "List" DROP COLUMN "createdAt",
DROP COLUMN "namespaceId",
DROP COLUMN "updatedAt",
ADD COLUMN     "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
ADD COLUMN     "namespace_id" INTEGER NOT NULL,
ADD COLUMN     "updated_at" TIMESTAMP(3) NOT NULL;

-- AlterTable
ALTER TABLE "ListContact" DROP CONSTRAINT "ListContact_pkey",
DROP COLUMN "contactId",
DROP COLUMN "createdAt",
DROP COLUMN "listId",
DROP COLUMN "updatedAt",
ADD COLUMN     "contact_id" INTEGER NOT NULL,
ADD COLUMN     "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
ADD COLUMN     "list_id" INTEGER NOT NULL,
ADD COLUMN     "updated_at" TIMESTAMP(3) NOT NULL,
ADD CONSTRAINT "ListContact_pkey" PRIMARY KEY ("list_id", "contact_id");

-- AlterTable
ALTER TABLE "Mail" DROP COLUMN "campaignId",
DROP COLUMN "contactId",
DROP COLUMN "sentAt",
DROP COLUMN "templateId",
ADD COLUMN     "campaign_id" INTEGER NOT NULL,
ADD COLUMN     "contact_id" INTEGER NOT NULL,
ADD COLUMN     "sent_at" TIMESTAMP(3) NOT NULL,
ADD COLUMN     "template_id" INTEGER NOT NULL;

-- AlterTable
ALTER TABLE "Namespace" DROP COLUMN "createdAt",
DROP COLUMN "updatedAt",
ADD COLUMN     "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
ADD COLUMN     "updated_at" TIMESTAMP(3) NOT NULL;

-- AlterTable
ALTER TABLE "Server" DROP COLUMN "createdAt",
DROP COLUMN "namespaceId",
DROP COLUMN "smtpPassword",
DROP COLUMN "smtpUsername",
DROP COLUMN "updatedAt",
ADD COLUMN     "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
ADD COLUMN     "namespace_id" INTEGER NOT NULL,
ADD COLUMN     "smtp_password" TEXT NOT NULL,
ADD COLUMN     "smtp_username" TEXT NOT NULL,
ADD COLUMN     "updated_at" TIMESTAMP(3) NOT NULL;

-- AlterTable
ALTER TABLE "Template" DROP COLUMN "contentHtml",
DROP COLUMN "contentPlaintext",
DROP COLUMN "createdAt",
DROP COLUMN "namespaceId",
DROP COLUMN "templateData",
DROP COLUMN "templateName",
DROP COLUMN "updatedAt",
ADD COLUMN     "content_html" TEXT,
ADD COLUMN     "content_plaintext" TEXT,
ADD COLUMN     "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
ADD COLUMN     "namespace_id" INTEGER NOT NULL,
ADD COLUMN     "template_data" JSONB NOT NULL,
ADD COLUMN     "template_name" TEXT NOT NULL,
ADD COLUMN     "updated_at" TIMESTAMP(3) NOT NULL;

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
