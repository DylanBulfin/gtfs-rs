use std::io::Read;

use serde::de::DeserializeOwned;

use crate::{
    error::Error,
    schedule::models::{
        Agency, Area, Attribution, BookingRule, Calendar, CalendarDate, FareAttribute,
        FareLegJoinRule, FareLegRule, FareMedia, FareProduct, FareRule, FareTransferRule, FeedInfo,
        Frequency, Level, Location, LocationGroup, LocationGroupStop, Network, Pathway,
        RiderCategory, Route, RouteNetwork, Shape, Stop, StopArea, StopTime, Timeframe, Transfer,
        Translation, Trip,
    },
};

// TODO remove Box<dyn Error>
pub fn parse_entity<T, R>(rdr: R) -> Result<Vec<T>, Error>
where
    T: DeserializeOwned,
    R: Read,
{
    let mut csv_rdr = csv::Reader::from_reader(rdr);
    csv_rdr
        .deserialize()
        .map(|r| r.map_err(Into::into))
        .collect()
}

macro_rules! parse_entities {
    ($(($id:ident, $t:ty)),+) => {
        $(
            pub fn $id<R>(rdr: R) -> Result<Vec<$t>, Error> where R: Read {
                parse_entity::<$t, R>(rdr)
            }
        )+
    }
}
parse_entities!(
    (parse_agencies, Agency),
    (parse_stops, Stop),
    (parse_routes, Route),
    (parse_trips, Trip),
    (parse_stop_times, StopTime),
    (parse_calendar, Calendar),
    (parse_calendar_dates, CalendarDate),
    (parse_fare_attributes, FareAttribute),
    (parse_fare_rules, FareRule),
    (parse_timeframes, Timeframe),
    (parse_rider_categories, RiderCategory),
    (parse_fare_media, FareMedia),
    (parse_fare_products, FareProduct),
    (parse_fare_leg_rules, FareLegRule),
    (parse_fare_leg_join_rules, FareLegJoinRule),
    (parse_fare_transfer_rules, FareTransferRule),
    (parse_areas, Area),
    (parse_stop_areas, StopArea),
    (parse_networks, Network),
    (parse_route_networks, RouteNetwork),
    (parse_shapes, Shape),
    (parse_frequencies, Frequency),
    (parse_transfers, Transfer),
    (parse_pathways, Pathway),
    (parse_levels, Level),
    (parse_location_groups, LocationGroup),
    (parse_location_group_stops, LocationGroupStop),
    (parse_locations, Location),
    (parse_booking_rules, BookingRule),
    (parse_translations, Translation),
    (parse_feed_info, FeedInfo),
    (parse_attributions, Attribution)
);

#[cfg(test)]
mod tests {
    use std::fs::File;

    use crate::schedule::{
        models::{Agency, CalendarDate},
        parse::parse_entity,
    };

    #[test]
    pub fn test() {
        let reader = File::open("test_data/schedule/agency.txt").unwrap();
        let _res: Vec<Agency> = parse_entity(reader).unwrap();

        let reader = File::open("test_data/schedule/calendar_dates.txt").unwrap();
        let _res: Vec<CalendarDate> = parse_entity(reader).unwrap();
    }
}
