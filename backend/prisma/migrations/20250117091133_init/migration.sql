/*
  Warnings:

  - Made the column `content_html` on table `Template` required. This step will fail if there are existing NULL values in that column.

*/
-- AlterTable
ALTER TABLE "Template" ALTER COLUMN "content_html" SET NOT NULL;
