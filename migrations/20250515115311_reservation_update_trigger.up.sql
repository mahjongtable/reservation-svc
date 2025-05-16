-- Add up migration script here
CREATE TABLE reservation.reservation_changelog (
    id SERIAL NOT NULL,
    reservation_id UUID NOT NULL,
    operation reservation.reservation_changelog_opr_type NOT NULL
    -- created_at TIMESTAMPTZ DEFAULT NULL
);

-- trigger for create/update/delete each reservation.
CREATE OR REPLACE FUNCTION reservation.reservations_trigger() RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        -- insert a new changelog for operation 'INSERT'.
        INSERT INTO reservation.reservation_changelog (reservation_id, operation) VALUES (NEW.id, 'create');
    ELSIF TG_OP = 'UPDATE' THEN
        -- insert a new changelog for operation 'UPDATE'.
        IF OLD.status <> NEW.status THEN
            INSERT INTO reservation.reservation_changelog (reservation_id, operation) VALUES (NEW.id, 'update');
        END IF;
    ELSIF TG_OP = 'DELETE' THEN
        -- insert a new changelog for operation 'DELETE'.
        INSERT INTO reservation.reservation_changelog (reservation_id, operation) VALUES (OLD.id, 'delete');
    END IF;

    -- notify a channel called reservation_changelog table.
    NOTIFY reservation_changelog;

    RETURN NULL;
END;
$$ LANGUAGE PLPGSQL;

CREATE TRIGGER reservations_trigger
    AFTER INSERT OR UPDATE OR DELETE ON reservation.reservations
    FOR EACH ROW EXECUTE PROCEDURE reservation.reservations_trigger();