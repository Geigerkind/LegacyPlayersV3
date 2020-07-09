use std::time::{UNIX_EPOCH, SystemTime};

pub fn now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs()
}