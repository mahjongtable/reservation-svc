mod db;
mod error;

use abi::pb::reservation::{Query as ReservationQuery, Reservation, ReservationStatus};
pub use error::RepositoryError;

pub type ReservationId = String;
pub type UserId = String;
pub type ResourceId = String;

#[async_trait::async_trait]
pub trait ReservationRepository {
    async fn create(&self, reservation: Reservation) -> Result<Reservation, RepositoryError>;
    async fn change_status(
        &self,
        reservation_id: ReservationId,
    ) -> Result<Reservation, RepositoryError>;
    async fn update_note(
        &self,
        reservation_id: ReservationId,
        note: String,
    ) -> Result<Reservation, RepositoryError>;
    async fn delete(&self, reservation_id: ReservationId) -> Result<(), RepositoryError>;
    async fn get(&self, reservation_id: ReservationId) -> Result<Reservation, RepositoryError>;
    async fn query(&self, query: ReservationQuery) -> Result<Vec<Reservation>, RepositoryError>;
}
