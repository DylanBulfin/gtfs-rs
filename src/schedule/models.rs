#![cfg(feature = "schedule")]

#[cfg(feature = "schedule_parse")]
use {
    serde::{Deserialize, Serialize},
    serde_repr::{Deserialize_repr, Serialize_repr},
};

#[cfg(all(feature = "schedule_parse", feature = "chrono"))]
use serde::Deserializer;

#[cfg(feature = "chrono")]
use chrono::{NaiveDate, NaiveTime};

#[cfg(feature = "chrono_tz")]
use chrono_tz::Tz;

#[cfg(feature = "schedule_parse")]
macro_rules! derives {
    ($($item:item)+) => {
        $(
            #[derive(Serialize, Deserialize, Debug, Clone)]
            $item
        )+
    };
}

#[cfg(not(feature = "schedule_parse"))]
macro_rules! derives {
    ($($item:item)+) => {
        $(
            #[derive(Debug, Clone)]
            $item
        )+
    };
}

#[cfg(feature = "schedule_parse")]
macro_rules! derives_enum {
    ($($item:item)+) => {
        $(
            #[derive(Serialize_repr, Deserialize_repr, Debug, Clone)]
            #[repr(u8)]
            $item
        )+
    };
}

#[cfg(not(feature = "schedule_parse"))]
macro_rules! derives_enum {
    ($($item:item)+) => {
        $(
            #[derive(Debug, Clone)]
            $item
        )+
    };
}

// Utility types used by GTFS-Schedule entity
derives_enum!(
    pub enum CEMVSupport {
        NoCEMVInfo = 0,
        CEMVSupported = 1,
        CEMVUnsupported = 2,
    }

    pub enum LocationType {
        StopOrPlatform = 0,
        Station = 1,
        EntranceOrExit = 2,
        GenericNode = 3,
        BoardingArea = 4,
    }

    pub enum StopAccess {
        NoStreetAccess = 0,
        DirectAccess = 1,
    }

    pub enum ContinuousPickup {
        ContinuousPickup = 0,
        NoContinuousPickup = 1,
        MustPhoneAgency = 2,
        CoordinateWithDriver = 3,
    }

    pub enum ContinuousDropoff {
        ContinuousDropoff = 0,
        NoContinuousDropoff = 1,
        MustPhoneAgency = 2,
        CoordinateWithDriver = 3,
    }

    pub enum RouteType {
        TramStreetcarLightRail = 0,
        SubwayMetro = 1,
        Rail = 2,
        Bus = 3,
        Ferry = 4,
        CableTram = 5,
        AerialLift = 6,
        Funicular = 7,
        Trolleybus = 11,
        Monorail = 12,
    }

    pub enum WheelchairAccessibility {
        NoInfo = 0,
        SomeWheelchairSupport = 1,
        NoWheelchairSupport = 2,
    }

    pub enum BikeSupport {
        NoInfo = 0,
        SomeBikeSupport = 1,
        NoBikeSupport = 2,
    }

    pub enum CarSupport {
        NoInfo = 0,
        SomeCarSupport = 1,
        NoCarSupport = 2,
    }

    pub enum PickupType {
        ScheduledPickup = 0,
        NoPickup = 1,
        MustPhoneAgency = 2,
        CoordinateWithDriver = 3,
    }
    pub enum DropoffType {
        ScheduledDropoff = 0,
        NoDropoff = 1,
        MustPhoneAgency = 2,
        CoordinateWithDriver = 3,
    }

    pub enum Timepoint {
        Approximate = 0,
        Exact = 1,
    }

    pub enum DaySchedule {
        ServiceNotAvailable = 0,
        ServiceAvailable = 1,
    }

    pub enum ExceptionType {
        ServiceAdded = 1,
        ServiceRemoved = 2,
    }

    pub enum PaymentMethod {
        OnBoard = 0,
        PreBoarding = 1,
    }

    pub enum TransfersLimit {
        NoTransfers = 0,
        OneTransfer = 1,
        TwoTransfers = 2,
    }

    pub enum FareMediaType {
        None = 0,
        PaperTicket = 1,
        PhysicalCard = 2,
        CEMV = 3,
        MobilePay = 4,
    }

    pub enum DurationLimitType {
        DepartureToArrival = 0,
        DepartureToDeparture = 1,
        ArrivalToDeparture = 2,
        ArrivalToArrival = 3,
    }

    pub enum FareTransferType {
        APlusAB = 0,
        APlusABPlusB = 1,
        AB = 2,
    }

    pub enum TripTiming {
        FrequencyBased = 0,
        ScheduleBased = 1,
    }

    pub enum TransferType {
        RecommendedTransfer = 0,
        TimedTransfer = 1,
        TimeRequired = 2,
        NoTransfers = 3,
        InSeatTransfer = 4,
        MustLeaveAndReenter = 5,
    }

    pub enum PathwayMode {
        Walkway = 1,
        Stairs = 2,
        MovingSidewalk = 3,
        Escalator = 4,
        Elevator = 5,
        FareGate = 6,
        ExitGate = 7,
    }

    pub enum BookingType {
        RealTimeBooking = 0,
        SameDayBooking = 1,
        PriorDaysBooking = 2,
    }

    pub enum TripDirection {
        DirectionOne = 0,
        DirectionTwo = 1,
    }
);

#[derive(Clone, Debug)]
pub struct GTFSData {
    pub agencies: Vec<Agency>,
    pub stops: Vec<Stop>,
    pub routes: Vec<Route>,
    pub trips: Vec<Trip>,
    pub stop_times: Vec<StopTime>,
    pub calendar: Vec<Calendar>,
    pub calendar_dates: Vec<CalendarDate>,
    pub fare_attributes: Vec<FareAttribute>,
    pub fare_rules: Vec<FareRule>,
    pub timeframes: Vec<Timeframe>,
    pub rider_categories: Vec<RiderCategory>,
    pub fare_media: Vec<FareMedia>,
    pub fare_products: Vec<FareProduct>,
    pub fare_leg_rules: Vec<FareLegRule>,
    pub fare_leg_join_rules: Vec<FareLegJoinRule>,
    pub fare_transfer_rules: Vec<FareTransferRule>,
    pub areas: Vec<Area>,
    pub stop_areas: Vec<StopArea>,
    pub networks: Vec<Network>,
    pub route_networks: Vec<RouteNetwork>,
    pub shapes: Vec<Shape>,
    pub frequencies: Vec<Frequency>,
    pub transfers: Vec<Transfer>,
    pub pathways: Vec<Pathway>,
    pub levels: Vec<Level>,
    pub location_groups: Vec<LocationGroup>,
    pub location_group_stops: Vec<LocationGroupStop>,
    pub locations: Vec<Location>,
    pub booking_rules: Vec<BookingRule>,
    pub translations: Vec<Translation>,
    pub feed_info: Vec<FeedInfo>,
    pub attributions: Vec<Attribution>,
}

#[cfg(all(feature = "schedule_parse", feature = "chrono"))]
pub fn deser_date<'de, D>(d: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(d)?;

    let year: i32 = (&s[0..4]).parse().unwrap_or_default();
    let month: u32 = (&s[4..6]).parse().unwrap_or_default();
    let day: u32 = (&s[6..8]).parse().unwrap_or_default();

    let date = NaiveDate::from_ymd_opt(year, month, day).unwrap_or_default();

    Ok(date)
}

#[cfg(all(feature = "schedule_parse", feature = "chrono"))]
pub fn deser_opt_date<'de, D>(d: D) -> Result<Option<NaiveDate>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(d)?;

    let year: i32 = (&s[0..4]).parse().unwrap_or(i32::MIN);
    let month: u32 = (&s[4..6]).parse().unwrap_or(u32::MAX);
    let day: u32 = (&s[6..8]).parse().unwrap_or(u32::MAX);

    Ok(NaiveDate::from_ymd_opt(year, month, day))
}

#[cfg(all(feature = "schedule_parse", feature = "chrono"))]
pub fn deser_time<'de, D>(d: D) -> Result<NaiveTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(d)?;

    let hour: u32 = (&s[0..2]).parse().unwrap_or_default();
    let min: u32 = (&s[3..5]).parse().unwrap_or_default();
    let sec: u32 = (&s[6..8]).parse().unwrap_or_default();

    let time = NaiveTime::from_hms_opt(hour, min, sec).unwrap_or_default();

    Ok(time)
}

#[cfg(all(feature = "schedule_parse", feature = "chrono"))]
pub fn deser_opt_time<'de, D>(d: D) -> Result<Option<NaiveTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(d)?;

    let hour: u32 = (&s[0..2]).parse().unwrap_or(u32::MAX);
    let min: u32 = (&s[3..5]).parse().unwrap_or(u32::MAX);
    let sec: u32 = (&s[6..8]).parse().unwrap_or(u32::MAX);

    Ok(NaiveTime::from_hms_opt(hour, min, sec))
}

derives!(
    pub struct Agency {
        pub agency_id: Option<String>,
        pub agency_name: String,
        pub agency_url: String,

        #[cfg(not(feature = "chrono_tz"))]
        pub agency_timezone: String,
        #[cfg(feature = "chrono_tz")]
        pub agency_timezone: Tz,

        pub agency_lang: Option<String>,
        pub agency_phone: Option<String>,
        pub agency_fare_url: Option<String>,
        pub agency_email: Option<String>,
        pub cemv_support: Option<CEMVSupport>,
    }

    pub struct Stop {
        pub stop_id: String,
        pub stop_code: Option<String>,
        pub stop_name: Option<String>,
        pub tts_stop_name: Option<String>,
        pub stop_desc: Option<String>,
        pub stop_lat: Option<String>,
        pub stop_lon: Option<String>,
        pub zone_id: Option<String>,
        pub stop_url: Option<String>,
        pub location_type: Option<LocationType>,
        pub parent_station: Option<String>,

        #[cfg(not(feature = "chrono_tz"))]
        pub stop_timezone: Option<String>,
        #[cfg(feature = "chrono_tz")]
        pub stop_timezone: Option<Tz>,

        pub wheelchair_boarding: Option<WheelchairAccessibility>,
        pub level_id: Option<String>,
        pub platform_code: Option<String>,
        pub stop_access: Option<StopAccess>,
    }

    pub struct Route {
        pub route_id: String,
        pub agency_id: Option<String>,
        pub route_short_name: Option<String>,
        pub route_long_name: Option<String>,
        pub route_desc: Option<String>,
        pub route_type: Option<RouteType>,
        pub route_url: Option<String>,
        pub route_color: Option<String>,
        pub route_text_color: Option<String>,
        pub route_sort_order: Option<u32>,
        pub continuous_pickup: Option<ContinuousPickup>,
        pub continuous_drop_off: Option<ContinuousDropoff>,
        pub network_id: Option<String>,
        pub cemv_support: Option<CEMVSupport>,
    }

    pub struct Trip {
        pub route_id: String,
        pub service_id: String,
        pub trip_id: String,
        pub trip_headsign: Option<String>,
        pub trip_short_name: Option<String>,
        pub direction_id: Option<TripDirection>,
        pub block_id: Option<String>,
        pub shape_id: Option<String>,
        pub wheelchair_accessible: Option<WheelchairAccessibility>,
        pub bikes_allowed: Option<BikeSupport>,
        pub cars_allowed: Option<CarSupport>,
        pub safe_duration_factor: Option<f64>,
        pub safe_duration_offset: Option<f64>,
    }

    pub struct StopTime {
        pub trip_id: String,

        #[cfg(not(feature = "chrono"))]
        pub arrival_time: Option<String>,
        #[cfg(all(feature = "chrono", feature = "schedule_parse"))]
        #[serde(deserialize_with = "deser_opt_time")]
        #[serde(default)]
        pub arrival_time: Option<NaiveTime>,
        #[cfg(all(feature = "chrono", not(feature = "schedule_parse")))]
        pub arrival_time: Option<NaiveTime>,

        #[cfg(not(feature = "chrono"))]
        pub departure_time: Option<String>,
        #[cfg(all(feature = "chrono", feature = "schedule_parse"))]
        #[serde(deserialize_with = "deser_opt_time")]
        #[serde(default)]
        pub departure_time: Option<NaiveTime>,
        #[cfg(all(feature = "chrono", not(feature = "schedule_parse")))]
        pub departure_time: Option<NaiveTime>,

        pub stop_id: Option<String>,
        pub location_group_id: Option<String>,
        pub location_id: Option<String>,
        pub stop_sequence: Option<u32>,
        pub stop_headsign: Option<String>,

        #[cfg(not(feature = "chrono"))]
        pub start_pickup_drop_off_window: Option<String>,
        #[cfg(all(feature = "chrono", feature = "schedule_parse"))]
        #[serde(deserialize_with = "deser_opt_time")]
        #[serde(default)]
        pub start_pickup_drop_off_window: Option<NaiveTime>,
        #[cfg(all(feature = "chrono", not(feature = "schedule_parse")))]
        pub start_pickup_drop_off_window: Option<NaiveTime>,

        #[cfg(not(feature = "chrono"))]
        pub end_pickup_drop_off_window: Option<String>,
        #[cfg(all(feature = "chrono", feature = "schedule_parse"))]
        #[serde(deserialize_with = "deser_opt_time")]
        #[serde(default)]
        pub end_pickup_drop_off_window: Option<NaiveTime>,
        #[cfg(all(feature = "chrono", not(feature = "schedule_parse")))]
        pub end_pickup_drop_off_window: Option<NaiveTime>,

        pub pickup_type: Option<PickupType>,
        pub drop_off_type: Option<DropoffType>,
        pub continuous_pickup: Option<ContinuousPickup>,
        pub continuous_drop_off: Option<ContinuousDropoff>,
        pub shape_dist_traveled: Option<f64>,
        pub timepoint: Option<Timepoint>,
        pub pickup_booking_rule_id: Option<String>,
        pub drop_off_booking_rule_id: Option<String>,
    }

    pub struct Calendar {
        pub service_id: String,
        pub monday: DaySchedule,
        pub tuesday: DaySchedule,
        pub wednesday: DaySchedule,
        pub thursday: DaySchedule,
        pub friday: DaySchedule,
        pub saturday: DaySchedule,
        pub sunday: DaySchedule,

        #[cfg(not(feature = "chrono"))]
        pub start_date: String,
        #[cfg(all(feature = "chrono", feature = "schedule_parse"))]
        #[serde(deserialize_with = "deser_date")]
        pub start_date: NaiveDate,
        #[cfg(all(feature = "chrono", not(feature = "schedule_parse")))]
        pub start_date: NaiveDate,

        #[cfg(not(feature = "chrono"))]
        pub end_date: String,
        #[cfg(all(feature = "chrono", feature = "schedule_parse"))]
        #[serde(deserialize_with = "deser_date")]
        pub end_date: NaiveDate,
        #[cfg(all(feature = "chrono", not(feature = "schedule_parse")))]
        pub end_date: NaiveDate,
    }

    pub struct CalendarDate {
        pub service_id: String,

        #[cfg(not(feature = "chrono"))]
        pub date: String,
        #[cfg(all(feature = "chrono", feature = "schedule_parse"))]
        #[serde(deserialize_with = "deser_date")]
        pub date: NaiveDate,
        #[cfg(all(feature = "chrono", not(feature = "schedule_parse")))]
        pub date: NaiveDate,

        pub exception_type: ExceptionType,
    }

    pub struct FareAttribute {
        pub fare_id: String,
        pub price: f64,
        pub currency_type: String,
        pub payment_method: PaymentMethod,
        pub transfers: TransfersLimit,
        pub agency_id: Option<String>,
        pub transfer_duration: Option<u32>,
    }

    pub struct FareRule {
        pub fare_id: String,
        pub route_id: Option<String>,
        pub origin_id: Option<String>,
        pub destination_id: Option<String>,
        pub contains_id: Option<String>,
    }

    pub struct Timeframe {
        pub timeframe_group_id: String,

        #[cfg(not(feature = "chrono"))]
        pub start_time: Option<String>,
        #[cfg(all(feature = "chrono", feature = "schedule_parse"))]
        #[serde(deserialize_with = "deser_opt_time")]
        #[serde(default)]
        pub start_time: Option<NaiveTime>,
        #[cfg(all(feature = "chrono", not(feature = "schedule_parse")))]
        pub start_time: Option<NaiveTime>,

        #[cfg(not(feature = "chrono"))]
        pub end_time: Option<String>,
        #[cfg(all(feature = "chrono", feature = "schedule_parse"))]
        #[serde(deserialize_with = "deser_opt_time")]
        #[serde(default)]
        pub end_time: Option<NaiveTime>,
        #[cfg(all(feature = "chrono", not(feature = "schedule_parse")))]
        pub end_time: Option<NaiveTime>,

        pub service_id: String,
    }

    pub struct RiderCategory {
        pub rider_category_id: String,
        pub rider_category_name: String,
        pub is_default_fare_category: Option<bool>,
        pub eligibility_url: Option<String>,
    }

    pub struct FareMedia {
        pub fare_media_id: String,
        pub fare_media_name: Option<String>,
        pub fare_media_type: FareMediaType,
    }

    pub struct FareProduct {
        pub fare_product_id: String,
        pub fare_product_name: Option<String>,
        pub rider_category_id: Option<String>,
        pub fare_media_id: Option<String>,
        pub amount: String,
        pub currency: String,
    }

    pub struct FareLegRule {
        pub leg_group_id: Option<String>,
        pub network_id: Option<String>,
        pub from_area_id: Option<String>,
        pub to_area_id: Option<String>,
        pub from_timeframe_group_id: Option<String>,
        pub to_timeframe_group_id: Option<String>,
        pub fare_product_id: String,
        pub rule_priority: Option<u32>,
    }

    pub struct FareLegJoinRule {
        pub from_network_id: String,
        pub to_network_id: String,
        pub from_stop_id: Option<String>,
        pub to_stop_id: Option<String>,
    }

    pub struct FareTransferRule {
        pub from_leg_group_id: Option<String>,
        pub to_leg_group_id: Option<String>,
        pub transfer_count: Option<i32>,
        pub duration_limit: Option<u32>,
        pub duration_limit_type: Option<DurationLimitType>,
        pub fare_transfer_type: FareTransferType,
        pub fare_product_id: Option<String>,
    }

    pub struct Area {
        pub area_id: String,
        pub area_name: Option<String>,
    }

    pub struct StopArea {
        pub area_id: String,
        pub stop_id: String,
    }

    pub struct Network {
        pub network_id: String,
        pub network_name: Option<String>,
    }

    pub struct RouteNetwork {
        pub network_id: String,
        pub route_id: String,
    }

    pub struct Shape {
        pub shape_id: String,
        pub shape_pt_lat: f64,
        pub shape_pt_lon: f64,
        pub shape_pt_sequence: u32,
        pub shape_dist_traveled: Option<f64>,
    }

    pub struct Frequency {
        pub trip_id: String,

        #[cfg(not(feature = "chrono"))]
        pub start_time: String,
        #[cfg(all(feature = "chrono", feature = "schedule_parse"))]
        #[serde(deserialize_with = "deser_time")]
        pub start_time: NaiveTime,
        #[cfg(all(feature = "chrono", not(feature = "schedule_parse")))]
        pub start_time: NaiveTime,

        #[cfg(not(feature = "chrono"))]
        pub end_time: String,
        #[cfg(all(feature = "chrono", feature = "schedule_parse"))]
        #[serde(deserialize_with = "deser_time")]
        pub end_time: NaiveTime,
        #[cfg(all(feature = "chrono", not(feature = "schedule_parse")))]
        pub end_time: NaiveTime,

        pub headway_secs: u32,
        pub exact_times: Option<TripTiming>,
    }

    pub struct Transfer {
        pub from_stop_id: Option<String>,
        pub to_stop_id: Option<String>,
        pub from_route_id: Option<String>,
        pub to_route_id: Option<String>,
        pub from_trip_id: Option<String>,
        pub to_trip_id: Option<String>,
        pub transfer_type: TransferType,
        pub min_transfer_time: Option<u32>,
    }

    pub struct Pathway {
        pub pathway_id: String,
        pub from_stop_id: String,
        pub to_stop_id: String,
        pub pathway_mode: PathwayMode,
        pub is_bidirectional: bool,
        pub length: Option<f64>,
        pub traversal_time: Option<u32>,
        pub stair_count: Option<i32>,
        pub max_slope: Option<f64>,
        pub min_width: Option<f64>,
        pub signposted_as: Option<String>,
        pub reversed_signposted_as: Option<String>,
    }

    pub struct Level {
        pub level_id: String,
        pub level_index: f64,
        pub level_name: Option<String>,
    }

    pub struct LocationGroup {
        pub location_group_id: String,
        pub location_group_name: Option<String>,
    }

    pub struct LocationGroupStop {
        pub location_group_id: String,
        pub stop_id: String,
    }

    // TODO Add GeoJSON parsing to support Location parsing
    pub struct Location {}

    pub struct BookingRule {
        pub booking_rule_id: String,
        pub booking_type: BookingType,
        pub prior_notice_duration_min: Option<i32>,
        pub prior_notice_duration_max: Option<i32>,
        pub prior_notice_last_day: Option<i32>,

        #[cfg(not(feature = "chrono"))]
        pub prior_notice_last_time: Option<String>,
        #[cfg(all(feature = "chrono", feature = "schedule_parse"))]
        #[serde(deserialize_with = "deser_opt_time")]
        #[serde(default)]
        pub prior_notice_last_time: Option<NaiveTime>,
        #[cfg(all(feature = "chrono", not(feature = "schedule_parse")))]
        pub prior_notice_last_time: Option<NaiveTime>,

        pub prior_notice_start_day: Option<i32>,

        #[cfg(not(feature = "chrono"))]
        pub prior_notice_start_time: Option<String>,
        #[cfg(all(feature = "chrono", feature = "schedule_parse"))]
        #[serde(deserialize_with = "deser_opt_time")]
        #[serde(default)]
        pub prior_notice_start_time: Option<NaiveTime>,
        #[cfg(all(feature = "chrono", not(feature = "schedule_parse")))]
        pub prior_notice_start_time: Option<NaiveTime>,

        pub prior_notice_service_id: Option<String>,
        pub message: Option<String>,
        pub pickup_message: Option<String>,
        pub drop_off_message: Option<String>,
        pub phone_number: Option<String>,
        pub info_url: Option<String>,
        pub booking_url: Option<String>,
    }

    pub struct Translation {
        pub table_name: String,
        pub field_name: String,
        pub language: String,
        pub translation: String,
        pub record_id: Option<String>,
        pub record_sub_id: Option<String>,
        pub field_value: Option<String>,
    }

    pub struct FeedInfo {
        pub feed_publisher_name: String,
        pub feed_publisher_url: String,
        pub feed_lang: String,
        pub default_lang: Option<String>,

        #[cfg(not(feature = "chrono"))]
        pub feed_start_date: Option<String>,
        #[cfg(all(feature = "chrono", feature = "schedule_parse"))]
        #[serde(deserialize_with = "deser_opt_date")]
        #[serde(default)]
        pub feed_start_date: Option<NaiveDate>,
        #[cfg(all(feature = "chrono", not(feature = "schedule_parse")))]
        pub feed_start_date: Option<NaiveDate>,

        #[cfg(not(feature = "chrono"))]
        pub feed_end_date: Option<String>,
        #[cfg(all(feature = "chrono", feature = "schedule_parse"))]
        #[serde(deserialize_with = "deser_opt_date")]
        #[serde(default)]
        pub feed_end_date: Option<NaiveDate>,
        #[cfg(all(feature = "chrono", not(feature = "schedule_parse")))]
        pub feed_end_date: Option<NaiveDate>,

        pub feed_version: Option<String>,
        pub feed_contact_email: Option<String>,
        pub feed_contact_url: Option<String>,
    }

    pub struct Attribution {
        pub attribution_id: Option<String>,
        pub agency_id: Option<String>,
        pub route_id: Option<String>,
        pub trip_id: Option<String>,
        pub organization_name: String,
        pub is_producer: Option<bool>,
        pub is_operator: Option<bool>,
        pub is_authority: Option<bool>,
        pub attribution_url: Option<String>,
        pub attribution_email: Option<String>,
        pub attribution_phone: Option<String>,
    }
);
