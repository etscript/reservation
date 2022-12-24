mod manager;

use async_trait::async_trait;
use sqlx::PgPool;

pub type ReservationId = String;

#[derive(Debug)]
pub struct ReservationManager {
    pool: PgPool,
}
#[async_trait]
pub trait Rsvp {
    // make a reservation
    async fn reserve(&self, reservation: abi::Reservation) -> Result<abi::Reservation, abi::Error>;
    // change reservation status (if current status is pending, change it to confirmed)
    async fn change_status(&self, id: i64) -> Result<abi::Reservation, abi::Error>;
    // update reservation
    async fn update(&self, id: i64, note: String) -> Result<abi::Reservation, abi::Error>;
    // delete reservation
    async fn delete(&self, id: i64) -> Result<(), abi::Error>;
    // get reservation by id
    async fn get(&self, id: i64) -> Result<abi::Reservation, abi::Error>;
    // query reservations
    async fn query(
        &self,
        query: abi::ReservationQuery,
    ) -> Result<Vec<abi::Reservation>, abi::Error>;
}
