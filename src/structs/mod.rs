use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KVPair {
    pub key: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFile {
    pub data: Vec<KVPair>,
}

pub struct Command {
    pub command: String,
    pub argument1: String,
    pub argument2: String,
}
