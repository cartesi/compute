use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_files = ["proto/compute.proto"];
    let proto_dirs = ["proto/"];
    let stub_dirs = Path::new("src/grpc");

    tonic_build::configure()
        .build_server(true)
        .out_dir(stub_dirs)
        .compile(&proto_files, &proto_dirs)
        .unwrap_or_else(|e| panic!("protobuf compilation failed: {}", e));
    
    Ok(())
}