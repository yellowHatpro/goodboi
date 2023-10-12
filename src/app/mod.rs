use crate::remember::*;
use crate::utils;
use colorize::*;

pub fn start() {
    utils::init();
    let args = utils::get_args();
    match args.command.as_str() {
        "a" => add(args.argument1, args.argument2),
        "l" => list(),
        "q" => std::process::exit(0),
        _ => {}
    }
}
