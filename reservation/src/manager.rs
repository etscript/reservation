use crate::{ReservationId, ReservationManager, Rsvp};
use async_trait::async_trait;
use sqlx::Row;

#[async_trait]
impl Rsvp for ReservationManager {
    // make a reservation
    async fn reserve(&self, mut rsvp: abi::Reservation) -> Result<abi::Reservation, abi::Error> {
        rsvp.validate()?;

        let status = abi::ReservationStatus::from_i32(rsvp.status)
            .unwrap_or(abi::ReservationStatus::Pending);

        let timespan = rsvp.get_timestamp();
        // generate a insert sql for the reservation
        let sql = "INSERT INTO rsvp.reservations (user_id, resource_id, timespan, note, status) VALUES ($1, $2,
            $3, $4, $5::rsvp.reservation_status) RETURNING id";
        // execute the sql
        let id = sqlx::query(sql)
            .bind(&rsvp.user_id.clone())
            .bind(&rsvp.resource_id.clone())
            .bind(timespan)
            .bind(&rsvp.note.clone())
            .bind(status.to_string())
            .fetch_one(&self.pool)
            .await?
            .get(0);
        rsvp.id = id;
        Ok(rsvp)
    }

    // change reservation status (if current status is pending, change it to confirmed)
    async fn change_status(&self, _id: ReservationId) -> Result<abi::Reservation, abi::Error> {
        todo!()
    }
    // update reservation
    async fn update(
        &self,
        _id: ReservationId,
        _note: String,
    ) -> Result<abi::Reservation, abi::Error> {
        todo!()
    }
    // delete reservation
    async fn delete(&self, _id: ReservationId) -> Result<abi::Reservation, abi::Error> {
        todo!()
    }
    // get reservation by id
    async fn get(&self, _id: ReservationId) -> Result<abi::Reservation, abi::Error> {
        todo!()
    }
    // query reservations
    async fn query(
        &self,
        _query: abi::ReservationQuery,
    ) -> Result<Vec<abi::Reservation>, abi::Error> {
        todo!()
    }
}

impl ReservationManager {
    pub async fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[sqlx_database_tester::test(pool(variable = "migrated_pool", migrations = "../migrations"))]
    async fn reserve_should_work_for_valid_windows() {
        let manager = ReservationManager {
            pool: migrated_pool.clone(),
        };
        let start = "2022-12-25T15:00:00-0700".parse().unwrap();
        let end = "2022-12-28T12:00:00-0700".parse().unwrap();
        let rsvp = abi::Reservation::new_pending("user1", "resource1", start, end, "just note");
        let rsvp = manager.reserve(rsvp).await.unwrap();
        assert!(rsvp.id > 0);
    }

    // #[sqlx_database_tester::test(pool(variable = "migrated_pool", migrations = "../migrations"))]
    // async fn reserve_should_reject_if_id_is_not_empty() {
    //     todo!()
    // }
}
