use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RememberEntity {
    pub id: String,
    pub title: String,
    pub cmds: Vec<String>,
    pub description: String,
    pub pwd: String,
}

impl fmt::Display for RememberEntity {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "ID: {}", self.id)?;
        writeln!(f, "Title: {}", self.title)?;
        writeln!(f, "Commands: {:?}", self.cmds)?;
        writeln!(f, "Description: {}", self.description)?;
        writeln!(f, "Directory where the command was used: {}", self.pwd)?;
        println!("--------------------------------------------");
        Ok(())
    }
}
