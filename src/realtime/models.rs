#[cfg(feature = "chrono")]
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
#[cfg(feature = "chrono_tz")]
use chrono_tz::Tz;

use crate::realtime;

#[cfg(feature = "realtime_parse")]
use {
    serde::{Serialize, Deserialize},
    serde_repr::{Serialize_repr, Deserialize_repr}
};

pub struct FeedMessage {
    header: FeedHeader,
    entities: Vec<FeedEntity>,
}

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
        Differential = 1
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
        CrushedStandingRoomOnly =4,
        Full = 5,
        NotAcceptingPassengers = 6,
        NoDataAvailable = 7,
        NotBoardable = 8
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
        InTransitTo = 2
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
        SevereCongestion = 4
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
        MedicalEmergency = 12
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
        AccessibilityIssue = 11
    }

    pub enum SeverityLevel {
        Unknown = 1,
        Info = 2,
        Warning = 3,
        Severe = 4
    }
);

derives!(
    pub struct FeedHeader {
        gtfs_realtime_version: String,
        incrementality: Option<Incrementality>,

        #[cfg(not(feature = "chrono"))]
        timestamp: Option<u64>,
        #[cfg(feature = "chrono")]
        timestamp: Option<NaiveDateTime>,

        feed_version: Option<String>,
    }

    pub struct FeedEntity {
        id: String,
        is_deleted: bool,
        trip_update: Option<TripUpdate>,
        vehicle: Option<VehiclePosition>,
        alert: Option<Alert>,
        shape: Option<Shape>,
        stop: Option<Stop>,
        trip_modifications: Option<TripModifications>,
    }

    pub struct StopTimeEvent {
        delay: Option<i32>,

        #[cfg(not(feature = "chrono"))]
        time: Option<i64>,
        #[cfg(feature = "chrono")]
        time: Option<NaiveDateTime>,

        uncertainty: Option<i32>,

        #[cfg(not(feature = "chrono"))]
        scheduled_time: Option<i64>,
        #[cfg(feature = "chrono")]
        scheduled_time: Option<NaiveDateTime>,
    }

    pub struct StopTimeProperties {
        assigned_stop_id: Option<String>,
        stop_headsign: Option<String>,
        pickup_type: Option<DropoffPickupType>,
        dropoff_type: Option<DropoffPickupType>,
    }

    pub struct StopTimeUpdate {
        stop_sequence: Option<u32>,
        stop_id: Option<String>,
        arrival: Option<StopTimeEvent>,
        departure: Option<StopTimeEvent>,
        departure_occupancy_status: Option<OccupancyStatus>,
        schedule_relationship: ScheduleRelationship,
        stop_time_properties: Option<StopTimeProperties>,
    }

    pub struct TripProperties {
        trip_id: Option<String>,

        #[cfg(not(feature = "chrono"))]
        start_date: Option<String>,
        #[cfg(feature = "chrono")]
        start_date: Option<NaiveDate>,

        #[cfg(not(feature = "chrono"))]
        start_time: Option<String>,
        #[cfg(feature = "chrono")]
        start_time: Option<NaiveTime>,

        shape_id: Option<String>,
        trip_headsign: Option<String>,
        trip_short_name: Option<String>,
    }

    pub struct TripUpdate {
        trip: TripDescriptor,
        vehicle: Option<VehicleDescriptor>,
        stop_time_updates: Vec<StopTimeUpdate>,

        #[cfg(not(feature = "chrono"))]
        timestamp: Option<u64>,
        #[cfg(feature = "chrono")]
        timestamp: Option<NaiveDateTime>,

        delay: Option<i32>,
        trip_properties: Option<TripProperties>,
    }

    pub struct CarriageDetails {
        id: Option<String>,
        label: Option<String>,
        occupancy_status: OccupancyStatus,
        occupancy_percentage: i32,
        carriage_sequence: Option<i32>,
    }

    pub struct VehiclePosition {
        trip: Option<TripDescriptor>,
        vehicle: Option<VehicleDescriptor>,
        position: Option<Position>,
        current_stop_sequence: Option<u32>,
        stop_id: Option<String>,
        current_status: VehicleStopStatus,

        #[cfg(not(feature = "chrono"))]
        timestamp: Option<u64>,
        #[cfg(feature = "chrono")]
        timestamp: Option<NaiveDateTime>,

        congestion_level: Option<CongestionLevel>,
        occupancy_status: Option<OccupancyStatus>,
        occupancy_percentage: Option<u32>,
        multi_carriage_details: Vec<CarriageDetails>,
    }

    pub struct Alert {
        active_periods: Vec<TimeRange>,
        informed_entities: Vec<EntitySelector>,
        cause: Cause,
        effect: Effect,
        url: Option<TranslatedString>,
        header_text: Option<TranslatedString>,
        description_text: Option<TranslatedString>,
        tts_header_text: Option<TranslatedString>,
        tts_description_text: Option<TranslatedString>,
        severity_level: SeverityLevel,
        image: Option<TranslatedImage>,
        image_alertnative_text: Option<TranslatedString>,
        cause_detail: Option<TranslatedString>,
        effect_detail: Option<TranslatedString>,
    }

    pub struct TimeRange {
        #[cfg(not(feature = "chrono"))]
        start: Option<u64>,
        #[cfg(feature = "chrono")]
        start: Option<NaiveDateTime>,

        #[cfg(not(feature = "chrono"))]
        end: Option<u64>,
        #[cfg(feature = "chrono")]
        end: Option<NaiveDateTime>,
    }

    pub struct Position {
        latitude: f64,
        longitude: f64,
        bearing: Option<f64>,
        odometer: Option<f64>,
        speed: Option<f64>,
    }

    pub struct ModifiedTripSelector {
        modifications_id: Option<String>,
        affected_trip_id: Option<String>,

        #[cfg(not(feature = "chrono"))]
        start_time: Option<String>,
        #[cfg(feature = "chrono")]
        start_time: Option<NaiveTime>,

        #[cfg(not(feature = "chrono"))]
        start_date: Option<String>,
        #[cfg(feature = "chrono")]
        start_date: Option<NaiveDate>,
    }

    pub struct TripDescriptor {
        trip_id: Option<String>,
        route_id: Option<String>,
        direction_id: Option<TripDirection>,

        #[cfg(not(feature = "chrono"))]
        start_time: Option<String>,
        #[cfg(feature = "chrono")]
        start_time: Option<NaiveTime>,

        #[cfg(not(feature = "chrono"))]
        start_date: Option<String>,
        #[cfg(feature = "chrono")]
        start_date: Option<NaiveDate>,

        schedule_relationship: Option<ScheduleRelationship>,
        modified_trip: Option<ModifiedTripSelector>,
    }

    pub struct VehicleDescriptor {
        id: Option<String>,
        label: Option<String>,
        license_plate: Option<String>,
        wheelchair_accessible: Option<WheelchairAccessOverride>,
    }

    pub struct EntitySelector {
        agency_id: Option<String>,
        route_id: Option<String>,
        route_type: Option<i32>,
        trip: Option<TripDescriptor>,
        stop_id: Option<String>,
        direction_id: Option<TripDirection>,
    }

    pub struct Translation {
        text: String,
        language: Option<String>,
    }

    pub struct TranslatedString {
        translations: Vec<Translation>,
    }

    pub struct LocalizedImage {
        url: String,
        media_type: String,
        language: Option<String>,
    }

    pub struct TranslatedImage {
        localized_images: Vec<LocalizedImage>,
    }

    pub struct Shape {
        shape_id: Option<String>,
        encoded_polyline: Option<String>,
    }

    pub struct Stop {
        stop_id: Option<String>,
        stop_code: Option<TranslatedString>,
        stop_name: Option<TranslatedString>,
        tts_stop_name: Option<TranslatedString>,
        stop_desc: Option<TranslatedString>,
        stop_lat: Option<f64>,
        stop_lon: Option<f64>,
        zone_id: Option<String>,
        stop_url: Option<TranslatedString>,
        parent_station: Option<String>,

        #[cfg(not(feature = "chrono_tz"))]
        stop_timezone: String,
        #[cfg(feature = "chrono_tz")]
        stop_timezone: Tz,

        wheelchair_boarding: WheelchairBoarding,
        level_id: Option<String>,
        platform_code: Option<TranslatedString>,
    }

    pub struct Modification {
        start_stop_selector: Option<StopSelector>,
        end_stop_selector: Option<StopSelector>,
        propagated_modification_delay: i32,
        replacement_stops: Vec<ReplacementStop>,
        service_alert_id: Option<String>,

        #[cfg(not(feature = "chrono"))]
        last_modified_time: Option<u64>,
        #[cfg(feature = "chrono")]
        last_modified_time: Option<NaiveDateTime>,
    }

    pub struct SelectedTrips {
        trip_ids: Vec<String>,
        shape_id: Option<String>,
    }

    pub struct TripModifications {
        selected_trips: Vec<SelectedTrips>,

        #[cfg(not(feature = "chrono"))]
        start_times: Vec<String>,
        #[cfg(feature = "chrono")]
        start_times: Vec<NaiveTime>,

        #[cfg(not(feature = "chrono"))]
        service_dates: Vec<String>,
        #[cfg(feature = "chrono")]
        service_dates: Vec<NaiveDate>,

        modifications: Vec<Modification>,
    }

    pub struct StopSelector {
        stop_sequence: Option<u32>,
        stop_id: Option<String>,
    }

    pub struct ReplacementStop {
        travel_time_to_stop: Option<i32>,
        stop_id: Option<String>,
    }
);
