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
        Fetch(_) => {}
        Recent(_) => {}
        StartListening => {}
        StopListening => {}
        Remove(id) => handle_delete_remember_entity(id)
    }
}
