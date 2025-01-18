/*
  Warnings:

  - You are about to drop the column `template_name` on the `Template` table. All the data in the column will be lost.
  - Added the required column `name` to the `Template` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE "Template" DROP COLUMN "template_name",
ADD COLUMN     "name" TEXT NOT NULL;
