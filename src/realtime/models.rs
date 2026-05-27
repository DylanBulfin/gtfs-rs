#[cfg(feature = "chrono")]
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
#[cfg(feature = "chrono_tz")]
use chrono_tz::Tz;

#[cfg(feature = "realtime_parse")]
use {
    serde::{Deserialize, Serialize},
    serde_repr::{Deserialize_repr, Serialize_repr},
};

#[cfg(feature = "realtime_mta")]
use crate::realtime::models_mta::{NyctFeedHeader, NyctStopTimeUpdate, NyctTripDescriptor};

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
    pub enum Incrementality {
        FullDataset = 0,
        Differential = 1,
    }

    pub enum DropoffPickupType {
        Regular = 0,
        None = 1,
        PhoneAgency = 2,
        CoordinateWithDriver = 3,
    }

    pub enum OccupancyStatus {
        Empty = 0,
        ManySeatsAvailable = 1,
        FewSeatsAvailable = 2,
        StandingRoomOnly = 3,
        CrushedStandingRoomOnly = 4,
        Full = 5,
        NotAcceptingPassengers = 6,
        NoDataAvailable = 7,
        NotBoardable = 8,
    }

    pub enum ScheduleRelationship {
        Scheduled = 0,
        Added = 1,
        Unscheduled = 2,
        Canceled = 3,
        Replacement = 5,
        Duplicated = 6,
        Deleted = 7,
        New = 8,
    }

    pub enum VehicleStopStatus {
        IncomingAt = 0,
        StoppedAt = 1,
        InTransitTo = 2,
    }

    pub enum TripDirection {
        DirectionOne = 0,
        DirectionTwo = 1,
    }

    pub enum WheelchairAccessOverride {
        NoValue = 0,
        Unknown = 1,
        WheelchairAccessible = 2,
        WheelchairInaccessible = 3,
    }

    pub enum WheelchairBoarding {
        Unknown = 0,
        Available = 1,
        NotAvailable = 2,
    }

    pub enum CongestionLevel {
        Unknown = 0,
        RunningSmoothly = 1,
        StopAndGo = 2,
        Congestion = 3,
        SevereCongestion = 4,
    }

    pub enum Cause {
        UnknownCause = 1,
        OtherCause = 2,
        TechnicalProblem = 3,
        Strike = 4,
        Demonstration = 5,
        Accident = 6,
        Holiday = 7,
        Weather = 8,
        Maintenance = 9,
        Construction = 10,
        PoliceActivity = 11,
        MedicalEmergency = 12,
    }

    pub enum Effect {
        NoService = 1,
        ReducedService = 2,
        SignificantDelays = 3,
        Detour = 4,
        AdditionalService = 5,
        ModifiedService = 6,
        OtherEffect = 7,
        UnknownEffect = 8,
        StopMoved = 9,
        NoEffect = 10,
        AccessibilityIssue = 11,
    }

    pub enum SeverityLevel {
        Unknown = 1,
        Info = 2,
        Warning = 3,
        Severe = 4,
    }
);

derives!(
    pub struct FeedMessage {
        header: FeedHeader,
        entities: Vec<FeedEntity>,
    }

    pub struct FeedHeader {
        pub gtfs_realtime_version: String,
        pub incrementality: Option<Incrementality>,

        #[cfg(not(feature = "chrono"))]
        pub timestamp: Option<u64>,
        #[cfg(feature = "chrono")]
        pub timestamp: Option<NaiveDateTime>,

        pub feed_version: Option<String>,

        #[cfg(feature = "realtime_mta")]
        pub nyct_feed_header: Option<NyctFeedHeader>,
    }

    pub struct FeedEntity {
        pub id: String,
        pub is_deleted: bool,
        pub trip_update: Option<TripUpdate>,
        pub vehicle: Option<VehiclePosition>,
        pub alert: Option<Alert>,
        pub shape: Option<Shape>,
        pub stop: Option<Stop>,
        pub trip_modifications: Option<TripModifications>,
    }

    pub struct StopTimeEvent {
        pub delay: Option<i32>,

        #[cfg(not(feature = "chrono"))]
        pub time: Option<i64>,
        #[cfg(feature = "chrono")]
        pub time: Option<NaiveDateTime>,

        pub uncertainty: Option<i32>,

        #[cfg(not(feature = "chrono"))]
        pub scheduled_time: Option<i64>,
        #[cfg(feature = "chrono")]
        pub scheduled_time: Option<NaiveDateTime>,
    }

    pub struct StopTimeProperties {
        pub assigned_stop_id: Option<String>,
        pub stop_headsign: Option<String>,
        pub pickup_type: Option<DropoffPickupType>,
        pub dropoff_type: Option<DropoffPickupType>,
    }

    pub struct StopTimeUpdate {
        pub stop_sequence: Option<u32>,
        pub stop_id: Option<String>,
        pub arrival: Option<StopTimeEvent>,
        pub departure: Option<StopTimeEvent>,
        pub departure_occupancy_status: Option<OccupancyStatus>,
        pub schedule_relationship: ScheduleRelationship,
        pub stop_time_properties: Option<StopTimeProperties>,

        #[cfg(feature = "realtime_mta")]
        pub nyct_stop_time_update: Option<NyctStopTimeUpdate>,
    }

    pub struct TripProperties {
        pub trip_id: Option<String>,

        #[cfg(not(feature = "chrono"))]
        pub start_date: Option<String>,
        #[cfg(feature = "chrono")]
        pub start_date: Option<NaiveDate>,

        #[cfg(not(feature = "chrono"))]
        pub start_time: Option<String>,
        #[cfg(feature = "chrono")]
        pub start_time: Option<NaiveTime>,

        pub shape_id: Option<String>,
        pub trip_headsign: Option<String>,
        pub trip_short_name: Option<String>,
    }

    pub struct TripUpdate {
        pub trip: TripDescriptor,
        pub vehicle: Option<VehicleDescriptor>,
        pub stop_time_updates: Vec<StopTimeUpdate>,

        #[cfg(not(feature = "chrono"))]
        pub timestamp: Option<u64>,
        #[cfg(feature = "chrono")]
        pub timestamp: Option<NaiveDateTime>,

        pub delay: Option<i32>,
        pub trip_properties: Option<TripProperties>,
    }

    pub struct CarriageDetails {
        pub id: Option<String>,
        pub label: Option<String>,
        pub occupancy_status: OccupancyStatus,
        pub occupancy_percentage: i32,
        pub carriage_sequence: Option<i32>,
    }

    pub struct VehiclePosition {
        pub trip: Option<TripDescriptor>,
        pub vehicle: Option<VehicleDescriptor>,
        pub position: Option<Position>,
        pub current_stop_sequence: Option<u32>,
        pub stop_id: Option<String>,
        pub current_status: VehicleStopStatus,

        #[cfg(not(feature = "chrono"))]
        pub timestamp: Option<u64>,
        #[cfg(feature = "chrono")]
        pub timestamp: Option<NaiveDateTime>,

        pub congestion_level: Option<CongestionLevel>,
        pub occupancy_status: Option<OccupancyStatus>,
        pub occupancy_percentage: Option<u32>,
        pub multi_carriage_details: Vec<CarriageDetails>,
    }

    pub struct Alert {
        pub active_periods: Vec<TimeRange>,
        pub informed_entities: Vec<EntitySelector>,
        pub cause: Cause,
        pub effect: Effect,
        pub url: Option<TranslatedString>,
        pub header_text: Option<TranslatedString>,
        pub description_text: Option<TranslatedString>,
        pub tts_header_text: Option<TranslatedString>,
        pub tts_description_text: Option<TranslatedString>,
        pub severity_level: SeverityLevel,
        pub image: Option<TranslatedImage>,
        pub image_alertnative_text: Option<TranslatedString>,
        pub cause_detail: Option<TranslatedString>,
        pub effect_detail: Option<TranslatedString>,
    }

    pub struct TimeRange {
        #[cfg(not(feature = "chrono"))]
        pub start: Option<u64>,
        #[cfg(feature = "chrono")]
        pub start: Option<NaiveDateTime>,

        #[cfg(not(feature = "chrono"))]
        pub end: Option<u64>,
        #[cfg(feature = "chrono")]
        pub end: Option<NaiveDateTime>,
    }

    pub struct Position {
        pub latitude: f64,
        pub longitude: f64,
        pub bearing: Option<f64>,
        pub odometer: Option<f64>,
        pub speed: Option<f64>,
    }

    pub struct ModifiedTripSelector {
        pub modifications_id: Option<String>,
        pub affected_trip_id: Option<String>,

        #[cfg(not(feature = "chrono"))]
        pub start_time: Option<String>,
        #[cfg(feature = "chrono")]
        pub start_time: Option<NaiveTime>,

        #[cfg(not(feature = "chrono"))]
        pub start_date: Option<String>,
        #[cfg(feature = "chrono")]
        pub start_date: Option<NaiveDate>,
    }

    pub struct TripDescriptor {
        pub trip_id: Option<String>,
        pub route_id: Option<String>,
        pub direction_id: Option<TripDirection>,

        #[cfg(not(feature = "chrono"))]
        pub start_time: Option<String>,
        #[cfg(feature = "chrono")]
        pub start_time: Option<NaiveTime>,

        #[cfg(not(feature = "chrono"))]
        pub start_date: Option<String>,
        #[cfg(feature = "chrono")]
        pub start_date: Option<NaiveDate>,

        pub schedule_relationship: Option<ScheduleRelationship>,
        pub modified_trip: Option<ModifiedTripSelector>,

        #[cfg(feature = "realtime_mta")]
        pub nyct_trip_descriptor: Option<NyctTripDescriptor>,
    }

    pub struct VehicleDescriptor {
        pub id: Option<String>,
        pub label: Option<String>,
        pub license_plate: Option<String>,
        pub wheelchair_accessible: Option<WheelchairAccessOverride>,
    }

    pub struct EntitySelector {
        pub agency_id: Option<String>,
        pub route_id: Option<String>,
        pub route_type: Option<i32>,
        pub trip: Option<TripDescriptor>,
        pub stop_id: Option<String>,
        pub direction_id: Option<TripDirection>,
    }

    pub struct Translation {
        pub text: String,
        pub language: Option<String>,
    }

    pub struct TranslatedString {
        pub translations: Vec<Translation>,
    }

    pub struct LocalizedImage {
        pub url: String,
        pub media_type: String,
        pub language: Option<String>,
    }

    pub struct TranslatedImage {
        pub localized_images: Vec<LocalizedImage>,
    }

    pub struct Shape {
        pub shape_id: Option<String>,
        pub encoded_polyline: Option<String>,
    }

    pub struct Stop {
        pub stop_id: Option<String>,
        pub stop_code: Option<TranslatedString>,
        pub stop_name: Option<TranslatedString>,
        pub tts_stop_name: Option<TranslatedString>,
        pub stop_desc: Option<TranslatedString>,
        pub stop_lat: Option<f64>,
        pub stop_lon: Option<f64>,
        pub zone_id: Option<String>,
        pub stop_url: Option<TranslatedString>,
        pub parent_station: Option<String>,

        #[cfg(not(feature = "chrono_tz"))]
        pub stop_timezone: String,
        #[cfg(feature = "chrono_tz")]
        pub stop_timezone: Tz,

        pub wheelchair_boarding: WheelchairBoarding,
        pub level_id: Option<String>,
        pub platform_code: Option<TranslatedString>,
    }

    pub struct Modification {
        pub start_stop_selector: Option<StopSelector>,
        pub end_stop_selector: Option<StopSelector>,
        pub propagated_modification_delay: i32,
        pub replacement_stops: Vec<ReplacementStop>,
        pub service_alert_id: Option<String>,

        #[cfg(not(feature = "chrono"))]
        pub last_modified_time: Option<u64>,
        #[cfg(feature = "chrono")]
        pub last_modified_time: Option<NaiveDateTime>,
    }

    pub struct SelectedTrips {
        pub trip_ids: Vec<String>,
        pub shape_id: Option<String>,
    }

    pub struct TripModifications {
        pub selected_trips: Vec<SelectedTrips>,

        #[cfg(not(feature = "chrono"))]
        pub start_times: Vec<String>,
        #[cfg(feature = "chrono")]
        pub start_times: Vec<NaiveTime>,

        #[cfg(not(feature = "chrono"))]
        pub service_dates: Vec<String>,
        #[cfg(feature = "chrono")]
        pub service_dates: Vec<NaiveDate>,

        pub modifications: Vec<Modification>,
    }

    pub struct StopSelector {
        pub stop_sequence: Option<u32>,
        pub stop_id: Option<String>,
    }

    pub struct ReplacementStop {
        pub travel_time_to_stop: Option<i32>,
        pub stop_id: Option<String>,
    }
);

impl From<crate::realtime::parse::protos::gtfs::ReplacementStop> for ReplacementStop {
    fn from(value: crate::realtime::parse::protos::gtfs::ReplacementStop) -> Self {
        let crate::realtime::parse::protos::gtfs::ReplacementStop {
            travel_time_to_stop,
            stop_id,
            ..
        } = value;

        Self {
            travel_time_to_stop,
            stop_id,
        }
    }
}
