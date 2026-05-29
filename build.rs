#![cfg(feature = "realtime_parse")]

#[cfg(feature = "realtime_mta")]
fn main() {
    protobuf_codegen::Codegen::new()
        // Use `protoc` parser, optional.
        .protoc()
        // All inputs and imports from the inputs must reside in `includes` directories.
        .includes(&["proto"])
        // Inputs must reside in some of include paths.
        .inputs(["proto/gtfs.proto"])
        // Specify output directory relative to Cargo output directory.
        .cargo_out_dir("protos")
        // .out_dir("./generated")
        .run_from_script();
}
