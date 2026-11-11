use std::{fs::OpenOptions, time::SystemTime};
use std::io::Write;
use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};
struct Record{
    key:Vec<u8>,
    value:Vec<u8>,
    timestamp: u64
}

impl Record{
    fn new()->Record{
        let timestamp = SystemTime::now()
        return Record{key, value, timestamp}
    }
}

fn main(){
    let mut file = OpenOptions::new()
            .read(true)
            .create(true)
            .append(true)
            .open("foo.txt")
            .expect("failed to open foo.txt");
    
}