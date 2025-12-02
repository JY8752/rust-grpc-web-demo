use prost::Message;
use prost_types::FileDescriptorSet;
use std::fs;

fn main() {
    let descriptor_bytes = fs::read("descriptor.binpb").expect("Failed to read descriptor.binpb");

    let fds =
        FileDescriptorSet::decode(&*descriptor_bytes).expect("Failed to decode FileDescriptorSet");

    tonic_prost_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_fds(fds)
        .expect("Failed to compile protos");
}
