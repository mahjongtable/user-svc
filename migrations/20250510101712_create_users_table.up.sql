-- Add up migration script here
CREATE TABLE IF NOT EXISTS `users` (
    `id` INT AUTO_INCREMENT PRIMARY KEY,
    `username` VARCHAR(100) DEFAULT NULL,
    `gender` TINYINT DEFAULT NULL,
    `avatar_url` VARCHAR(100) DEFAULT NULL,
    `email` VARCHAR(100) NOT NULL,
    `cellphone_number` VARCHAR(100) DEFAULT NULL,
    `password` VARCHAR(100) DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8;