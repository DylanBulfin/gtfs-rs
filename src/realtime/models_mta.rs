use gtfs_macros::{gtfs_realtime_enum, gtfs_realtime_model};

use crate::realtime::models::TimeRange;

#[cfg(feature = "realtime_parse")]
use {
    serde::{Deserialize, Serialize},
    serde_repr::{Deserialize_repr, Serialize_repr},
};

#[gtfs_realtime_enum(crate::realtime::parse::protos::gtfs::nyct_trip_descriptor::Direction)]
pub enum Direction {
    NORTH = 1,
    EAST = 2,
    SOUTH = 3,
    WEST = 4,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::TripReplacementPeriod)]
pub struct TripReplacementPeriod {
    pub route_id: Option<String>,
    #[gtfs(mf)]
    pub replacement_period: Option<TimeRange>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::NyctFeedHeader)]
pub struct NyctFeedHeader {
    #[gtfs(required)]
    pub nyct_subway_version: String,
    #[gtfs(vec)]
    pub trip_replacement_period: Vec<TripReplacementPeriod>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::NyctTripDescriptor)]
pub struct NyctTripDescriptor {
    pub train_id: Option<String>,
    pub is_assigned: Option<bool>,
    #[gtfs(Enum)]
    pub direction: Option<Direction>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::NyctStopTimeUpdate)]
pub struct NyctStopTimeUpdate {
    pub scheduled_track: Option<String>,
    pub actual_track: Option<String>,
}
