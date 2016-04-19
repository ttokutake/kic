mod command;
mod constant;
mod error;
mod lib;

use error::CliError;
use lib::io::*;
use std::process;

fn main() {
    if let Err(why) = command::execute() {
        let error_code = match why {
            CliError::Essential(_) | CliError::RunningPlace(_) => {
                print_with_tag(Tag::Warning, why);
                if let Err(e) = command::clean_up_cron() {
                    print_with_tag(Tag::Error, e);
                }
                1
            },
            CliError::Usage(u) => {
                echo(u);
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
