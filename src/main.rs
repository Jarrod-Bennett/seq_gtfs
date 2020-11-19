//mod gtfs_translator;

use reqwest;
use prost::Message;
use bytes;

use std::fmt::Error;

const GTFS_RT_URL: &str = "https://gtfsrt.api.translink.com.au/api/realtime/SEQ";
//const GTFS_STATIC_URL: &str = "https://gtfsrt.api.translink.com.au/GTFS/SEQ_GTFS.zip";
//const GTFS_STATIC_PATH: &str = "Path\\to\\SEQ_GTFS.zip";

fn main() {

    let req = reqwest::blocking::get(GTFS_RT_URL).unwrap_or_else(|error| {
        panic!("Problem with GTFS Real Time get request: {:?}", error);
    });

    let bytes = req.bytes().unwrap();
    let mut buf = &bytes[..];

    let f2 = seq_gtfs::gtfs_rt::FeedMessage::decode(&mut buf).unwrap();
    let mut i = 0;
    for entity in f2.entity {
        println!("E {} = {:?}", i, entity);
        i += 1;
    }
    let header = f2.header;
    println!("header = {:?}", header);

}