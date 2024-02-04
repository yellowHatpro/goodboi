use crate::structs;
use colorize::*;
use rand::prelude::*;
use serde_json::from_str;
use serde_json::Result;
use std::{env, fs, io, io::Write};
use std::collections::VecDeque;
use std::fs::File;
use std::io::BufRead;
use std::string::ToString;

const DATA_FILE: &'static str = "/home/yellowhatpro/.i-remember/data.json";

pub fn init() {
    if !fs::metadata("/home/yellowhatpro/.i-remember").is_ok() {
        fs::create_dir("~/.i-remember")
            .unwrap_or_else(|e| panic!("{} {}", "error due to".red(), e)); //create folder

        //Create file
        let mut file = File::create(DATA_FILE).unwrap();

        //Write to file
        file.write_all(b"{\"data\":[]}").unwrap();

        println!("{} {}", "Created folder and file".green(), DATA_FILE);
    }
    //check if file exists
    else if !fs::metadata(DATA_FILE).is_ok() {
        //Create file
        let mut file = File::create(DATA_FILE).unwrap();

        //Write to file
        file.write_all(b"{\"data\":[]}").unwrap();

        println!("{} {}", "Created file".green(), "DATA_FILE");
    }
}

pub fn get_pwd() ->String {
    env::current_dir().unwrap().to_string_lossy().to_string()
}

pub fn get_id() -> String {
    let mut rng = thread_rng();
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

    let mut file = File::create(DATA_FILE).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}

pub fn which_shell() -> String {
    match env::var_os("SHELL") {
        None => { "".to_string() }
        Some(sh) => { sh.to_string_lossy().to_string() }
    }
}

pub fn read_shell_history_file(path: String, n: usize) -> Vec<String> {
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);
    let mut rolling_buffer: VecDeque<String> = VecDeque::with_capacity(n);
    for line in reader.lines() {
        let line = line.unwrap_or_else(|e| e.to_string());
        rolling_buffer.push_back(line);
        if rolling_buffer.len() > n {
            rolling_buffer.pop_front();
        }
    }
    rolling_buffer.into_iter().collect()
}

pub fn read_from_sh_history(number_of_lines: usize) -> Vec<String> {
    match which_shell().as_str() {
        "/usr/bin/zsh" => {
            let history_file_path = env!("HOME").to_string() + "/.zsh_history";
            read_shell_history_file(history_file_path, number_of_lines)
        },
        "/usr/bin/bash" => {
            let history_file_path = env!("HOME").to_string() + "/.bash_history";
            read_shell_history_file(history_file_path, number_of_lines)
        },
        _ => vec![]
    }
}