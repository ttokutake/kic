#[macro_use]
mod lib;

mod command;
mod constant;

use constant::*;
use lib::io::*;
use std::env;

fn main() {
    validate_at_first();

    let args = env::args()
        .skip(1)
        .collect::<Vec<String>>();

    command::execute(args);
}

fn validate_at_first() {
    let current_dir = match env::current_dir() {
        Ok(dir)  => dir,
        Err(why) => print_with_error(0, why),
    };

    if let Some(dir) = BANNED_DIRS.iter().find(|d| current_dir.ends_with(d)) {
        print_with_error(0, format!("Cannot run in \"{}\" directory", dir));
    }
}
