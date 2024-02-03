use std::fmt;
use std::fmt::Formatter;
use serde::{Deserialize, Serialize};
use clap::Parser;

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
        writeln!(f, "Password: {}", self.pwd)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFile {
    pub data: Vec<RememberEntity>,
}

pub struct Command {
    pub method: String,
    pub title: String,
    pub description: String,
    pub cmds: String,
    pub pwd: String
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