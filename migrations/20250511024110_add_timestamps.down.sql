-- Add down migration script here
ALTER TABLE `users` DROP COLUMN `created_at`;
ALTER TABLE `users` DROP COLUMN `updated_at`;
ALTER TABLE `users` DROP COLUMN `deleted_at`;