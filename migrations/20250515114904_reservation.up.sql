-- Add up migration script here
CREATE TYPE reservation.reservation_status AS ENUM ('unknown', 'pending', 'confirmed', 'blocked');
CREATE TYPE reservation.reservation_changelog_opr_type AS ENUM ('unknown', 'create', 'update', 'delete');

CREATE TABLE reservation.reservations (
    id UUID NOT NULL DEFAULT uuid_generate_v4(),
    user_id VARCHAR(64) NOT NULL,
    resource_id VARCHAR(64) NOT NULL,
    status reservation.reservation_status NOT NULL DEFAULT 'pending',
    -- The column 'time_span' already contains 'start_at' and 'end_at'.
    time_span TSTZRANGE NOT NULL,
    -- start_at TIMESTAMPTZ NOT NULL,
    -- end_at TIMESTAMPTZ NOT NULL,
    note TEXT,
    PRIMARY KEY (id),
    -- It will check whether the column time_span overlap.
    CONSTRAINT reservations_conflict EXCLUDE USING gist (resource_id WITH =, time_span WITH &&)
);

CREATE INDEX reservations_resource_id_idx ON reservation.reservations (resource_id);
CREATE INDEX reservations_user_id_idx ON reservation.reservations (user_id);