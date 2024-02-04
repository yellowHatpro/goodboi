use std::fmt;
use std::fmt::Formatter;
use serde::{Deserialize, Serialize};
use clap::{Parser, Subcommand, Args};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RememberEntity {
    pub id: String,
    pub title: String,
    pub cmds: Vec<String>,
    pub description: String,
    pub pwd: String
}

impl fmt::Display for RememberEntity {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "ID: {}", self.id)?;
        writeln!(f, "Title: {}", self.title)?;
        writeln!(f, "Commands: {:?}", self.cmds)?;
        writeln!(f, "Description: {}", self.description)?;
        writeln!(f, "Directory where the command was used: {}", self.pwd)?;
        Ok(())
    }
}

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
    Fetch(FetchCommand),
    /// Save recent commands
    Recent(RecentCommand),
    /// Start registering commands
    StartListening,
    /// Stop registering commands
    StopListening,
    /// Remove a remembered entity
    Remove(RemoveCommand),
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
pub struct FetchCommand{

}
#[derive(Debug, Args)]
pub struct RecentCommand{
    /// How many recent commands you want to see
    pub number_of_lines: usize
}
#[derive(Debug, Args)]
pub struct StartListeningCommand{

}
#[derive(Debug, Args)]
pub struct StopListeningCommand{

}
#[derive(Debug, Args)]
pub struct RemoveCommand{
    /// id of the remembered command
 pub id: String
}
