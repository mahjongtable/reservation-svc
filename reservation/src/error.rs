use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("an error occurred while operating the database.")]
    DatabaseError(#[from] sqlx::Error),

    #[error("invalid start or end timestamp.")]
    InvalidTimestampRange,

    #[error("invalid timestamp.")]
    InvalidTimestamp,

    #[error("reservation status is unknown")]
    UnknownReservationStatus,
}

// impl From<sqlx::Error> for RepositoryError {
//     fn from(value: sqlx::Error) -> Self {
//         todo!()
//     }
// }