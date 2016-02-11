mod command;
mod constant;
mod lib;

use constant::*;
use std::env;

fn main() {
    if let Some(message) = validate_at_first() {
        println!("{}", message);
        return;
    }

    let args = env::args().skip(1).collect::<Vec<String>>();

    command::execute(args);
}

fn validate_at_first() -> Option<String> {
    let current_dir = env::current_dir().unwrap();

    BANNED_DIRS
        .iter()
        .find(|d| current_dir.ends_with(d))
        .map(|d| format!("Cannot run in \"{}\"", d))
}
