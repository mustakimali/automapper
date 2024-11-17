// build.rs
use grpc_build::Builder;

fn main() {
    Builder::new()
        .build_client(false)
        .build_server(false)
        .force(true)
        .out_dir("src/protogen")
        .build("protos")
        .unwrap();
}
