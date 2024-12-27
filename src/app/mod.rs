use crate::remember::*;
use crate::structs::{EntityType, IRememberArgs};
use clap::Parser;
use EntityType::*;

pub mod app_entity;

pub async fn start() {
    let app_entity = app_entity::AppEntity::init().await;
    dotenv::dotenv().ok();
    let args = IRememberArgs::parse();
    match args.entity_type {
        List => list(format!("{}/.i-remember/data.json", app_entity.dir)).await,
        Save(command) => {
            handle_save_remember_entity(
                command,
                format!("{}/.i-remember/data.json", app_entity.dir),
            )
            .await
        }
        Recent(number_of_lines) => handle_recent_commands(number_of_lines).await,
        Run(title) => {
            handle_run_command(title, format!("{}/.i-remember/data.json", app_entity.dir)).await
        }
        Remove(title) => {
            handle_delete_remember_entity(
                title,
                format!("{}/.i-remember/data.json", app_entity.dir),
            )
            .await
        }
        Search(title) => {
            handle_search_command(title, format!("{}/.i-remember/data.json", app_entity.dir)).await
        }
    }
}
