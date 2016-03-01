#[macro_use]
mod lib;

mod command;
mod constant;

use constant::*;
use lib::io::*;
use std::env;

fn main() {
    if let Err(_) = validate_at_first() {
        return;
    }

    let args = env::args()
        .skip(1)
        .collect::<Vec<String>>();

    command::execute(args);
}

fn validate_at_first() -> Result<(), ()> {
    let current_dir = match env::current_dir() {
        Ok(dir)  => dir,
        Err(why) => {
            print_with_error(0, why);
            return Err(());
        }
    };

    match BANNED_DIRS.iter().find(|d| current_dir.ends_with(d)) {
        Some(dir) => {
            print_with_error(0, format!("Cannot run in \"{}\" directory", dir));
            Err(())
        },
        None => Ok(()),
    }
}
