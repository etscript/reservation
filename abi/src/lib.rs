mod pb;
pub use pb::*;

use chrono::{DateTime, NaiveDateTime, Utc};
use prost_types::Timestamp;

pub fn convert_to_utc_timestamp(ts: Timestamp) -> DateTime<Utc> {
    DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp(ts.seconds, ts.nanos as _),
        Utc,
    )
}
