pub(crate) mod protos {
    include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));
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

            let $ident = $crate::realtime::models::FeedMessage::try_from($ident).unwrap();
        };
    }

    #[test]
    fn test_parse_all_static() {
        parse_file!(_rtace, "ace");
        parse_file!(_rt1234567s, "1234567S");
        parse_file!(_rtbdfm, "bdfm");
        parse_file!(_rtg, "g");
        parse_file!(_rtjz, "jz");
        parse_file!(_rtl, "l");
        parse_file!(_rtnqrw, "nqrw");
        parse_file!(_rtsir, "sir");
    }
}
