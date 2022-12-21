use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Database error")]
    DbError(#[from] sqlx::Error),
    #[error("Invalid userid {0}")]
    InvalidUserId(String),
    #[error("Invalid resourceid {0}")]
    InvalidResourceId(String),
    #[error("Invalid start or end time for the reservation")]
    InvalidReservation,
    #[error("unknown data store error")]
    Unknown,
}
