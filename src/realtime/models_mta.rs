use crate::realtime::models::TimeRange;

#[cfg(feature = "realtime_parse")]
use {
    serde::{Deserialize, Serialize},
    serde_repr::{Deserialize_repr, Serialize_repr},
};

#[cfg(feature = "realtime_parse")]
macro_rules! derives {
    ($($item:item)+) => {
        $(
            #[derive(Serialize, Deserialize, Debug, Clone)]
            $item
        )+
    };
}

#[cfg(not(feature = "realtime_parse"))]
macro_rules! derives {
    ($($item:item)+) => {
        $(
            #[derive(Debug, Clone)]
            $item
        )+
    };
}

#[cfg(feature = "realtime_parse")]
macro_rules! derives_enum {
    ($($item:item)+) => {
        $(
            #[derive(Serialize_repr, Deserialize_repr, Debug, Clone)]
            #[repr(u8)]
            $item
        )+
    };
}

#[cfg(not(feature = "realtime_parse"))]
macro_rules! derives_enum {
    ($($item:item)+) => {
        $(
            #[derive(Debug, Clone)]
            $item
        )+
    };
}

derives_enum!(
    pub enum Direction {
        North = 1,
        East = 2,
        South = 3,
        West = 4,
    }
);

derives!(
    pub struct TripReplacementPeriod {
        pub route_id: Option<String>,
        pub replacement_period: Option<TimeRange>,
    }

    pub struct NyctFeedHeader {
        pub nyct_subway_version: String,
        pub trip_replacement_periods: Vec<TripReplacementPeriod>,
    }

    pub struct NyctTripDescriptor {
        pub train_id: Option<String>,
        pub is_assigned: Option<bool>,
        pub direction: Option<Direction>,
    }

    pub struct NyctStopTimeUpdate {
        pub scheduled_track: Option<String>,
        pub actual_track: Option<String>,
    }
);
