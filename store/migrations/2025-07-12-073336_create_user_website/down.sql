-- This file should undo anything in `up.sql`
-- DropForeignKey
ALTER TABLE "website_tick" DROP CONSTRAINT "website_tick_region_id_fkey";
ALTER TABLE "website_tick" DROP CONSTRAINT "website_tick_website_id_fkey";
ALTER TABLE "website" DROP CONSTRAINT "website_user_id_fkey";

-- DropTable
DROP TABLE IF EXISTS "website_tick";
DROP TABLE IF EXISTS "region";
DROP TABLE IF EXISTS "website";
DROP TABLE IF EXISTS "user";

-- DropEnum
DROP TYPE IF EXISTS "website_status";