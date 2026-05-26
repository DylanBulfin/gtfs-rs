use std::io::{Read, Seek};

use serde::de::DeserializeOwned;
use zip::{ZipArchive, result::ZipError};

use crate::{
    error::Error,
    schedule::{models::GTFSData, parse::parse_entity},
};

pub fn process_file_inside_zip<T, R, S>(
    archive: &mut ZipArchive<R>,
    filename: S,
) -> Result<Vec<T>, Error>
where
    T: DeserializeOwned,
    R: Read + Seek,
    S: AsRef<str>,
{
    let file = archive.by_name(filename.as_ref());

    match file {
        Ok(rdr) => Ok(parse_entity(rdr)?),
        Err(ZipError::FileNotFound) => Ok(Vec::new()),
        Err(e) => Err(e.into()),
    }
}

pub fn parse_from_zip<R>(mut zip: ZipArchive<R>) -> Result<GTFSData, Error>
where
    R: Read + Seek,
{
    Ok(GTFSData {
        agencies: process_file_inside_zip(&mut zip, "agency.txt")?,
        stops: process_file_inside_zip(&mut zip, "stops.txt")?,
        routes: process_file_inside_zip(&mut zip, "routes.txt")?,
        trips: process_file_inside_zip(&mut zip, "trips.txt")?,
        stop_times: process_file_inside_zip(&mut zip, "stop_times.txt")?,
        calendar: process_file_inside_zip(&mut zip, "calendar.txt")?,
        calendar_dates: process_file_inside_zip(&mut zip, "calendar_dates.txt")?,
        fare_attributes: process_file_inside_zip(&mut zip, "fare_attributes.txt")?,
        fare_rules: process_file_inside_zip(&mut zip, "fare_rules.txt")?,
        timeframes: process_file_inside_zip(&mut zip, "timeframes.txt")?,
        rider_categories: process_file_inside_zip(&mut zip, "rider_categories.txt")?,
        fare_media: process_file_inside_zip(&mut zip, "fare_media.txt")?,
        fare_products: process_file_inside_zip(&mut zip, "fare_products.txt")?,
        fare_leg_rules: process_file_inside_zip(&mut zip, "fare_leg_rules.txt")?,
        fare_leg_join_rules: process_file_inside_zip(&mut zip, "fare_leg_join_rules.txt")?,
        fare_transfer_rules: process_file_inside_zip(&mut zip, "fare_transfer_rules.txt")?,
        areas: process_file_inside_zip(&mut zip, "areas.txt")?,
        stop_areas: process_file_inside_zip(&mut zip, "stop_areas.txt")?,
        networks: process_file_inside_zip(&mut zip, "networks.txt")?,
        route_networks: process_file_inside_zip(&mut zip, "route_networks.txt")?,
        shapes: process_file_inside_zip(&mut zip, "shapes.txt")?,
        frequencies: process_file_inside_zip(&mut zip, "frequencies.txt")?,
        transfers: process_file_inside_zip(&mut zip, "transfers.txt")?,
        pathways: process_file_inside_zip(&mut zip, "pathways.txt")?,
        levels: process_file_inside_zip(&mut zip, "levels.txt")?,
        location_groups: process_file_inside_zip(&mut zip, "location_groups.txt")?,
        location_group_stops: process_file_inside_zip(&mut zip, "location_group_stops.txt")?,
        locations: process_file_inside_zip(&mut zip, "locations.txt")?,
        booking_rules: process_file_inside_zip(&mut zip, "booking_rules.txt")?,
        translations: process_file_inside_zip(&mut zip, "translations.txt")?,
        feed_info: process_file_inside_zip(&mut zip, "feed_info.txt")?,
        attributions: process_file_inside_zip(&mut zip, "attributions.txt")?,
    })
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use zip::ZipArchive;

    use crate::schedule::zip::parse_from_zip;

    #[test]
    fn test_zip() {
        let path = "test_data/gtfs_subway.zip";

        let data = parse_from_zip(ZipArchive::new(File::open(path).unwrap()).unwrap()).unwrap();
    }
}
