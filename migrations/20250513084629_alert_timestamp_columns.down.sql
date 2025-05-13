-- Add down migration script here
ALTER TABLE `users` MODIFY `created_at` DATETIME DEFAULT NULL;
ALTER TABLE `users` MODIFY `updated_at` DATETIME DEFAULT NULL;