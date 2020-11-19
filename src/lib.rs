pub mod gtfs_rt {
    include!(concat!(env!("OUT_DIR"), "/transit_realtime.rs"));
}

pub struct GtfsRtRequest {
    real_time_url: String,
}

impl GtfsRtRequest {
    pub fn new_request(url: &str) -> GtfsRtRequest {
        return GtfsRtRequest {
            real_time_url: String::from(url),
        }
    }

    pub fn do_request(&mut self) -> gtfs_rt::FeedMessage {
//        let req = reqwest::blocking::get(&self.real_time_url).unwrap_or_else(|error| {
//            panic!("Problem with GTFS Real Time get request: {:?}", error);
//        });

        let fm = gtfs_rt::FeedMessage {
            header: gtfs_rt::FeedHeader::default(),
            entity: Vec::new(),
        };

        return fm;

        //self.text = req.text().expect("Error retrieving text from request!");
    }
}

// non public functions to populate gtfs_rt struct fields from string/bytes?
// this could go with the transit_realtime structs
// should the transit_realtime structs be moved somewhere else?

pub fn new_feed_message() -> gtfs_rt::FeedMessage {
    return gtfs_rt::FeedMessage::default();
}

impl gtfs_rt::FeedMessage {
    pub fn new_one() -> gtfs_rt::FeedMessage {
        return gtfs_rt::FeedMessage::default();
    }
}