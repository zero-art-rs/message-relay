use std::io::Result;
fn main() -> Result<()> {
    prost_build::Config::new().compile_protos(&["src/proto/zero_art_proto.proto"], &["src/proto"])
}
