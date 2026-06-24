fn main() {
    tonic_build::configure()
        .build_server(false)
        .build_client(false)
        .compile_protos(&["proto/ancora.proto"], &["proto"])
        .expect("failed to compile proto files");
}
