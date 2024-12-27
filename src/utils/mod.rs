use crate::remember::remember_entity::RememberEntity;
use crate::structs;
use rand::prelude::*;
use serde_json::from_str;
use serde_json::Result;
use std::collections::VecDeque;
use std::env;
use std::string::ToString;
use strsim::levenshtein;
use tokio::fs;
use tokio::fs::File;
use tokio::io;
use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncWriteExt;

pub fn get_pwd() -> String {
    env::current_dir().unwrap().to_string_lossy().to_string()
}

pub fn get_id() -> String {
    let mut rng = thread_rng();
    let id: u32 = rng.gen_range(1..1000);
    (id + rng.gen_range(1..1000)).to_string()
}

pub async fn get_remember_entities(data_file: &str) -> Result<Vec<RememberEntity>> {
    let data = fs::read_to_string(data_file).await.unwrap();
    let remember_entities: structs::ConfigFile = from_str(&data)?;

    Ok(remember_entities.data)
}

pub async fn get_remember_entity_by_title(
    title: &str,
    data_file: String,
) -> Result<RememberEntity> {
    let remember_entities = match get_remember_entities(&data_file).await {
        Ok(remember_entities) => remember_entities,
        Err(_) => {
            vec![]
        }
    };
    if let Some(re) = find_closest_name(title, remember_entities) {
        Ok(re)
    } else {
        panic!("Could not find the entity")
    }
}

fn find_closest_name(input: &str, names: Vec<RememberEntity>) -> Option<RememberEntity> {
    let mut closest_name = None;
    let mut min_distance = usize::MAX;

    for re in names {
        let distance = levenshtein(input, re.title.as_str());
        if distance < min_distance {
            min_distance = distance;
            closest_name = Some(re.clone());
        }
    }
    closest_name
}

pub async fn execute_command(cmd: String) {
    let output = std::process::Command::new(which_shell())
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("df");
    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).await.unwrap();
    io::stderr().write_all(&output.stderr).await.unwrap();
}

pub async fn save_remember_entities(remember_entities: Vec<RememberEntity>, data_file: String) {
    let config_file = structs::ConfigFile {
        data: remember_entities,
    };
    let json = serde_json::to_string(&config_file).unwrap();

    let mut file = File::create(data_file).await.unwrap();
    file.write_all(json.as_bytes()).await.unwrap();
}

pub fn which_shell() -> String {
    match env::var_os("SHELL") {
        None => "".to_string(),
        Some(sh) => sh.to_string_lossy().to_string(),
    }
}

pub async fn read_shell_history_file(path: String, n: usize) -> Vec<String> {
    let file = File::open(path).await.unwrap();
    let reader = io::BufReader::new(file);
    let mut rolling_buffer: VecDeque<String> = VecDeque::with_capacity(n);
    let mut lines = reader.lines();

    while let Ok(Some(line)) = lines.next_line().await {
        if line.starts_with(':') {
            let parts: Vec<&str> = line.splitn(3, ':').collect();
            if parts.len() >= 3 {
                let command_part = parts[2];
                if let Some(command) = command_part.split_once(';') {
                    let formatted = command.1.trim().to_string();
                    rolling_buffer.push_back(formatted);
                    if rolling_buffer.len() > n {
                        rolling_buffer.pop_front();
                    }
                    continue;
                }
            }
        }

        // Fallback for unparseable lines
        rolling_buffer.push_back(line);
        if rolling_buffer.len() > n {
            rolling_buffer.pop_front();
        }
    }

    rolling_buffer.into_iter().collect()
}

pub async fn read_from_sh_history(number_of_lines: usize) -> Vec<String> {
    match which_shell().as_str() {
        "/usr/bin/zsh" | "/bin/zsh" => {
            let history_file_path = env!("HOME").to_string() + "/.zsh_history";
            read_shell_history_file(history_file_path, number_of_lines).await
        }
        "/usr/bin/bash" | "/bin/bash" => {
            let history_file_path = env!("HOME").to_string() + "/.bash_history";
            read_shell_history_file(history_file_path, number_of_lines).await
        }
        _ => vec![],
    }
}
