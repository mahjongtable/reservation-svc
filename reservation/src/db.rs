pub mod postgresql {
    use abi::pb::reservation::{Query as ReservationQuery, Reservation};
    use sqlx::{PgPool, postgres::types::PgRange};

    use crate::{RepositoryError, ReservationId, ReservationRepository, convert::DatetimeWrapper};

    pub struct PgReservationRepository {
        pool: PgPool,
    }

    #[async_trait::async_trait]
    impl ReservationRepository for PgReservationRepository {
        async fn create(&self, reservation: Reservation) -> Result<Reservation, RepositoryError> {
            // handle none value of start or end time.
            if reservation.start_at.is_none() || reservation.end_at.is_none() {
                return Err(RepositoryError::InvalidTimestampRange);
            }

            let start_at_timestamp = reservation
                .start_at
                .ok_or(RepositoryError::InvalidTimestamp)?;
            let end_at_timestamp = reservation
                .end_at
                .ok_or(RepositoryError::InvalidTimestamp)?;

            let datetime_wrapper: DatetimeWrapper = start_at_timestamp
                .try_into()
                .map_err(|_| RepositoryError::InvalidTimestamp)?;
            let start_at = datetime_wrapper.into_inner();

            let datetime_wrapper: DatetimeWrapper = end_at_timestamp
                .try_into()
                .map_err(|_| RepositoryError::InvalidTimestamp)?;
            let end_at = datetime_wrapper.into_inner();

            if start_at >= end_at {
                return Err(RepositoryError::InvalidTimestampRange);
            }

            let time_span = PgRange::from(start_at..end_at);

            let uuid = sqlx::query!(
                "INSERT INTO reservation.reservations (user_id, resource_id, status, time_span, note) VALUES ($1, $2, $3, $4, $5) RETURNING id",
                reservation.user_id,
                reservation.resource_id,
                reservation.status as i32,
                time_span,
                reservation.note
            ).fetch_one(&self.pool).await.map_err(|db_err| RepositoryError::DatabaseError(db_err)).map(|record| record.id)?;

            println!("got uuid: {}", uuid);

            todo!()
        }

        async fn change_status(
            &self,
            reservation_id: ReservationId,
        ) -> Result<Reservation, RepositoryError> {
            todo!()
        }

        async fn update_note(
            &self,
            reservation_id: ReservationId,
            note: String,
        ) -> Result<Reservation, RepositoryError> {
            todo!()
        }

        async fn delete(&self, reservation_id: ReservationId) -> Result<(), RepositoryError> {
            todo!()
        }

        async fn get(&self, reservation_id: ReservationId) -> Result<Reservation, RepositoryError> {
            todo!()
        }

        async fn query(
            &self,
            query: ReservationQuery,
        ) -> Result<Vec<Reservation>, RepositoryError> {
            todo!()
        }
    }
}

pub mod mysql {}
