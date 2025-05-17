pub mod postgresql {
    use abi::{
        convert_to_utc_datetime_from,
        pb::reservation::{Query as ReservationQuery, Reservation, ReservationStatus},
    };
    use sqlx::{PgPool, postgres::types::PgRange};

    use crate::{RepositoryError, ReservationId, ReservationRepository};

    pub struct PgReservationRepository {
        pool: PgPool,
    }

    impl PgReservationRepository {
        pub fn new(pool: PgPool) -> Self {
            Self { pool }
        }
    }

    #[async_trait::async_trait]
    impl ReservationRepository for PgReservationRepository {
        async fn create(&self, reservation: Reservation) -> Result<Reservation, RepositoryError> {
            // handle none value of start or end time.
            if reservation.start_at.is_none() || reservation.end_at.is_none() {
                return Err(RepositoryError::InvalidTimestampRange);
            }

            let start_at = reservation
                .start_at
                .ok_or(RepositoryError::InvalidTimestamp)
                .and_then(|t| Ok(convert_to_utc_datetime_from(t)))?
                .map_err(|_| RepositoryError::InvalidTimestamp)?;

            let end_at = reservation
                .end_at
                .ok_or(RepositoryError::InvalidTimestamp)
                .and_then(|t| Ok(convert_to_utc_datetime_from(t)))?
                .map_err(|_| RepositoryError::InvalidTimestamp)?;

            if start_at >= end_at {
                return Err(RepositoryError::InvalidTimestampRange);
            }

            let time_span = PgRange::from(start_at..end_at);

            println!("{}", reservation.status.to_string());

            let res = sqlx::query("INSERT INTO reservation.reservations (user_id, resource_id, status, time_span, note) VALUES ($1, $2, $3::reservation.reservation_status, $4, $5) RETURNING *")
                .bind(reservation.user_id)
                .bind(reservation.resource_id)
                .bind(ReservationStatus::try_from(reservation.status).unwrap().to_string())
                .bind(time_span)
                .bind(reservation.note)
                .fetch_one(&self.pool)
                .await?;

            println!("got uuid: {:?}", res);

            Ok(Reservation::default())
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

#[cfg(test)]
pub mod tests {
    use abi::pb::reservation::{Reservation, ReservationStatus};
    use chrono::{DateTime, Utc};
    use sqlx::PgPool;

    use crate::{
        RepositoryError, ReservationRepository, convert::DatetimeWrapper,
        db::postgresql::PgReservationRepository,
    };

    #[sqlx::test(migrations = "../migrations")]
    async fn creating_should_work(pool: PgPool) -> sqlx::Result<()> {
        let pool = pool;
        let pg_repo = PgReservationRepository::new(pool);

        let start_at_dt = DatetimeWrapper::new(Utc::now());
        let end_at_dt = DatetimeWrapper::new(Utc::now());

        let result = pg_repo
            .create(Reservation {
                id: "reservation_id_123".to_string(),
                user_id: "user_id_221".to_string(),
                resource_id: "resource_id_300".to_string(),
                status: ReservationStatus::StatusPending as i32,
                // todo: make a mock timestamp here.
                start_at: Some(start_at_dt.into()),
                // todo: make a mock timestamp here.
                end_at: Some(end_at_dt.into()),
                note: "This is a note for the reservation".to_string(),
            })
            .await
            .map_err(|repo_err| match repo_err {
                RepositoryError::DatabaseError(err) => err,
                _ => todo!(),
            })?;

        println!("reservation created successfully: {:?}", result);

        Ok(())
    }

    #[sqlx::test(migrations = "../migrations")]
    async fn conflict_should_be_reject(pool: PgPool) -> sqlx::Result<()> {
        let pool = pool;
        let pg_repo = PgReservationRepository::new(pool);

        let reservation1 = Reservation::new_pending(
            "lucas_id".to_string(),
            "lucas_resource_id".to_string(),
            "2022-12-25T15:00:00-0700".parse().unwrap(),
            "2022-12-28T12:00:00-0700".parse().unwrap(),
            "It's a first reservation to be held".to_string(),
        );

        pg_repo.create(reservation1).await.unwrap();

        let reservation2 = Reservation::new_pending(
            "hjkl1_id".to_string(),
            "lucas_resource_id".to_string(),
            "2022-12-26T15:00:00-0700".parse().unwrap(),
            "2022-12-30T12:00:00-0700".parse().unwrap(),
            "It's a second reservation to be held".to_string(),
        );

        pg_repo.create(reservation2).await.unwrap();

        Ok(())
    }
}
