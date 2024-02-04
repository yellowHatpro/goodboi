use crate::structs::{RecentCommand, RememberEntity, RemoveCommand, SaveCommand};
use crate::utils;
use colorize::*;
use crate::utils::get_pwd;

pub fn handle_save_remember_entity(save_command: SaveCommand) {
    let pwd = get_pwd();
    if save_command.title.len() < 1 || save_command.description.len() < 1 || save_command.cmd.len() < 1 {
        println!("{}", "Please provide some value".red());
        return;
    }
    let mut remember_entities = match utils::get_remember_entities()  {
        Ok(remember_entities) => {remember_entities}
        Err(_) => {vec![]}
    };


    let re = RememberEntity {
        title: save_command.title,
        cmds: vec![save_command.cmd],
        description: save_command.description,
        id: utils::get_id(),
        pwd,
    };
    remember_entities.push(re);
    utils::save_remember_entities(remember_entities);
    println!("{}", "Added command".green());
}

pub fn list() {
    let remember_entities = utils::get_remember_entities().unwrap_or_else(|_| vec![]);

    if remember_entities.len() == 0 {
        println!("{}", "No commands".red());
        return;
    }
    for re in &remember_entities {
        println!("{}", re);
        println!("--------------------------------------------");
    }
}

pub fn handle_recent_commands(rc: RecentCommand){
    utils::read_from_sh_history(rc.number_of_lines)
        .iter()
        .for_each(|x|
            println!("{}", x.clone().green()))
}

pub fn start_listening() {
    //TODO
}
pub fn end_listening() {
    //TODO
}

pub fn handle_delete_remember_entity(id: RemoveCommand) {
    let id = id.id;
    let mut remember_entities = match utils::get_remember_entities() {
        Ok(remember_entities) => {remember_entities}
        Err(_) => {vec![]}
    };

    let id = id.parse::<u32>().unwrap_or(0).to_string();
    let exists = remember_entities.iter().any(|re| re.id == id);
    if !exists {
        println!("{}", "Remembered entity not found".red());
        return;
    }
    remember_entities.retain(|re| re.id != id);
    utils::save_remember_entities(remember_entities);
    println!("{}", "Removed command".green());
}
