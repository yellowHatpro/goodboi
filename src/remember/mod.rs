use crate::structs::RememberEntity;
use crate::utils;
use colorize::*;

pub fn add(title: String, description: String, cmd: String, pwd: String) {
    if title.len() < 1 || description.len() < 1 || cmd.len() < 1 {
        println!("{}", "Please provide some value".red());
        return;
    }
    let mut remember_entities = match utils::get_remember_entities()  {
        Ok(remember_entities) => {remember_entities}
        Err(_) => {vec![]}
    };


    let re = RememberEntity {
        title,
        cmds: vec![cmd],
        description,
        id: utils::get_id(),
        pwd,
    };
    remember_entities.push(re);
    utils::save_remember_entities(remember_entities);
    println!("{}", "Added command".green());
}

pub fn list() {
    let remember_entities = utils::get_remember_entities().unwrap();

    if remember_entities.len() == 0 {
        println!("{}", "No commands".red());
        return;
    }
    for re in &remember_entities {
        println!("{}", re);
        println!("--------------------------------------------");
    }
}

pub fn start_listening() {
    //TODO
}
pub fn end_listening() {
    //TODO
}

pub fn delete(id: String) {
    let mut remember_entities = match utils::get_remember_entities() {
        Ok(remember_entities) => {remember_entities}
        Err(_) => {vec![]}
    };

    let id = id.parse::<u32>().unwrap_or(0).to_string();
    let exists = remember_entities.iter().any(|re| re.id == id);
    if !exists {
        println!("{}", "Key Value pair not found".red());
        return;
    }
    remember_entities.retain(|re| re.id != id);
    utils::save_remember_entities(remember_entities);
    println!("{}", "Removed command".green());
}
