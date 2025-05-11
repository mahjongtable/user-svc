-- Add up migration script here
ALTER TABLE `users` ADD COLUMN `created_at` DATETIME DEFAULT NULL;
ALTER TABLE `users` ADD COLUMN `updated_at` DATETIME DEFAULT NULL;
ALTER TABLE `users` ADD COLUMN `deleted_at` DATETIME DEFAULT NULL;