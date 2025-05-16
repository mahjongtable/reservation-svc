use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("an error occurred while operating the database.")]
    DatabaseError(#[from] sqlx::Error),

    #[error("invalid start or end timestamp.")]
    InvalidTimestampRange,

    #[error("invalid timestamp.")]
    InvalidTimestamp,
}