use std::time::UNIX_EPOCH;
use std::{fs::OpenOptions, time::SystemTime};
use std::io::Write;
struct Record{
    key:Vec<u8>,
    value:Vec<u8>,
    timestamp: u64
}

impl Record{
    fn new(key:Vec<u8>, value:Vec<u8>)->Self{
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).expect("CLOCK").as_secs();
        Record{key, value, timestamp}
    }
}

fn write_record(file:&mut std::fs::File, r:Record) -> std::io::Result<()>{
    let key_len = r.key.len();
    let value_len = r.value.len();

    file.write_all(&key_len.to_be_bytes());
     file.write_all(&value_len.to_be_bytes());
    file.write_all(&r.timestamp.to_be_bytes());
    file.write_all(&r.key);
    file.write_all(&r.value);
    Ok(())
} 

fn main(){
    let mut file = OpenOptions::new()
            .read(true)
            .create(true)
            .append(true)
            .open("foo.txt")
            .expect("failed to open foo.txt");
    let r = Record::new(vec![1, 2, 3], vec![4, 5, 6]);
    let ans = write_record(&mut file, r);
   
    println!("{:?}", ans);
}