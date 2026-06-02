#![cfg(feature = "realtime_parse")]

#[cfg(feature = "realtime_mta")]
const INPUTS: ([&'static str; 1], [&'static str; 1]) = (["proto/mta"], ["proto/mta/gtfs.proto"]);
#[cfg(not(feature = "realtime_mta"))]
const INPUTS: ([&'static str; 1], [&'static str; 1]) = (["proto"], ["proto/gtfs.proto"]);

fn main() {
    protobuf_codegen::Codegen::new()
        // Use `protoc` parser, optional.
        .protoc()
        // All inputs and imports from the inputs must reside in `includes` directories.
        .includes(&INPUTS.0)
        // Inputs must reside in some of include paths.
        .inputs(INPUTS.1)
        // Specify output directory relative to Cargo output directory.
        .cargo_out_dir("protos")
        // .out_dir("./generated")
        .run_from_script();
}
