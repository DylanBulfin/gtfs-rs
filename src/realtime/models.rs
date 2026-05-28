#![allow(non_camel_case_types)]

#[cfg(feature = "chrono")]
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
#[cfg(feature = "chrono_tz")]
use chrono_tz::Tz;
use gtfs_macros::{gtfs_realtime_enum, gtfs_realtime_model};
use protobuf::MessageField;

#[cfg(feature = "realtime_parse")]
use {
    serde::{Deserialize, Serialize},
    serde_repr::{Deserialize_repr, Serialize_repr},
};

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

pub fn parse_required<T, U>(input: Option<T>) -> Result<U, crate::error::Error>
where
    U: TryFrom<T>,
    crate::error::Error: From<U::Error>,
{
    match input {
        None => Err(String::from("Required variable does not exist"))?,
        Some(field) => Ok(field
            .try_into()
            .map_err(|_| String::from("Unable to convert required variable to required type"))?),
    }
}

pub fn parse_mf<T, U>(mut input: MessageField<T>) -> Result<Option<U>, crate::error::Error>
where
    U: TryFrom<T>,
    crate::error::Error: From<U::Error>,
{
    match input.take() {
        None => Ok(None),
        Some(field) => Ok(Some(field.try_into().map_err(|_| {
            String::from("Unable to convert message field variable into necessary type")
        })?)),
    }
}

pub fn parse_mf_req<T, U>(mut input: MessageField<T>) -> Result<U, crate::error::Error>
where
    U: TryFrom<T>,
    crate::error::Error: From<U::Error>,
{
    match input.take() {
        None => Err(String::from("Unable to parse required message field").into()),
        Some(field) => Ok(field.try_into()?),
    }
}

pub fn parse_vec<T, U>(input: Vec<T>) -> Result<Vec<U>, crate::error::Error>
where
    U: TryFrom<T>,
    crate::error::Error: From<U::Error>,
{
    let b: Result<Vec<U>, U::Error> = input.into_iter().map(|t| U::try_from(t)).collect();

    Ok(b?)
}

pub struct FeedMessage {
    header: FeedHeader,
    entity: Vec<FeedEntity>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::FeedHeader)]
pub struct FeedHeader {
    #[gtfs_custom(parse_required)]
    pub gtfs_realtime_version: String,
    pub incrementality: Option<Incrementality>,

    pub timestamp: Option<u64>,

    pub feed_version: Option<String>,
    //
    // #[cfg(feature = "realtime_mta")]
    // pub nyct_feed_header: Option<NyctFeedHeader>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::FeedEntity)]
pub struct FeedEntity {
    #[gtfs_custom(parse_required)]
    pub id: String,
    #[gtfs_custom(parse_required)]
    pub is_deleted: bool,
    #[gtfs_custom(parse_mf)]
    pub trip_update: Option<TripUpdate>,
    #[gtfs_custom(parse_mf)]
    pub vehicle: Option<VehiclePosition>,
    #[gtfs_custom(parse_mf)]
    pub alert: Option<Alert>,
    #[gtfs_custom(parse_mf)]
    pub shape: Option<Shape>,
    #[gtfs_custom(parse_mf)]
    pub stop: Option<Stop>,
    #[gtfs_custom(parse_mf)]
    pub trip_modifications: Option<TripModifications>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::trip_update::StopTimeEvent)]
pub struct StopTimeEvent {
    pub delay: Option<i32>,

    pub time: Option<i64>,

    pub uncertainty: Option<i32>,

    pub scheduled_time: Option<i64>,
}

fn test(
    a: crate::realtime::parse::protos::gtfs::trip_update::stop_time_update::StopTimeProperties,
) {
}

#[gtfs_realtime_model(
    crate::realtime::parse::protos::gtfs::trip_update::stop_time_update::StopTimeProperties
)]
pub struct StopTimeProperties {
    pub assigned_stop_id: Option<String>,
    pub stop_headsign: Option<String>,
    pub pickup_type: Option<DropoffPickupType>,
    pub drop_off_type: Option<DropoffPickupType>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::trip_update::StopTimeUpdate)]
pub struct StopTimeUpdate {
    pub stop_sequence: Option<u32>,
    pub stop_id: Option<String>,
    #[gtfs_custom(parse_mf)]
    pub arrival: Option<StopTimeEvent>,
    #[gtfs_custom(parse_mf)]
    pub departure: Option<StopTimeEvent>,
    pub departure_occupancy_status: Option<OccupancyStatus>,
    #[gtfs_custom(parse_required)]
    pub schedule_relationship: StopScheduleRelationship,
    #[gtfs_custom(parse_mf)]
    pub stop_time_properties: Option<StopTimeProperties>,
    // #[cfg(feature = "realtime_mta")]
    // pub nyct_stop_time_update: Option<NyctStopTimeUpdate>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::trip_update::TripProperties)]
pub struct TripProperties {
    pub trip_id: Option<String>,

    pub start_date: Option<String>,

    pub start_time: Option<String>,

    pub shape_id: Option<String>,
    pub trip_headsign: Option<String>,
    pub trip_short_name: Option<String>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::TripUpdate)]
pub struct TripUpdate {
    #[gtfs_custom(parse_mf_req)]
    pub trip: TripDescriptor,
    #[gtfs_custom(parse_mf)]
    pub vehicle: Option<VehicleDescriptor>,
    #[gtfs_custom(parse_vec)]
    pub stop_time_update: Vec<StopTimeUpdate>,

    pub timestamp: Option<u64>,

    pub delay: Option<i32>,
    #[gtfs_custom(parse_mf)]
    pub trip_properties: Option<TripProperties>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::vehicle_position::CarriageDetails)]
pub struct CarriageDetails {
    pub id: Option<String>,
    pub label: Option<String>,
    #[gtfs_custom(parse_required)]
    pub occupancy_status: OccupancyStatus,
    #[gtfs_custom(parse_required)]
    pub occupancy_percentage: i32,
    pub carriage_sequence: Option<i32>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::VehiclePosition)]
pub struct VehiclePosition {
    #[gtfs_custom(parse_mf)]
    pub trip: Option<TripDescriptor>,
    #[gtfs_custom(parse_mf)]
    pub vehicle: Option<VehicleDescriptor>,
    #[gtfs_custom(parse_mf)]
    pub position: Option<Position>,
    pub current_stop_sequence: Option<u32>,
    pub stop_id: Option<String>,
    #[gtfs_custom(parse_required)]
    pub current_status: VehicleStopStatus,

    pub timestamp: Option<u64>,

    pub congestion_level: Option<CongestionLevel>,
    pub occupancy_status: Option<OccupancyStatus>,
    pub occupancy_percentage: Option<u32>,
    #[gtfs_custom(parse_vec)]
    pub multi_carriage_details: Vec<CarriageDetails>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::Alert)]
pub struct Alert {
    #[gtfs_custom(parse_vec)]
    pub active_period: Vec<TimeRange>,
    #[gtfs_custom(parse_vec)]
    pub informed_entity: Vec<EntitySelector>,
    #[gtfs_custom(parse_required)]
    pub cause: Cause,
    #[gtfs_custom(parse_required)]
    pub effect: Effect,
    #[gtfs_custom(parse_mf)]
    pub url: Option<TranslatedString>,
    #[gtfs_custom(parse_mf)]
    pub header_text: Option<TranslatedString>,
    #[gtfs_custom(parse_mf)]
    pub description_text: Option<TranslatedString>,
    #[gtfs_custom(parse_mf)]
    pub tts_header_text: Option<TranslatedString>,
    #[gtfs_custom(parse_mf)]
    pub tts_description_text: Option<TranslatedString>,
    #[gtfs_custom(parse_required)]
    pub severity_level: SeverityLevel,
    #[gtfs_custom(parse_mf)]
    pub image: Option<TranslatedImage>,
    #[gtfs_custom(parse_mf)]
    pub image_alternative_text: Option<TranslatedString>,
    #[gtfs_custom(parse_mf)]
    pub cause_detail: Option<TranslatedString>,
    #[gtfs_custom(parse_mf)]
    pub effect_detail: Option<TranslatedString>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::TimeRange)]
pub struct TimeRange {
    pub start: Option<u64>,

    pub end: Option<u64>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::Position)]
pub struct Position {
    #[gtfs_custom(parse_required)]
    pub latitude: f64,
    #[gtfs_custom(parse_required)]
    pub longitude: f64,
    pub bearing: Option<f64>,
    pub odometer: Option<f64>,
    pub speed: Option<f64>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::trip_descriptor::ModifiedTripSelector)]
pub struct ModifiedTripSelector {
    pub modifications_id: Option<String>,
    pub affected_trip_id: Option<String>,

    pub start_time: Option<String>,

    pub start_date: Option<String>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::TripDescriptor)]
pub struct TripDescriptor {
    pub trip_id: Option<String>,
    pub route_id: Option<String>,
    pub direction_id: Option<bool>,

    pub start_time: Option<String>,

    pub start_date: Option<String>,

    pub schedule_relationship: Option<TripScheduleRelationship>,
    #[gtfs_custom(parse_mf)]
    pub modified_trip: Option<ModifiedTripSelector>,
    // #[cfg(feature = "realtime_mta")]
    // pub nyct_trip_descriptor: Option<NyctTripDescriptor>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::VehicleDescriptor)]
pub struct VehicleDescriptor {
    pub id: Option<String>,
    pub label: Option<String>,
    pub license_plate: Option<String>,
    pub wheelchair_accessible: Option<WheelchairAccessOverride>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::EntitySelector)]
pub struct EntitySelector {
    pub agency_id: Option<String>,
    pub route_id: Option<String>,
    pub route_type: Option<i32>,
    #[gtfs_custom(parse_mf)]
    pub trip: Option<TripDescriptor>,
    pub stop_id: Option<String>,
    pub direction_id: Option<bool>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::translated_string::Translation)]
pub struct Translation {
    #[gtfs_custom(parse_required)]
    pub text: String,
    pub language: Option<String>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::TranslatedString)]
pub struct TranslatedString {
    #[gtfs_custom(parse_vec)]
    pub translation: Vec<Translation>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::translated_image::LocalizedImage)]
pub struct LocalizedImage {
    #[gtfs_custom(parse_required)]
    pub url: String,
    #[gtfs_custom(parse_required)]
    pub media_type: String,
    pub language: Option<String>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::TranslatedImage)]
pub struct TranslatedImage {
    #[gtfs_custom(parse_vec)]
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
    #[gtfs_custom(parse_mf)]
    pub stop_code: Option<TranslatedString>,
    #[gtfs_custom(parse_mf)]
    pub stop_name: Option<TranslatedString>,
    #[gtfs_custom(parse_mf)]
    pub tts_stop_name: Option<TranslatedString>,
    #[gtfs_custom(parse_mf)]
    pub stop_desc: Option<TranslatedString>,
    pub stop_lat: Option<f64>,
    pub stop_lon: Option<f64>,
    pub zone_id: Option<String>,
    #[gtfs_custom(parse_mf)]
    pub stop_url: Option<TranslatedString>,
    pub parent_station: Option<String>,

    #[gtfs_custom(parse_required)]
    pub stop_timezone: String,

    #[gtfs_custom(parse_required)]
    pub wheelchair_boarding: WheelchairBoarding,
    pub level_id: Option<String>,
    #[gtfs_custom(parse_mf)]
    pub platform_code: Option<TranslatedString>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::trip_modifications::Modification)]
pub struct Modification {
    #[gtfs_custom(parse_mf)]
    pub start_stop_selector: Option<StopSelector>,
    #[gtfs_custom(parse_mf)]
    pub end_stop_selector: Option<StopSelector>,
    #[gtfs_custom(parse_required)]
    pub propagated_modification_delay: i32,
    #[gtfs_custom(parse_vec)]
    pub replacement_stops: Vec<ReplacementStop>,
    pub service_alert_id: Option<String>,

    pub last_modified_time: Option<u64>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::trip_modifications::SelectedTrips)]
pub struct SelectedTrips {
    #[gtfs_custom(parse_vec)]
    pub trip_ids: Vec<String>,
    pub shape_id: Option<String>,
}

#[gtfs_realtime_model(crate::realtime::parse::protos::gtfs::TripModifications)]
pub struct TripModifications {
    #[gtfs_custom(parse_vec)]
    pub selected_trips: Vec<SelectedTrips>,

    #[gtfs_custom(parse_vec)]
    pub start_times: Vec<String>,

    #[gtfs_custom(parse_vec)]
    pub service_dates: Vec<String>,

    #[gtfs_custom(parse_vec)]
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
