use chrono::{DateTime, Utc};
use prost_types::Timestamp;

pub struct DatetimeWrapper(DateTime<Utc>);

impl DatetimeWrapper {
    // pub fn new(dt: DateTime<Utc>) -> Self {
    //     Self(dt)
    // }

    pub fn into_inner(self) -> DateTime<Utc> {
        self.0
    }
}

impl TryFrom<Timestamp> for DatetimeWrapper {
    type Error = &'static str;

    fn try_from(value: Timestamp) -> Result<Self, Self::Error> {
        let datetime = DateTime::from_timestamp(value.seconds, value.nanos as u32)
            .ok_or("invalid timestamp")?;

        let new_datetime = DateTime::<Utc>::from_naive_utc_and_offset(
            datetime.naive_utc(),
            datetime.offset().clone(),
        );

        Ok(Self(new_datetime))
    }
}