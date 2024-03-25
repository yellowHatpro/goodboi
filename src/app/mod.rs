use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use EntityType::*;
use crate::remember::*;
use crate::structs::{EntityType, IRememberArgs};
use crate::utils;
pub async fn start() {
    utils::init();
    dotenv::dotenv().ok();
    let args = IRememberArgs::parse();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres@localhost/iremember");

    match args.entity_type {
        List => list(),
        Save(command) => handle_save_remember_entity(command),
        Sync => handle_sync().await,
        Fetch => handle_fetch(),
        Recent(number_of_lines) => handle_recent_commands(number_of_lines),
        Run(title) =>  handle_run_command(title),
        Remove(title) => handle_delete_remember_entity(title),
        Search(title) => handle_search_command(title)
    }
}
