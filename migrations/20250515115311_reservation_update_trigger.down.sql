-- Add down migration script here
DROP TRIGGER reservations_trigger ON reservation.reservations;
DROP FUNCTION reservation.reservations_trigger();
DROP TABLE reservation.reservation_changelog;