#![cfg(feature = "realtime_parse")]

#[cfg(feature = "realtime_mta")]
fn main() {
    protobuf_codegen::Codegen::new()
        // Use `protoc` parser, optional.
        .protoc()
        // All inputs and imports from the inputs must reside in `includes` directories.
        .includes(&["proto"])
        // Inputs must reside in some of include paths.
        .input("proto/gtfs.proto")
        .input("proto/mta-gtfs.proto")
        // Specify output directory relative to Cargo output directory.
        .cargo_out_dir("protos")
        .run_from_script();
}
