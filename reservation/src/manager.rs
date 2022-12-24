use crate::{ReservationManager, Rsvp};
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
    async fn change_status(&self, id: i64) -> Result<abi::Reservation, abi::Error> {
        // if current status is pending, change it to confirmed, otherwise do nothing
        let rsvp: abi::Reservation = sqlx::query_as(
            "UPDATE rsvp.reservations SET status = 'confirmed' WHERE id = $1 RETURNING
            id, user_id, resource_id, timespan, note, status",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(rsvp)
    }
    // update reservation
    async fn update(&self, id: i64, note: String) -> Result<abi::Reservation, abi::Error> {
        let rsvp: abi::Reservation = sqlx::query_as(
            "UPDATE rsvp.reservations SET note = $2 WHERE id = $1 RETURNING
            id, user_id, resource_id, timespan, note, status",
        )
        .bind(id)
        .bind(note)
        .fetch_one(&self.pool)
        .await?;

        Ok(rsvp)
    }
    // delete reservation
    async fn delete(&self, id: i64) -> Result<(), abi::Error> {
        sqlx::query("DELETE FROM rsvp.reservations WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
    // get reservation by id
    async fn get(&self, id: i64) -> Result<abi::Reservation, abi::Error> {
        let rsvp: abi::Reservation = sqlx::query_as(
            "SELECT id, user_id, resource_id, timespan, note, status FROM rsvp.reservations WHERE id = $1",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(rsvp)
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
    use sqlx::PgPool;

    use super::*;
    #[sqlx_database_tester::test(pool(variable = "migrated_pool", migrations = "../migrations"))]
    async fn reserve_should_work_for_valid_windows() {
        let (rsvp, _manager) = make_reservation(migrated_pool.clone()).await;
        assert!(rsvp.id > 0);
    }

    #[sqlx_database_tester::test(pool(variable = "migrated_pool", migrations = "../migrations"))]
    async fn reserve_change_status_should_work() {
        let (rsvp, manager) = make_reservation(migrated_pool.clone()).await;
        assert!(rsvp.id > 0);
        let rsvp = manager.change_status(rsvp.id).await.unwrap();
        assert_eq!(rsvp.status, abi::ReservationStatus::Confirmed as i32);
    }

    #[sqlx_database_tester::test(pool(variable = "migrated_pool", migrations = "../migrations"))]
    async fn reserve_update_note_should_work() {
        let (rsvp, manager) = make_reservation(migrated_pool.clone()).await;
        assert!(rsvp.id > 0);
        let rsvp = manager
            .update(rsvp.id, "new note".to_string())
            .await
            .unwrap();
        assert_eq!(rsvp.note, "new note");
    }

    #[sqlx_database_tester::test(pool(variable = "migrated_pool", migrations = "../migrations"))]
    async fn get_reserve_should_work() {
        let (rsvp, manager) = make_reservation(migrated_pool.clone()).await;
        assert!(rsvp.id > 0);
        let rsvp = manager.get(rsvp.id).await.unwrap();
        assert_eq!(rsvp.note, "just note");
    }

    #[sqlx_database_tester::test(pool(variable = "migrated_pool", migrations = "../migrations"))]
    async fn delete_reserve_should_work() {
        let (rsvp, manager) = make_reservation(migrated_pool.clone()).await;
        assert!(rsvp.id > 0);
        manager.delete(rsvp.id).await.unwrap();
        let rsvp = manager.get(rsvp.id).await;
        assert!(rsvp.is_err());
    }

    async fn make_reservation(pool: PgPool) -> (abi::Reservation, ReservationManager) {
        let manager = ReservationManager { pool: pool.clone() };
        let start = "2022-12-25T15:00:00-0700".parse().unwrap();
        let end = "2022-12-28T12:00:00-0700".parse().unwrap();
        let rsvp = abi::Reservation::new_pending("user1", "resource1", start, end, "just note");
        (manager.reserve(rsvp).await.unwrap(), manager)
    }
}
