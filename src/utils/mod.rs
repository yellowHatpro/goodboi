use crate::structs;
use colorize::*;
use rand::prelude::*;
use serde_json::from_str;
use serde_json::Result;
use std::{env, fs, io::Write};
use std::string::ToString;
use clap::Parser;
use crate::structs::CmdArgs;

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

    let args = CmdArgs::parse();

    let current_dir = env::current_dir().unwrap().to_string_lossy().to_string();

    structs::Command {
        method: args.method,
        title: args.title,
        description: args.description,
        cmds: args.cmds,
        pwd: current_dir
    }
}

pub fn get_id() -> String {
    let mut rng = rand::thread_rng();
    let id: u32 = rng.gen_range(1..1000);
    (id + rng.gen_range(1..1000)).to_string()
}

pub fn get_remember_entities() -> Result<Vec<structs::RememberEntity>> {
    let data = fs::read_to_string(DATA_FILE).unwrap();
    let remember_entities: structs::ConfigFile = from_str(&data)?;

    Ok(remember_entities.data)
}

pub fn save_remember_entities(remember_entities: Vec<structs::RememberEntity>) {
    let config_file = structs::ConfigFile { data: remember_entities };
    let json = serde_json::to_string(&config_file).unwrap();

    let mut file = fs::File::create(DATA_FILE).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}
