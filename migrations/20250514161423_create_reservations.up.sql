-- Add up migration script here
-- CREATE SCHEMA `reservation`;

-- CREATE TYPE `reservation`.`reservation_status` AS ENUM ('unknown', 'pending', 'confirmed', 'blocked');
-- CREATE TYPE `reservation`.`reservation_changelog_opr_type` AS ENUM ('unknown', 'create', 'update', 'delete');

-- CREATE TABLE `reservation`.`reservations` IF NOT EXISTS (
--     `id` UUID NOT NULL DEFAULT uuid_generate_v4(),
--     `user_id` VARCHAR(64) NOT NULL,
--     `resource_id` VARCHAR(64) NOT NULL,
--     `status` `reservation`.`reservation_status` NOT NULL DEFAULT 'pending',
--     -- The column 'time_span' already contains 'start_at' and 'end_at'.
--     `time_span` TSTZRANGE NOT NULL,
--     -- `start_at` TIMESTAMPTZ NOT NULL,
--     -- `end_at` TIMESTAMPTZ NOT NULL,
--     `note` TEXT,
--     PRIMARY KEY (`id`),
--     -- It will check whether the column `time_span` overlap.
--     CONSTRAINT `reservations_conflict` EXCLUDE USING gist (`resource_id` WITH =, `time_span` WITH &&)
-- );


-- CREATE TABLE `reservation`.`reservation_changelog` (
--     `id` SERIAL NOT NULL,
--     `reservation_id` UUID NOT NULL,
--     `operation` `reservation`.`reservation_changelog_opr_type` NOT NULL,
--     `created_at` TIMESTAMPZ DEFAULT NULL
-- );

-- CREATE INDEX `reservations_resource_id_idx` ON `reservation`.`reservations` (`resource_id`);
-- CREATE INDEX `reservations_user_id_idx` ON `reservation`.`reservations` (`user_id`);

-- -- trigger for create/update/delete each reservation.
-- CREATE OR REPLACE FUNCTION reservation.reservations_trigger() RETURNS TRIGGER AS
-- $$
-- BEGIN
--     IF TG_OP = 'INSERT' THEN
--         -- insert a new changelog for operation 'INSERT'.
--         INSERT INTO `reservation`.`reservation_changelog` (`reservation_id`, `operation`) VALUES (NEW.id, 'create');
--     IF TG_OP = 'UPDATE' THEN
--         -- insert a new changelog for operation 'UPDATE'.
--         IF OLD.status <> NEW.status THEN
--             INSERT INTO `reservation`.`reservation_changelog` (`reservation_id`, `operation`) VALUES (NEW.id, 'update');
--         END IF;
--     ELSEIF TG_OP = 'DELETE' THEN
--         -- insert a new changelog for operation 'DELETE'.
--         INSERT INTO `reservation`.`reservation_changelog` (`reservation_id`, `operation`) VALUES (OLD.id, 'delete');
--     END IF;

--     -- notify a channel called reservation_changelog table.
--     NOTIFY reservation_changelog;

--     RETURN NULL;
-- END
-- $$
-- LANGUAGE PLPGSQL;

-- CREATE TRIGGER reservations_trigger;