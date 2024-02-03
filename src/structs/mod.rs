use std::fmt;
use std::fmt::Formatter;
use serde::{Deserialize, Serialize};

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
    pub command: String,
    pub argument1: String,
    pub argument2: String,
    pub argument3: String,
    pub pwd: String
}
