use crate::remember::*;
use crate::utils;

pub fn start() {
    utils::init();
    let args = utils::get_args();
    match args.method.as_str() {
        "start" => start_listening(),
        "end" => end_listening(),
        "add" => add(args.title, args.description, args.cmds, args.pwd),
        "list" => list(),
        "remove" => delete(args.title),
        "q" => std::process::exit(0),
        _ => {}
    }
}
