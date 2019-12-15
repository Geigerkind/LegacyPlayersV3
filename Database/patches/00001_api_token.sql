CREATE TABLE `main`.`api_token` (
  `id` INT UNSIGNED NOT NULL AUTO_INCREMENT,
  `member_id` INT UNSIGNED NOT NULL,
  `token` VARCHAR(512) NOT NULL,
  `purpose` VARCHAR(128) NOT NULL,
  `exp_date` BIGINT UNSIGNED NOT NULL,
  PRIMARY KEY (`id`),
  KEY `SECONDARY` (`member_id`),
  KEY `TERNARY` (`token`),
  CONSTRAINT `member_must_exist` FOREIGN KEY (`member_id`) REFERENCES `main`.`member`(`id`) ON UPDATE CASCADE ON DELETE CASCADE
);
