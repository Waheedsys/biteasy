use std::collections::HashMap;
use std::fmt::Error;
use std::{fs, vec};
use std::time::UNIX_EPOCH;
use std::{fs::OpenOptions, time::SystemTime};
use std::io::Write;
const HEADER_SIZE:usize = 25;
 #[derive(Clone, Debug, PartialEq, Eq)]

 //the actual record
struct Record{
    key:Vec<u8>,
    value:Vec<u8>,
    timestamp: u64,
    tomestone: u8
}

//in-memory index entry
#[derive(PartialEq, Eq, Hash, Debug)]
struct IndexEntry{
    value_start: usize,
    value_len: usize,
    timestamp: u64
}

impl Record{
    fn new(key:Vec<u8>, value:Vec<u8>)->Self{
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).expect("CLOCK").as_secs();
        Record{key, value, timestamp, tomestone:0}
    }
}
// Main write into file function
fn write_record(file:&mut std::fs::File, r:Record) -> std::io::Result<()>{
    let key_len = r.key.len();
    let value_len = r.value.len();
    let tomestone= &[0u8];
    file.write_all(&key_len.to_be_bytes());
    file.write_all(&value_len.to_be_bytes());
    file.write_all(&r.timestamp.to_be_bytes());
    file.write_all(tomestone);
    file.write_all(&r.key);
    file.write_all(&r.value);
    Ok(())
} 
// Main read function
fn read_record(content:&[u8], offset:usize)-> (Record, usize){
    //offset(start point) ->HEADER_SIZE(headers)
    // key -> HEADER_SIZE ... 27
    //value -> 27...30
    //next record will start from offset..29)(30
    let key_len   = u64::from_be_bytes(content[offset..offset+8].try_into().unwrap())  as usize;//0->8
    let value_len = u64::from_be_bytes(content[offset+8..offset+16].try_into().unwrap()) as usize;//8->16
    let timestamp = u64::from_be_bytes(content[offset+16..offset+24].try_into().unwrap());//16->24
    let tomestone = content[offset+24];
    println!("tomestone:{}", tomestone);
    let key_start = offset+HEADER_SIZE;
    let value_start = key_start+key_len;
    let key = content[key_start..value_start].to_vec();
    let value = content[value_start..value_start+value_len].to_vec();
     
    return (Record{key, value, timestamp, tomestone}, value_start+value_len);
}
//function for building index
fn build_index(content: &[u8]) -> HashMap<Vec<u8>, IndexEntry>{
    let mut m: HashMap<Vec<u8>, IndexEntry> = HashMap::new();
    let mut offset = 0;
    while offset < content.len(){
        let (rec, next_offset) = read_record(content, offset);
        let value_start = next_offset - rec.value.len();
        m.insert(rec.key, IndexEntry { value_start, value_len: rec.value.len(), timestamp: rec.timestamp });
        offset = next_offset;
    }
    return m;
}
// GET for retrival of value using index
fn Get(content: &[u8], entry: &IndexEntry) -> Vec<u8>{
   return content[entry.value_start..entry.value_start+entry.value_len].to_vec();
}

//PUT update function -> insert new record into active file then update the index
fn Put(file:&mut std::fs::File, newrecord:Record, index:&mut HashMap<Vec<u8>, IndexEntry>) -> Result<(), Error>{
    let key_len = newrecord.key.len();
    let value_len = newrecord.value.len();
    let tomestone= &[0u8];

    file.write_all(&key_len.to_be_bytes());
    file.write_all(&value_len.to_be_bytes());
    file.write_all(&newrecord.timestamp.to_be_bytes());
    file.write_all(tomestone);
    file.write_all(&newrecord.key);
    file.write_all(&newrecord.value);
    let fileendposition =file.metadata().iter().len(); 
    let value_start = fileendposition + HEADER_SIZE + key_len;
    index.insert(newrecord.key, IndexEntry { value_start, value_len: newrecord.value.len(), timestamp: newrecord.timestamp});
    // let old = index.insert(newrecord.key, ); 
    return Ok(());
}

fn delete(file:&mut std::fs::File, newrecord:Record, index:&mut HashMap<Vec<u8>, IndexEntry>){
    let key_len = newrecord.key.len();
    let value_len = newrecord.value.len();
    let tomestone= &[1u8];

    file.write_all(&key_len.to_be_bytes());
    file.write_all(&value_len.to_be_bytes());
    file.write_all(&newrecord.timestamp.to_be_bytes());
    file.write_all(tomestone);
    file.write_all(&newrecord.key);
    file.write_all(&newrecord.value);

    index.remove(&newrecord.key);

}

fn main(){
    // let mut records = Vec::new();
    {
    let mut file = OpenOptions::new()
            .read(true)
            .create(true)
            .append(true)
            .open("foo.txt")
            .expect("failed to open foo.txt");

    let r = vec![Record::new(vec![1, 2, 3], vec![4, 5, 6]),  Record::new(vec![3, 2, 1], vec![7, 8, 9, 5])];
   
        // rclone =  r.clone();
        for i in r{
          write_record(&mut file, i).expect("issue in write");
        }
    }
    let content = fs::read("foo.txt").expect("error while reading foo.txt");
    
    //build index
    let mut index = build_index(&content);
    // println!("index Map: {:?}", index);
    let entry = index.get(&vec![1u8, 2, 3]).expect("key not found");
    // println!("{:?}", entry);
    let result  = Get(&content, entry);
    
    let mut activefile = OpenOptions::new()
            .create(true)
            .append(true)
            .open("foo.txt")
            .expect("failed to open foo.txt");

    //update
    let newrecord = Record::new(vec![30, 20, 10], vec![70, 80, 90, 50]);
    Put(&mut activefile, newrecord, &mut index);
    // println!("{:?}", result);
    let deletedrecord = Record::new(vec![30, 20, 10], vec![5 ,3 , 1]);
     
   //delete
    delete(&mut activefile, deletedrecord, &mut index);
}