pub mod remember_entity;
use crate::structs::{RecentCommand, RemoveCommand, RunCommand, SaveCommand, SearchCommand};
use crate::utils;
use crate::utils::get_pwd;
use colorize::*;
use remember_entity::RememberEntity;

pub async fn handle_save_remember_entity(save_command: SaveCommand, data_file: String) {
    let pwd = get_pwd();
    if save_command.title.len() < 1
        || save_command.description.len() < 1
        || save_command.cmd.len() < 1
    {
        println!("{}", "Please provide some value".red());
        return;
    }
    let mut remember_entities = match utils::get_remember_entities(&data_file).await {
        Ok(remember_entities) => remember_entities,
        Err(_) => {
            vec![]
        }
    };

    let re = RememberEntity {
        title: save_command.title,
        cmds: vec![save_command.cmd],
        description: save_command.description,
        id: utils::get_id(),
        pwd,
    };
    remember_entities.push(re);
    utils::save_remember_entities(remember_entities, data_file).await;
    println!("{}", "Added command".green());
}

pub async fn list(data_file: String) {
    let remember_entities = match utils::get_remember_entities(&data_file).await {
        Ok(remember_entities) => remember_entities,
        Err(_) => {
            vec![]
        }
    };

    if remember_entities.len() == 0 {
        println!("{}", "No commands".red());
        return;
    }
    for re in &remember_entities {
        println!("{}", re);
    }
}

pub async fn handle_recent_commands(rc: RecentCommand) {
    utils::read_from_sh_history(rc.number_of_lines)
        .await
        .iter()
        .for_each(|x| println!("{}", x.clone().green()))
}

pub async fn handle_run_command(title: RunCommand, data_file: String) {
    match utils::get_remember_entity_by_title(&title.title, data_file).await {
        Ok(re) => {
            if re.title != title.title {
                println!(
                    "{} {} {}",
                    "Did you mean".yellow(),
                    re.title.yellow(),
                    "?".yellow()
                );
                return;
            }
            for cmd in re.cmds {
                utils::execute_command(cmd).await;
            }
        }
        Err(_) => {
            println!("{}", "No command".red())
        }
    }
}

pub async fn handle_search_command(title: SearchCommand, data_file: String) {
    match utils::get_remember_entity_by_title(&title.title, data_file).await {
        Ok(re) => {
            println!("{}", re);
        }
        Err(_) => {
            println!("{}", "No commands".red());
        }
    }
}

pub async fn handle_delete_remember_entity(title: RemoveCommand, data_file: String) {
    let title = title.title;
    let mut remember_entities = match utils::get_remember_entities(&data_file).await {
        Ok(remember_entities) => remember_entities,
        Err(_) => {
            vec![]
        }
    };

    let exists = remember_entities.iter().any(|re| re.title == title);
    if !exists {
        println!("{}", "Remembered entity not found".red());
        return;
    }
    remember_entities.retain(|re| re.title != title);
    utils::save_remember_entities(remember_entities, data_file).await;
    println!("{}", "Removed command".green());
}
