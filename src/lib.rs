use chrono::NaiveDateTime;
use gtfs_macros::gtfs_schedule_model;
use serde::{Deserializer, de::DeserializeOwned};

#[cfg(feature = "schedule")]
pub mod schedule;
#[cfg(feature = "realtime")]
pub mod realtime;

pub mod error;

fn deser_timestamp<'de, D>(d: D) -> Result<NaiveDateTime, D::Error> where D: Deserializer<'de> {
    unimplemented!()
}

#[gtfs_schedule_model]
pub struct Test {
    pub(crate) a: u32,
    #[gtfs_chrono(chrono::NaiveDateTime, deser_timestamp)]
    pub(crate) timestamp: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
