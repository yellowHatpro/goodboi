use crate::structs;
use colorize::*;
use rand::prelude::*;
use serde_json::from_str;
use serde_json::Result;
use std::{fs, io::Write};

const DATA_FILE: &'static str = "/home/yellowhatpro/.i-remember/data.json";

pub fn init() {
    if !fs::metadata("/home/yellowhatpro/.i-remember").is_ok() {
        fs::create_dir("~/.i-remember")
            .unwrap_or_else(|e| panic!("{} {}", "error due to".red(), e)); //create folder

        //Create file
        let mut file = fs::File::create(DATA_FILE).unwrap();

        //Write to file
        file.write_all(b"{\"data\":[]}").unwrap();

        println!("{} {}", "Created folder and file".green(), DATA_FILE);
    }
    //check if file exists
    else if !fs::metadata(DATA_FILE).is_ok() {
        //Create file
        let mut file = fs::File::create(DATA_FILE).unwrap();

        //Write to file
        file.write_all(b"{\"data\":[]}").unwrap();

        println!("{} {}", "Created file".green(), "DATA_FILE");
    }
}

pub fn get_args() -> structs::Command {
    let args = std::env::args().collect::<Vec<String>>(); // get args and collect them to a vector
    let command = args.get(1).unwrap_or(&"".to_string()).to_string();
    let argument1 = args.get(2).unwrap_or(&"".to_string()).to_string();
    let argument2 = args.get(3).unwrap_or(&"".to_string()).to_string();

    structs::Command {
        command,
        argument1,
        argument2,
    }
}

pub fn get_id() -> u32 {
    let mut rng = rand::thread_rng();
    let id: u32 = rng.gen_range(1..1000);
    id + rng.gen_range(1..1000)
}

pub fn get_kvpairs() -> Result<Vec<structs::KVPair>> {
    let data = fs::read_to_string(DATA_FILE).unwrap();
    let kvpairs: structs::ConfigFile = from_str(&data)?;

    Ok(kvpairs.data)
}

pub fn save_kvpairs(kvpairs: Vec<structs::KVPair>) {
    let config_file = structs::ConfigFile { data: kvpairs };
    let json = serde_json::to_string(&config_file).unwrap();

    let mut file = fs::File::create(DATA_FILE).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}
