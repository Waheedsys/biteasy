use std::fmt::Error;
use std::{fs, vec};
use std::time::UNIX_EPOCH;
use std::{fs::OpenOptions, time::SystemTime};
use std::io::Write;

 #[derive(Clone, Debug, PartialEq, Eq)]
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
    let rclone:Record;
    {
    let mut file = OpenOptions::new()
            .read(true)
            .create(true)
            .append(true)
            .open("foo.txt")
            .expect("failed to open foo.txt");

    let r = Record::new(vec![1, 2, 3], vec![4, 5, 6]);
       rclone =  r.clone();
        write_record(&mut file, r);
    }

    let content = fs::read("foo.txt").expect("error while reading foo.txt");
    // key -> 24 ... 27
    //value -> 27...30
    let key_len   = u64::from_be_bytes(content[0..8].try_into().unwrap())  as usize;
    let value_len = u64::from_be_bytes(content[8..16].try_into().unwrap()) as usize;
    let timestamp = u64::from_be_bytes(content[16..24].try_into().unwrap());
   
    let key_start = 24;
    let value_start = key_start+key_len;
    let key = content[key_start..value_start].to_vec();
    let value = content[value_start..value_start+value_len].to_vec();
    println!("{}", rclone.key==key);
     println!("{}", rclone.value==value);
      println!("{}", rclone.timestamp==timestamp);

   
}