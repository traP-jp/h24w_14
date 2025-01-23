CREATE TABLE IF NOT EXISTS `users` (
    `id` BINARY(16) NOT NULL,
    `name` VARCHAR(255) NOT NULL,
    `display_name` VARCHAR(255) NOT NULL,
    `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    `updated_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`)
);

CREATE TABLE IF NOT EXISTS `message` {
    `id` BINARY(16) NOT NULL,
    `user_id` BINARY(16) NOT NULL,
    `content` TEXT NOT NULL,
    `position_x` INT NOT NULL,
    `position_y` INT NOT NULL,
    `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    `expires_at` TIMESTAMP NOT NULL,
    PRIMARY KEY (`id`),
};
