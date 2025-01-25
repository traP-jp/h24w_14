CREATE TABLE IF NOT EXISTS `traq_users` (
    `id` BINARY(16) NOT NULL, -- traQ user ID
    `user_id` BINARY(16) NOT NULL, -- App user ID
    `bot` BOOLEAN NOT NULL,
    `bio` TEXT NOT NULL,
    `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    `updated_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`)
);

CREATE TABLE IF NOT EXISTS `traq_messages` (
    `id` BINARY(16) NOT NULL, -- traQ message ID
    `message_id` BINARY(16) NOT NULL, -- App message ID
    `channel_id` BINARY(16) NOT NULL, -- App channel ID
    `user_id` BINARY(16) NOT NULL, -- App user ID
    `content` TEXT NOT NULL,
    `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    `updated_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`)
);
