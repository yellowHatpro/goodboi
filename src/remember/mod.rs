use crate::structs::KVPair;
use crate::utils;
use colorize::*;

pub fn add(key: String, value: String) {
    if key.len() < 1 || value.len() < 1 {
        println!("{}", "Please provide some value".red());
        return;
    }
    let mut kvs = utils::get_kvpairs().unwrap();

    let kv = KVPair { key, value };

    kvs.push(kv);
    utils::save_kvpairs(kvs);
    println!("{}", "Added command".green());
}

pub fn list() {
    let kvs = utils::get_kvpairs().unwrap();

    if kvs.len() == 0 {
        println!("{}", "No commands".red());
        return;
    }

    println!("{0: <20} | {1: <50}", "Key", "Value");

    println!("");

    for kv in kvs {
        println!("{0: <20} | {1: <50}", kv.key, kv.value,)
    }
}
