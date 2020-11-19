extern crate prost_build;

fn main() {
    prost_build::compile_protos(&["src/gtfs_data/gtfs-realtime.proto"],
                                &["src/"]).unwrap();
}