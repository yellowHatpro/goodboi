use crate::remember::*;
use crate::structs::{EntityType, IRememberArgs};
use crate::utils;
use clap::Parser;
use EntityType::*;
pub async fn start() {
    utils::init();
    dotenv::dotenv().ok();
    let args = IRememberArgs::parse();
    match args.entity_type {
        List => list(),
        Save(command) => handle_save_remember_entity(command),
        Recent(number_of_lines) => handle_recent_commands(number_of_lines),
        Run(title) => handle_run_command(title),
        Remove(title) => handle_delete_remember_entity(title),
        Search(title) => handle_search_command(title),
    }
}
