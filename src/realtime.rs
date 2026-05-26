mod protos {
    include!("../proto/generated/generated.rs");
}
use protobuf::Parse;

pub struct FeedMessage {
    header: FeedHeader,
    entity: Vec<FeedEntity>,
}

pub struct FeedHeader {

}

pub struct FeedEntity {

}

// TODO to enable extensions, need to write a proc-macro of some kind. Need to have one that will
// allow the user to configure extra fields for specific entities, or add extra entities, to support
// extension fields for specific providers 
//
