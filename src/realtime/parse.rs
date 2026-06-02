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

#[cfg(test)]
mod tests {
    use std::fs::File;

    use protobuf::Message;

    use crate::realtime::parse::protos;

    macro_rules! parse_file {
        ($ident:ident, $file:literal) => {
            let $ident = protos::gtfs::FeedMessage::parse_from_reader(
                &mut File::open(format!("test_data/realtime/{}", $file)).unwrap(),
            )
            .unwrap();

            let _ = crate::realtime::models::FeedMessage::try_from($ident).unwrap();
        };
    }

    #[test]
    fn test_parse_all_static() {
        parse_file!(rtace, "ace");
        parse_file!(rt1234567s, "1234567S");
        parse_file!(rtbdfm, "bdfm");
        parse_file!(rtg, "g");
        parse_file!(rtjz, "jz");
        parse_file!(rtl, "l");
        parse_file!(rtnqrw, "nqrw");
        parse_file!(rtsir, "sir");

        // panic!();
    }
}
