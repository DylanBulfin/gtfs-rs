#[cfg(feature = "schedule_parse")]
use {
    serde::{Deserialize, Serialize},
    serde_repr::{Deserialize_repr, Serialize_repr},
};

#[cfg(feature = "schedule_parse")]
macro_rules! derives {
    ($($item:item)+) => {
        $(
            #[derive(Serialize, Deserialize, Debug, Clone)]
            $item
        )+
    };
}

#[cfg(feature = "schedule_parse")]
macro_rules! derives_c_enum {
    ($($item:item)+) => {
        $(
            #[derive(Serialize_repr, Deserialize_repr, Debug, Clone)]
            #[repr(u8)]
            $item
        )+
    };
}

// Utility types used by GTFS-Schedule entity
derives_c_enum!(
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
);

// Entities corresponding to GTFS-Schedule data files
derives!(
    pub struct Agency {
        agency_id: Option<String>,
        agency_name: String,
        agency_url: String,
        agency_timezone: String, // TODO parse into better timezone object
        agency_lang: Option<String>,
        agency_phone: Option<String>,
        agency_fare_url: Option<String>,
        agency_email: Option<String>,
        cemv_support: Option<CEMVSupport>,
    }

    pub struct Stop {
        stop_id: String,
        stop_code: Option<String>,
        stop_name: Option<String>,
        tts_stop_name: Option<String>,
        stop_desc: Option<String>,
        stop_lat: Option<String>,
        stop_lon: Option<String>,
        zone_id: Option<String>,
        stop_url: Option<String>,
        location_type: Option<LocationType>,
        parent_station: Option<String>,
        stop_timezone: Option<String>, // TODO parse into better timezone object
        // This field has different behaviors based on location_type, making it difficult to encode
        // in enum
        // TODO look into structuring data more, have Stop as an enum with variants based on
        // location_type
        wheelchair_boarding: Option<u32>,
        level_id: Option<String>,
        platform_code: Option<String>,
        stop_access: Option<StopAccess>,
    }

    pub struct Route {
        route_id: String,
        agency_id: Option<String>,
        route_short_name: Option<String>,
        route_long_name: Option<String>,
        route_desc: Option<String>,
        route_type: Option<RouteType>,
        route_url: Option<String>,
        route_color: Option<String>,
        route_text_color: Option<String>,
        route_sort_order: Option<u32>,
        continuous_pickup: Option<ContinuousPickup>,
        continuous_drop_off: Option<ContinuousDropoff>,
        network_id: Option<String>,
        cemv_support: Option<CEMVSupport>,
    }

    pub struct Trip {
        route_id: String,
        service_id: String,
        trip_id: String,
        trip_headsign: Option<String>,
        trip_short_name: Option<String>,
        // direction_id does not assign meaning to either direction (e.g. uptown, downtown), inbound and
        // outbound are mentioned as examples. This makes it difficult to design a typed enum
        direction_id: Option<u32>,
        block_id: Option<String>,
        shape_id: Option<String>,
        wheelchair_accessible: Option<WheelchairAccessibility>,
        bikes_allowed: Option<BikeSupport>,
        cars_allowed: Option<CarSupport>,
        safe_duration_factor: Option<f64>,
        safe_duration_offset: Option<f64>,
    }

    pub struct StopTime {
        trip_id: String,
        arrival_time: Option<String>,   // TODO Time parsing
        departure_time: Option<String>, // TODO Time parsing
        stop_id: Option<String>,
        location_group_id: Option<String>,
        location_id: Option<String>,
        stop_sequence: Option<u32>,
        stop_headsign: Option<String>,
        start_pickup_drop_off_window: Option<String>, // TODO Time parsing
        end_pickup_drop_off_window: Option<String>,   // TODO Time parsing
        pickup_type: Option<PickupType>,
        drop_off_type: Option<DropoffType>,
        continuous_pickup: Option<ContinuousPickup>,
        continuous_drop_off: Option<ContinuousDropoff>,
        shape_dist_traveled: Option<f64>,
        timepoint: Option<Timepoint>,
        pickup_booking_rule_id: Option<String>,
        drop_off_booking_rule_id: Option<String>,
    }

    pub struct Calendar {
        service_id: String,
        monday: DaySchedule,
        tuesday: DaySchedule,
        wednesday: DaySchedule,
        thursday: DaySchedule,
        friday: DaySchedule,
        saturday: DaySchedule,
        sunday: DaySchedule,
        start_date: String, //TODO time parsing
        end_date: String,   //TODO time parsing
    }

    pub struct CalendarDate {
        service_id: String,
        date: String, //TODO time parsing
        exception_type: ExceptionType,
    }

    pub struct FareAttribute {
        fare_id: String,
        price: f64,
        currency_type: String,
        payment_method: PaymentMethod,
        transfers: TransfersLimit,
        agency_id: Option<String>,
        transfer_duration: Option<u32>,
    }

    pub struct FareRule {
        fare_id: String,
        route_id: Option<String>,
        origin_id: Option<String>,
        destination_id: Option<String>,
        contains_id: Option<String>,
    }

    pub struct Timeframe {
        timeframe_group_id: String,
        start_time: Option<String>, //TODO time parsing
        end_time: Option<String>,   //TODO time parsing
        service_id: String,
    }

    pub struct RiderCategory {
        rider_category_id: String,
        rider_category_name: String,
        is_default_fare_category: Option<bool>,
        eligibility_url: Option<String>,
    }

    pub struct FareMedia {
        fare_media_id: String,
        fare_media_name: Option<String>,
        fare_media_type: FareMediaType,
    }

    pub struct FareProduct {
        fare_product_id: String,
        fare_product_name: Option<String>,
        rider_category_id: Option<String>,
        fare_media_id: Option<String>,
        amount: String,
        currency: String,
    }

    pub struct FareLegRule {
        leg_group_id: Option<String>,
        network_id: Option<String>,
        from_area_id: Option<String>,
        to_area_id: Option<String>,
        from_timeframe_group_id: Option<String>,
        to_timeframe_group_id: Option<String>,
        fare_product_id: String,
        rule_priority: Option<u32>,
    }

    pub struct FareLegJoinRule {
        from_network_id: String,
        to_network_id: String,
        from_stop_id: Option<String>,
        to_stop_id: Option<String>,
    }

    pub struct FareTransferRule {
        from_leg_group_id: Option<String>,
        to_leg_group_id: Option<String>,
        transfer_count: Option<i32>,
        duration_limit: Option<u32>,
        duration_limit_type: Option<DurationLimitType>,
        fare_transfer_type: FareTransferType,
        fare_product_id: Option<String>,
    }

    pub struct Area {
        area_id: String,
        area_name: Option<String>,
    }

    pub struct StopArea {
        area_id: String,
        stop_id: String,
    }

    pub struct Network {
        network_id: String,
        network_name: Option<String>,
    }

    pub struct RouteNetwork {
        network_id: String,
        route_id: String,
    }

    pub struct Shape {
        shape_id: String,
        shape_pt_lat: f64,
        shape_pt_lon: f64,
        shape_pt_sequence: u32,
        shape_dist_traveled: f64,
    }

    pub struct Frequency {
        trip_id: String,
        start_time: String, // TODO Time parsing
        end_time: String,   // TODO Time parsing
        headway_secs: u32,
        exact_times: Option<TripTiming>,
    }

    pub struct Transfer {
        from_stop_id: Option<String>,
        to_stop_id: Option<String>,
        from_route_id: Option<String>,
        to_route_id: Option<String>,
        from_trip_id: Option<String>,
        to_trip_id: Option<String>,
        transfer_type: TransferType,
        min_transfer_time: Option<u32>,
    }

    pub struct Pathway {
        pathway_id: String,
        from_stop_id: String,
        to_stop_id: String,
        pathway_mode: PathwayMode,
        is_bidirectional: bool,
        length: Option<f64>,
        traversal_time: Option<u32>,
        stair_count: Option<i32>,
        max_slope: Option<f64>,
        min_width: Option<f64>,
        signposted_as: Option<String>,
        reversed_signposted_as: Option<String>,
    }

    pub struct Level {
        level_id: String,
        level_index: f64,
        level_name: Option<String>,
    }

    pub struct LocationGroup {
        location_group_id: String,
        location_group_name: Option<String>,
    }

    pub struct LocationGroupStop {
        location_group_id: String,
        stop_id: String,
    }

    // TODO Add GeoJSON parsing to support Location parsing
    pub struct Location {}

    pub struct BookingRule {
        booking_rule_id: String,
        booking_type: BookingType,
        prior_notice_duration_min: Option<i32>,
        prior_notice_duration_max: Option<i32>,
        prior_notice_last_day: Option<i32>,
        prior_notice_last_time: Option<String>, //TODO Time parsing
        prior_notice_start_day: Option<i32>,
        prior_notice_start_time: Option<String>, //TODO Time parsing
        prior_notice_service_id: Option<String>,
        message: Option<String>,
        pickup_message: Option<String>,
        drop_off_message: Option<String>,
        phone_number: Option<String>,
        info_url: Option<String>,
        booking_url: Option<String>,
    }

    pub struct Translation {
        table_name: String,
        field_name: String,
        language: String,
        translation: String,
        record_id: Option<String>,
        record_sub_id: Option<String>,
        field_value: Option<String>,
    }

    pub struct FeedInfo {
        feed_publisher_name: String,
        feed_publisher_url: String,
        feed_lang: String,
        default_lang: Option<String>,
        feed_start_date: Option<String>, // TODO Time parsing
        feed_end_date: Option<String>,   // TODO Time parsing
        feed_version: Option<String>,
        feed_contact_email: Option<String>,
        feed_contact_url: Option<String>,
    }

    pub struct Attribution {
        attribution_id: Option<String>,
        agency_id: Option<String>,
        route_id: Option<String>,
        trip_id: Option<String>,
        organization_name: String,
        is_producer: Option<bool>,
        is_operator: Option<bool>,
        is_authority: Option<bool>,
        attribution_url: Option<String>,
        attribution_email: Option<String>,
        attribution_phone: Option<String>,
    }
);
