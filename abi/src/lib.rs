pub mod pb;

use std::fmt::Display;

use chrono::{DateTime, FixedOffset, Utc};
use pb::reservation::{Reservation, ReservationStatus};
use prost_types::Timestamp;
use sqlx::{FromRow, Postgres, Row, postgres::PgRow};

impl Reservation {
    pub fn new_pending(
        user_id: String,
        resource_id: String,
        start_at: DateTime<FixedOffset>,
        end_at: DateTime<FixedOffset>,
        note: String,
    ) -> Self {
        Self {
            id: String::new(),
            user_id,
            resource_id,
            status: ReservationStatus::StatusPending as i32,
            start_at: Some(convert_to_timestamp_from(start_at.with_timezone(&Utc))),
            end_at: Some(convert_to_timestamp_from(end_at.with_timezone(&Utc))),
            note,
        }
    }
}

impl FromRow<'_, PgRow> for Reservation {
    fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.get("id"),
            user_id: row.get("user_id"),
            resource_id: row.get("resource_id"),
            status: row.get("status"),
            // TODO: need to convert database timestamp to prost::Timestamp.
            start_at: Some(todo!()),
            // TODO: need to convert database timestamp to prost::Timestamp.
            end_at: Some(todo!()),
            note: row.get("note"),
        })
    }
}

pub fn convert_to_utc_datetime_from(value: Timestamp) -> Result<DateTime<Utc>, &'static str> {
    let datetime =
        DateTime::from_timestamp(value.seconds, value.nanos as u32).ok_or("invalid timestamp")?;

    let datetime =
        DateTime::<Utc>::from_naive_utc_and_offset(datetime.naive_utc(), datetime.offset().clone());

    Ok(datetime)
}

pub fn convert_to_timestamp_from(datetime: DateTime<Utc>) -> Timestamp {
    Timestamp {
        seconds: datetime.timestamp(),
        nanos: datetime.timestamp_subsec_nanos() as _,
    }
}

impl Display for ReservationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReservationStatus::StatusUnknown => write!(f, "unknown"),
            ReservationStatus::StatusPending => write!(f, "pending"),
            ReservationStatus::StatusConfirmed => write!(f, "confirmed"),
            ReservationStatus::StatusBlocked => write!(f, "blocked"),
        }
    }
}
