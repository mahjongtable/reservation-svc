-- Add down migration script here
DROP TABLE reservation.reservations CASCADE;
DROP TYPE reservation.reservation_status;
DROP TYPE reservation.reservation_changelog_opr_type;