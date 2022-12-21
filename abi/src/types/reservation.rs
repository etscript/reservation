use chrono::{DateTime, FixedOffset, Utc};
use sqlx::postgres::types::PgRange;

use crate::{
    convert_to_timestamp, convert_to_utc_timestamp, Error, Reservation, ReservationStatus,
};

impl Reservation {
    pub fn new_pending(
        uid: impl Into<String>,
        rid: impl Into<String>,
        start: DateTime<FixedOffset>,
        end: DateTime<FixedOffset>,
        note: impl Into<String>,
    ) -> Self {
        Reservation {
            id: 0,
            user_id: uid.into(),
            resource_id: rid.into(),
            start: Some(convert_to_timestamp(start.with_timezone(&Utc))),
            end: Some(convert_to_timestamp(end.with_timezone(&Utc))),
            note: note.into(),
            status: ReservationStatus::Pending as i32,
        }
    }

    pub fn validate(&self) -> Result<(), Error> {
        if self.id != 0 {
            return Err(Error::InvalidUserId(self.id.to_string()));
        }
        if self.resource_id.is_empty() {
            return Err(Error::InvalidResourceId(self.resource_id.clone()));
        }
        if self.start.is_none() || self.end.is_none() {
            return Err(Error::InvalidReservation);
        }
        let start = convert_to_utc_timestamp(self.start.as_ref().unwrap().clone());
        let end = convert_to_utc_timestamp(self.end.as_ref().unwrap().clone());
        if start > end {
            return Err(Error::InvalidReservation);
        }
        Ok(())
    }

    pub fn get_timestamp(&self) -> PgRange<DateTime<Utc>> {
        let start = convert_to_utc_timestamp(self.start.as_ref().unwrap().clone());
        let end = convert_to_utc_timestamp(self.end.as_ref().unwrap().clone());
        (start..end).into()
    }
}
