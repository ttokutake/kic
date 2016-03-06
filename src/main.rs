#[macro_use]
mod lib;

mod command;
mod constant;
mod error;

use constant::*;
use error::*;
use lib::io::*;
use std::env;

fn main() {
    if let Err(why) = validate_at_first() {
        print_with_error(0, why);
    };

    let args = env::args()
        .skip(1)
        .collect::<Vec<String>>();

    command::execute(args);
}

fn validate_at_first() -> Result<(), CliError> {
    let current_dir = try!(env::current_dir());

    if let Some(d) = BANNED_DIRS.iter().find(|d| current_dir.ends_with(d)) {
        return Err(CliError::RunningPlace(RunningPlaceError { dir: d.to_string() }));
    }

    Ok(())
}
