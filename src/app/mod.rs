use crate::remember::*;
use crate::utils;

pub fn start() {
    utils::init();
    let args = utils::get_args();
    match args.command.as_str() {
        "start" => start_listening(),
        "end" => end_listening(),
        "add" => add(args.argument1, args.argument2, args.argument3, args.pwd),
        "list" => list(),
        "remove" => delete(args.argument1),
        "q" => std::process::exit(0),
        _ => {}
    }
}
