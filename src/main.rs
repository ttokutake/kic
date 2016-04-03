#[macro_use]
mod lib;

mod command;
mod constant;
mod error;

use error::CliError;
use lib::io::*;
use std::process;

fn main() {
    if let Err(why) = command::execute() {
        let error_code = match why {
            CliError::Essential(_) | CliError::RunningPlace(_) => {
                print_with_tag(Tag::Warning, why);
                1
            },
            CliError::Usage(u) => {
                println!("{}", u);
                1
            },
            _ => {
                print_with_tag(Tag::Error, why);
                1
            },
        };
        process::exit(error_code);
    };
}
