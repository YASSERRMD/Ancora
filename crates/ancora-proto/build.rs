use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR not set"));
    let descriptor_path = out_dir.join("ancora_descriptor.bin");

    // Compile proto files and emit a file descriptor set for pbjson.
    tonic_build::configure()
        .build_server(false)
        .build_client(false)
        .file_descriptor_set_path(&descriptor_path)
        .compile_protos(
            &[
                "proto/ancora.proto",
                "proto/messages.proto",
                "proto/contracts.proto",
                "proto/journal.proto",
                "proto/errors.proto",
            ],
            &["proto"],
        )
        .expect("failed to compile proto files");

    // Decode the file descriptor set and register each file with pbjson-build.
    use prost::Message as _;
    let descriptor_bytes =
        std::fs::read(&descriptor_path).expect("failed to read file descriptor set");
    let descriptor_set = prost_types::FileDescriptorSet::decode(descriptor_bytes.as_slice())
        .expect("failed to decode file descriptor set");

    let mut builder = pbjson_build::Builder::new();
    for file in descriptor_set.file {
        builder.register_file_descriptor(file);
    }
    builder
        .build(&[".ancora"])
        .expect("failed to build pbjson serde impls");
}
