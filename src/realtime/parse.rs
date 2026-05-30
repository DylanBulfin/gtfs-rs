use gtfs_macros::gtfs_realtime_model;
use protobuf::Message;

use crate::realtime::{models::FeedMessage, parse::protos::gtfs::ReplacementStop};

pub(crate) mod protos {
    include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));
}

#[gtfs_realtime_model(protos::gtfs::ReplacementStop)]
pub struct ReplacementStop2 {
    pub travel_time_to_stop: Option<i32>,
    pub stop_id: Option<String>,
}

fn test(stop: protos::gtfs::ReplacementStop) {
    let a: u32 = 2u32.try_into().unwrap();
    let a: ReplacementStop2 = stop.try_into().unwrap();
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use protobuf::Message;

    use crate::realtime::parse::protos;

    #[test]
    fn test_parse_ace() {
        let message = protos::gtfs::FeedMessage::parse_from_reader(
            &mut File::open("test_data/realtime/ace").unwrap(),
        )
        .unwrap();

        let converted_message = crate::realtime::models::FeedMessage::try_from(message).unwrap();

        eprintln!("{:?}", converted_message);
    }
}
