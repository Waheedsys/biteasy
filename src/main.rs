use core::panic;
use std::error::Error;
use std::{env, process};
use std::env::args;
use std::fs;
use biteasy::{search, search_case_insensitive};

fn main() {
   let args:Vec<String> = env::args().collect();
   print!("arggggggggggggggg: {}\n", args[0]);
   let config= Config::new(&args).unwrap_or_else(|err| {
        print!("problem parsing argument: {err}");
        process::exit(1);
   });
   println!("searching query: {}", config.query);
   println!("in file path: {}", config.file_path);
  if let Err(e) =  run(config){
  println!("Application error {:?}", e);
  process::exit(1);
  }
}

fn run(config : Config)->Result<(), Box<dyn Error>>{
    let content = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &content)
    } else {
        search(&config.query, &content)
    };
    for line in results{ 
        println!("{line}");
    }
    Ok(())
}
struct Config{
    query:String,
    file_path:String,
    pub ignore_case: bool,
}
impl Config{
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();


        Ok(Config{query, file_path, ignore_case})
    }
}

// keeping parameter as reference of string array so it supports both vector and array
// fn parse_config(args:&[String])->Config{
//     let query = args[1].clone();
//     let file_path = args[2].clone();
//     println!("{:?}", &args);
//     println!("{:?}", args);
//     Config {query, file_path}
// }