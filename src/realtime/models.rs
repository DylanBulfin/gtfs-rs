#![allow(non_camel_case_types)]

use std::str::FromStr;

#[cfg(feature = "chrono")]
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
#[cfg(feature = "chrono_tz")]
use chrono_tz::Tz;
use gtfs_macros::{gtfs_realtime_enum, gtfs_realtime_model};
use protobuf::MessageField;

#[cfg(feature = "realtime_mta")]
use crate::realtime::models_mta::{NyctFeedHeader, NyctStopTimeUpdate, NyctTripDescriptor};

#[gtfs_realtime_enum(crate::realtime::parse::protos::gtfs::feed_header::Incrementality)]
pub enum Incrementality {
    FULL_DATASET = 0,
    DIFFERENTIAL = 1,
}

#[gtfs_realtime_enum(crate::realtime::parse::protos::gtfs::trip_update::stop_time_update::stop_time_properties::DropOffPickupType)]
pub enum DropoffPickupType {
    REGULAR = 0,
    NONE = 1,
    PHONE_AGENCY = 2,
    COORDINATE_WITH_DRIVER = 3,
}

#[gtfs_realtime_enum(crate::realtime::parse::protos::gtfs::vehicle_position::OccupancyStatus)]
pub enum OccupancyStatus {
    EMPTY = 0,
    MANY_SEATS_AVAILABLE = 1,
    FEW_SEATS_AVAILABLE = 2,
    STANDING_ROOM_ONLY = 3,
    CRUSHED_STANDING_ROOM_ONLY = 4,
    FULL = 5,
    NOT_ACCEPTING_PASSENGERS = 6,
    NO_DATA_AVAILABLE = 7,
    NOT_BOARDABLE = 8,
}

#[gtfs_realtime_enum(
    crate::realtime::parse::protos::gtfs::trip_update::stop_time_update::ScheduleRelationship
)]
pub enum StopScheduleRelationship {
    SCHEDULED = 0,
    SKIPPED = 1,
    NO_DATA = 2,
    UNSCHEDULED = 3,
}

#[gtfs_realtime_enum(crate::realtime::parse::protos::gtfs::trip_descriptor::ScheduleRelationship)]
pub enum TripScheduleRelationship {
    SCHEDULED = 0,
    ADDED = 1,
    UNSCHEDULED = 2,
    CANCELED = 3,
    REPLACEMENT = 5,
    DUPLICATED = 6,
    DELETED = 7,
    NEW = 8,
}

#[gtfs_realtime_enum(crate::realtime::parse::protos::gtfs::vehicle_position::VehicleStopStatus)]
pub enum VehicleStopStatus {
    INCOMING_AT = 0,
    STOPPED_AT = 1,
    IN_TRANSIT_TO = 2,
}

#[gtfs_realtime_enum(
    crate::realtime::parse::protos::gtfs::vehicle_descriptor::WheelchairAccessible
)]
pub enum WheelchairAccessOverride {
    NO_VALUE = 0,
    UNKNOWN = 1,
    WHEELCHAIR_ACCESSIBLE = 2,
    WHEELCHAIR_INACCESSIBLE = 3,
}

#[gtfs_realtime_enum(crate::realtime::parse::protos::gtfs::stop::WheelchairBoarding)]
pub enum WheelchairBoarding {
    UNKNOWN = 0,
    AVAILABLE = 1,
    NOT_AVAILABLE = 2,
}

#[gtfs_realtime_enum(crate::realtime::parse::protos::gtfs::vehicle_position::CongestionLevel)]
pub enum CongestionLevel {
    UNKNOWN_CONGESTION_LEVEL = 0,
    RUNNING_SMOOTHLY = 1,
    STOP_AND_GO = 2,
    CONGESTION = 3,
    SEVERE_CONGESTION = 4,
}

#[gtfs_realtime_enum(crate::realtime::parse::protos::gtfs::alert::Cause)]
pub enum Cause {
    UNKNOWN_CAUSE = 1,
    OTHER_CAUSE = 2,
    TECHNICAL_PROBLEM = 3,
    STRIKE = 4,
    DEMONSTRATION = 5,
    ACCIDENT = 6,
    HOLIDAY = 7,
    WEATHER = 8,
    MAINTENANCE = 9,
    CONSTRUCTION = 10,
    POLICE_ACTIVITY = 11,
    MEDICAL_EMERGENCY = 12,
}

#[gtfs_realtime_enum(crate::realtime::parse::protos::gtfs::alert::Effect)]
pub enum Effect {
    NO_SERVICE = 1,
    REDUCED_SERVICE = 2,
    SIGNIFICANT_DELAYS = 3,
    DETOUR = 4,
    ADDITIONAL_SERVICE = 5,
    MODIFIED_SERVICE = 6,
    OTHER_EFFECT = 7,
    UNKNOWN_EFFECT = 8,
    STOP_MOVED = 9,
    NO_EFFECT = 10,
    ACCESSIBILITY_ISSUE = 11,
}

#[gtfs_realtime_enum(crate::realtime::parse::protos::gtfs::alert::SeverityLevel)]
pub enum SeverityLevel {
    UNKNOWN_SEVERITY = 1,
    INFO = 2,
    WARNING = 3,
    SEVERE = 4,
}

fn parse_dt_u64(
    value: Option<u64>,
) -> Result<Option<chrono::DateTime<chrono::Utc>>, crate::error::Error> {
    match value {
        Some(n) => Ok(chrono::DateTime::<chrono::Utc>::from_timestamp(
            n.try_into()
                .map_err(|_| String::from("Unable to convert timestamp to i64"))?,
            0,
        )),
        None => Ok(None),
    }
}

fn parse_dt_i64(
    value: Option<i64>,
) -> Result<Option<chrono::DateTime<chrono::Utc>>, crate::error::Error> {
    match value {
        Some(n) => Ok(chrono::DateTime::<chrono::Utc>::from_timestamp(n, 0)),
        None => Ok(None),
    }
}

fn parse_nt(value: Option<String>) -> Result<Option<chrono::NaiveTime>, crate::error::Error> {
    match value {
        Some(s) => {
            let hour: u32 = s[0..2]
                .parse()
                .map_err(|_| String::from("Unable to parse hour"))?;
            let min: u32 = s[3..5]
                .parse()
                .map_err(|_| String::from("Unable to parse minute"))?;
            let sec: u32 = s[6..8]
                .parse()
                .map_err(|_| String::from("Unable to parse second"))?;

            Ok(NaiveTime::from_hms_opt(hour, min, sec))
        }
        None => Ok(None),
    }
}

fn parse_nd(value: Option<String>) -> Result<Option<chrono::NaiveDate>, crate::error::Error> {
    match value {
        Some(s) => {
            let year: i32 = s[0..4]
                .parse()
                .map_err(|_| String::from("Unable to parse year"))?;
            let month: u32 = s[4..6]
                .parse()
                .map_err(|_| String::from("Unable to parse month"))?;
            let day: u32 = s[6..8]
                .parse()
                .map_err(|_| String::from("Unable to parse day"))?;

            Ok(NaiveDate::from_ymd_opt(year, month, day))
        }
        None => Ok(None),
    }
}

fn parse_nds(value: Vec<String>) -> Result<Vec<chrono::NaiveDate>, crate::error::Error> {
    let mut res = Vec::new();

    for s in value {
        let year: i32 = s[0..4]
            .parse()
            .map_err(|_| String::from("Unable to parse year"))?;
        let month: u32 = s[4..6]
            .parse()
            .map_err(|_| String::from("Unable to parse month"))?;
        let day: u32 = s[6..8]
            .parse()
            .map_err(|_| String::from("Unable to parse day"))?;

        res.push(
            NaiveDate::from_ymd_opt(year, month, day)
                .ok_or(String::from("Unable to parse NaiveDate in list"))?,
        );
    }

    Ok(res)
}

fn parse_nts(value: Vec<String>) -> Result<Vec<chrono::NaiveTime>, crate::error::Error> {
    let mut res = Vec::new();

    for s in value {
        let hour: u32 = s[0..2]
            .parse()
            .map_err(|_| String::from("Unable to parse hour"))?;
        let min: u32 = s[3..5]
            .parse()
            .map_err(|_| String::from("Unable to parse min"))?;
        let sec: u32 = s[6..8]
            .parse()
            .map_err(|_| String::from("Unable to parse sec"))?;

        res.push(
            NaiveTime::from_hms_opt(hour, min, sec)
                .ok_or(String::from("Unable to parse NaiveTime in list"))?,
        );
    }

    Ok(res)
}

fn parse_tz(value: Option<String>) -> Result<Option<chrono_tz::Tz>, crate::error::Error> {
    match value {
        Some(s) => Ok(Some(
            chrono_tz::Tz::from_str(&s).map_err(|_| String::from("Unable to parse timezone"))?,
        )),
        None => Ok(None),
    }
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::FeedMessage)]
pub struct FeedMessage {
    #[gtfs(mfreq)]
    header: FeedHeader,
    #[gtfs(vec)]
    entity: Vec<FeedEntity>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::FeedHeader)]
pub struct FeedHeader {
    #[gtfs(required)]
    pub gtfs_realtime_version: String,
    #[gtfs(Enum)]
    pub incrementality: Option<Incrementality>,
    #[gtfs("chrono", Option<chrono::DateTime<chrono::Utc>>, parse_dt_u64)]
    pub timestamp: Option<u64>,
    pub feed_version: Option<String>,
    #[gtfs(mf, "realtime_mta")]
    pub nyct_feed_header: Option<NyctFeedHeader>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::FeedEntity)]
pub struct FeedEntity {
    #[gtfs(required)]
    pub id: String,
    #[gtfs(required)]
    pub is_deleted: bool,
    #[gtfs(mf)]
    pub trip_update: Option<TripUpdate>,
    #[gtfs(mf)]
    pub vehicle: Option<VehiclePosition>,
    #[gtfs(mf)]
    pub alert: Option<Alert>,
    #[gtfs(mf)]
    pub shape: Option<Shape>,
    #[gtfs(mf)]
    pub stop: Option<Stop>,
    #[gtfs(mf)]
    pub trip_modifications: Option<TripModifications>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::trip_update::StopTimeEvent)]
pub struct StopTimeEvent {
    pub delay: Option<i32>,
    #[gtfs("chrono", Option<chrono::DateTime<chrono::Utc>>, parse_dt_i64)]
    pub time: Option<i64>,
    pub uncertainty: Option<i32>,
    #[gtfs("chrono", Option<chrono::DateTime<chrono::Utc>>, parse_dt_i64)]
    pub scheduled_time: Option<i64>,
}

#[gtfs_realtime_model(
    crate::realtime::parse::protos::gtfs::trip_update::stop_time_update::StopTimeProperties
)]
pub struct StopTimeProperties {
    pub assigned_stop_id: Option<String>,
    pub stop_headsign: Option<String>,
    #[gtfs(mf)]
    pub pickup_type: Option<DropoffPickupType>,
    #[gtfs(mf)]
    pub drop_off_type: Option<DropoffPickupType>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::trip_update::StopTimeUpdate)]
pub struct StopTimeUpdate {
    pub stop_sequence: Option<u32>,
    pub stop_id: Option<String>,
    #[gtfs(mf)]
    pub arrival: Option<StopTimeEvent>,
    #[gtfs(mf)]
    pub departure: Option<StopTimeEvent>,
    #[gtfs(Enum)]
    pub departure_occupancy_status: Option<OccupancyStatus>,
    #[gtfs(mfreq)]
    pub schedule_relationship: StopScheduleRelationship,
    #[gtfs(mf)]
    pub stop_time_properties: Option<StopTimeProperties>,
    #[gtfs(mf, "realtime_mta")]
    pub nyct_stop_time_update: Option<NyctStopTimeUpdate>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::trip_update::TripProperties)]
pub struct TripProperties {
    pub trip_id: Option<String>,
    #[gtfs("chrono", Option<chrono::NaiveDate>, parse_nd)]
    pub start_date: Option<String>,
    #[gtfs("chrono", Option<chrono::NaiveTime>, parse_nt)]
    pub start_time: Option<String>,
    pub shape_id: Option<String>,
    pub trip_headsign: Option<String>,
    pub trip_short_name: Option<String>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::TripUpdate)]
pub struct TripUpdate {
    #[gtfs(mfreq)]
    pub trip: TripDescriptor,
    #[gtfs(mf)]
    pub vehicle: Option<VehicleDescriptor>,
    #[gtfs(vec)]
    pub stop_time_update: Vec<StopTimeUpdate>,
    #[gtfs("chrono", Option<chrono::DateTime<chrono::Utc>>, parse_dt_u64)]
    pub timestamp: Option<u64>,
    pub delay: Option<i32>,
    #[gtfs(mf)]
    pub trip_properties: Option<TripProperties>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::vehicle_position::CarriageDetails)]
pub struct CarriageDetails {
    pub id: Option<String>,
    pub label: Option<String>,
    #[gtfs(enumreq)]
    pub occupancy_status: OccupancyStatus,
    #[gtfs(required)]
    pub occupancy_percentage: i32,
    pub carriage_sequence: Option<u32>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::VehiclePosition)]
pub struct VehiclePosition {
    #[gtfs(mf)]
    pub trip: Option<TripDescriptor>,
    #[gtfs(mf)]
    pub vehicle: Option<VehicleDescriptor>,
    #[gtfs(mf)]
    pub position: Option<Position>,
    pub current_stop_sequence: Option<u32>,
    pub stop_id: Option<String>,
    #[gtfs(enumreq)]
    pub current_status: VehicleStopStatus,
    #[gtfs("chrono", Option<chrono::DateTime<chrono::Utc>>, parse_dt_u64)]
    pub timestamp: Option<u64>,
    #[gtfs(mf)]
    pub congestion_level: Option<CongestionLevel>,
    #[gtfs(Enum)]
    pub occupancy_status: Option<OccupancyStatus>,
    pub occupancy_percentage: Option<u32>,
    #[gtfs(vec)]
    pub multi_carriage_details: Vec<CarriageDetails>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::Alert)]
pub struct Alert {
    #[gtfs(vec)]
    pub active_period: Vec<TimeRange>,
    #[gtfs(vec)]
    pub informed_entity: Vec<EntitySelector>,
    #[gtfs(enumreq)]
    pub cause: Cause,
    #[gtfs(enumreq)]
    pub effect: Effect,
    #[gtfs(mf)]
    pub url: Option<TranslatedString>,
    #[gtfs(mf)]
    pub header_text: Option<TranslatedString>,
    #[gtfs(mf)]
    pub description_text: Option<TranslatedString>,
    #[gtfs(mf)]
    pub tts_header_text: Option<TranslatedString>,
    #[gtfs(mf)]
    pub tts_description_text: Option<TranslatedString>,
    #[gtfs(enumreq)]
    pub severity_level: SeverityLevel,
    #[gtfs(mf)]
    pub image: Option<TranslatedImage>,
    #[gtfs(mf)]
    pub image_alternative_text: Option<TranslatedString>,
    #[gtfs(mf)]
    pub cause_detail: Option<TranslatedString>,
    #[gtfs(mf)]
    pub effect_detail: Option<TranslatedString>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::TimeRange)]
pub struct TimeRange {
    #[gtfs("chrono", Option<chrono::DateTime<chrono::Utc>>, parse_dt_u64)]
    pub start: Option<u64>,
    #[gtfs("chrono", Option<chrono::DateTime<chrono::Utc>>, parse_dt_u64)]
    pub end: Option<u64>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::Position)]
pub struct Position {
    #[gtfs(required)]
    pub latitude: f32,
    #[gtfs(required)]
    pub longitude: f32,
    pub bearing: Option<f32>,
    pub odometer: Option<f64>,
    pub speed: Option<f32>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::trip_descriptor::ModifiedTripSelector)]
pub struct ModifiedTripSelector {
    pub modifications_id: Option<String>,
    pub affected_trip_id: Option<String>,
    #[gtfs("chrono", Option<chrono::NaiveTime>, parse_nt)]
    pub start_time: Option<String>,
    #[gtfs("chrono", Option<chrono::NaiveDate>, parse_nd)]
    pub start_date: Option<String>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::TripDescriptor)]
pub struct TripDescriptor {
    pub trip_id: Option<String>,
    pub route_id: Option<String>,
    pub direction_id: Option<u32>,
    #[gtfs("chrono", Option<chrono::NaiveTime>, parse_nt)]
    pub start_time: Option<String>,
    #[gtfs("chrono", Option<chrono::NaiveDate>, parse_nd)]
    pub start_date: Option<String>,
    #[gtfs(mf)]
    pub schedule_relationship: Option<TripScheduleRelationship>,
    #[gtfs(mf)]
    pub modified_trip: Option<ModifiedTripSelector>,
    #[gtfs(mf, "realtime_mta")]
    pub nyct_trip_descriptor: Option<NyctTripDescriptor>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::VehicleDescriptor)]
pub struct VehicleDescriptor {
    pub id: Option<String>,
    pub label: Option<String>,
    pub license_plate: Option<String>,
    #[gtfs(Enum)]
    pub wheelchair_accessible: Option<WheelchairAccessOverride>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::EntitySelector)]
pub struct EntitySelector {
    pub agency_id: Option<String>,
    pub route_id: Option<String>,
    pub route_type: Option<i32>,
    #[gtfs(mf)]
    pub trip: Option<TripDescriptor>,
    pub stop_id: Option<String>,
    pub direction_id: Option<u32>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::translated_string::Translation)]
pub struct Translation {
    #[gtfs(required)]
    pub text: String,
    pub language: Option<String>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::TranslatedString)]
pub struct TranslatedString {
    #[gtfs(vec)]
    pub translation: Vec<Translation>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::translated_image::LocalizedImage)]
pub struct LocalizedImage {
    #[gtfs(required)]
    pub url: String,
    #[gtfs(required)]
    pub media_type: String,
    pub language: Option<String>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::TranslatedImage)]
pub struct TranslatedImage {
    #[gtfs(vec)]
    pub localized_image: Vec<LocalizedImage>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::Shape)]
pub struct Shape {
    pub shape_id: Option<String>,
    pub encoded_polyline: Option<String>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::Stop)]
pub struct Stop {
    pub stop_id: Option<String>,
    #[gtfs(mf)]
    pub stop_code: Option<TranslatedString>,
    #[gtfs(mf)]
    pub stop_name: Option<TranslatedString>,
    #[gtfs(mf)]
    pub tts_stop_name: Option<TranslatedString>,
    #[gtfs(mf)]
    pub stop_desc: Option<TranslatedString>,
    pub stop_lat: Option<f32>,
    pub stop_lon: Option<f32>,
    pub zone_id: Option<String>,
    #[gtfs(mf)]
    pub stop_url: Option<TranslatedString>,
    pub parent_station: Option<String>,
    #[gtfs(required, "chrono_tz", chrono_tz::Tz, parse_tz)]
    pub stop_timezone: String,
    #[gtfs(enumreq)]
    pub wheelchair_boarding: WheelchairBoarding,
    pub level_id: Option<String>,
    #[gtfs(mf)]
    pub platform_code: Option<TranslatedString>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::trip_modifications::Modification)]
pub struct Modification {
    #[gtfs(mf)]
    pub start_stop_selector: Option<StopSelector>,
    #[gtfs(mf)]
    pub end_stop_selector: Option<StopSelector>,
    #[gtfs(required)]
    pub propagated_modification_delay: i32,
    #[gtfs(vec)]
    pub replacement_stops: Vec<ReplacementStop>,
    pub service_alert_id: Option<String>,
    #[gtfs("chrono", Option<chrono::DateTime<chrono::Utc>>, parse_dt_u64)]
    pub last_modified_time: Option<u64>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::trip_modifications::SelectedTrips)]
pub struct SelectedTrips {
    #[gtfs(vec)]
    pub trip_ids: Vec<String>,
    pub shape_id: Option<String>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::TripModifications)]
pub struct TripModifications {
    #[gtfs(vec)]
    pub selected_trips: Vec<SelectedTrips>,
    #[gtfs("chrono", Vec<chrono::NaiveTime>, parse_nts)]
    pub start_times: Vec<String>,
    #[gtfs("chrono", Vec<chrono::NaiveDate>, parse_nds)]
    pub service_dates: Vec<String>,
    #[gtfs(vec)]
    pub modifications: Vec<Modification>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::StopSelector)]
pub struct StopSelector {
    pub stop_sequence: Option<u32>,
    pub stop_id: Option<String>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::ReplacementStop)]
pub struct ReplacementStop {
    pub travel_time_to_stop: Option<i32>,
    pub stop_id: Option<String>,
}
