use clap::Parser;
use EntityType::*;
use crate::remember::*;
use crate::structs::{EntityType, IRememberArgs};
use crate::utils;
pub fn start() {
    utils::init();
    let args = IRememberArgs::parse();
    match args.entity_type {
        List => list(),
        Save(command) => handle_save_remember_entity(command),
        Sync => {}
        Fetch => {}
        Recent(number_of_lines) => handle_recent_commands(number_of_lines),
        Run(id) =>  handle_run_command(id),
        Remove(id) => handle_delete_remember_entity(id),
        Search(id) => handle_search_command(id)
    }
}
