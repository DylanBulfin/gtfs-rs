#![cfg(feature = "realtime_parse")]

use protobuf_codegen::CodeGen;

#[cfg(not(feature = "realtime_mta"))]
const INPUTS: [&str; 1] = ["gtfs.proto"];
#[cfg(feature = "realtime_mta")]
const INPUTS: [&str; 1] = ["gtfs.proto", "mta-gtfs.proto"];

fn main() {
    CodeGen::new()
        .inputs(INPUTS)
        .include("proto")
        .dependency(protobuf_well_known_types::get_dependency("protobuf_well_known_types"))
        .generate_and_compile()
        .unwrap();
}
