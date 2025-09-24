use std::io::Result;
fn main() -> Result<()> {
    prost_build::Config::new()
        .compile_protos(&["proto/zero_art_proto.proto"], &["proto"])
}
