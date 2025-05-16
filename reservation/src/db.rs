pub mod postgresql {
    use abi::pb::reservation::{Query as ReservationQuery, Reservation};
    use sqlx::{PgPool, postgres::types::PgRange};

    use crate::{RepositoryError, ReservationId, ReservationRepository};

    pub struct PgReservationRepository {
        pool: PgPool,
    }

    #[async_trait::async_trait]
    impl ReservationRepository for PgReservationRepository {
        async fn create(&self, reservation: Reservation) -> Result<Reservation, RepositoryError> {
            let sql = "INSERT INTO reservation (user_id, resource_id, status, time_span, note) VALUES ($1, $2, $3, $4, $5) RETURNING id";

            // handle none value of start or end time.
            if reservation.start_at.is_none() || reservation.end_at.is_none() {
                return Err(RepositoryError::InvalidTimestampRange);
            }

            let start_at = reservation.start_at.unwrap().seconds;
            let end_at = reservation.end_at.unwrap().seconds;

            if start_at >= end_at {
                return Err(RepositoryError::InvalidTimestampRange);
            }

            let time_span = PgRange::from(start_at..end_at);

            let new_id: i64 = sqlx::query_as(sql)
                .bind(reservation.user_id)
                .bind(reservation.resource_id)
                .bind(reservation.status)
                .bind(time_span)
                .bind(reservation.note)
                .fetch_one(&self.pool)
                .await
                .map_err(|err| RepositoryError::DatabaseError(err))
                .map(|row: (i64,)| row.0)
                .unwrap();

            println!("new_id: {}", new_id);

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

pub mod mysql {
}