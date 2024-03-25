use serde::{Deserialize, Serialize};
use clap::{Parser, Subcommand, Args};
use i_remember_structs::RememberEntity;

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFile {
    pub data: Vec<RememberEntity>,
}

#[derive(Parser, Debug)]
#[command(author="yellowhatpro", version="1", about="A cli tool for cli fans", long_about=None)]
pub struct CmdArgs {
    #[arg(short,long)]
    pub method: String,
    #[arg(short, long, default_value = "")]
    pub title: String,
    #[arg(short,long, default_value = "")]
    pub description: String,
    #[arg(short,long, default_value = "")]
    pub cmds: String,
}

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct IRememberArgs{
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType{
    /// List all remembered entities
    List,
    /// Save new command
    Save(SaveCommand),
    /// Sync commands from remote storage
    Sync,
    /// Fetch commands from remote storage
    Fetch,
    /// Save recent commands
    Recent(RecentCommand),
    /// Run a Remembered Entity
    Run(RunCommand),
    /// Remove a remembered entity
    Remove(RemoveCommand),
    /// Search a remembered entity
    Search(SearchCommand)
}

#[derive(Debug, Args)]
pub struct SaveCommand{
    ///Title of the command
    pub title: String,
    /// Description of the command
    pub description: String,
    /// Command that you want to save
    pub cmd: String
}
#[derive(Debug, Args)]
pub struct RecentCommand{
    /// How many recent commands you want to see
    pub number_of_lines: usize
}
#[derive(Debug, Args)]
pub struct RunCommand {
    ///ID of the remembered entity
    pub title: String
}
#[derive(Debug, Args)]
pub struct RemoveCommand{
    /// id of the remembered command
    pub title: String
}

#[derive(Debug, Args)]
pub struct SearchCommand{
    /// title of the remembered command
    pub title: String
}
