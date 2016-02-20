#[macro_use]
mod lib;

mod command;
mod constant;

use constant::*;
use lib::io::*;
use std::env;

fn main() {
    if let Err(message) = validate_at_first() {
        return print_with_tag(0, Tag::Error, message);
    }

    let args = env::args()
        .skip(1)
        .collect::<Vec<String>>();

    command::execute(args);
}

fn validate_at_first() -> Result<(), String> {
    let current_dir = match env::current_dir() {
        Ok(dir)  => dir,
        Err(why) => return Err(why.to_string()),
    };

    BANNED_DIRS
        .iter()
        .find(|d| current_dir.ends_with(d))
        .map_or(Ok(()), |dir| Err(format!("Cannot run in \"{}\"", dir)))
}
