#[macro_use]
mod lib;

mod command;
mod constant;
mod error;

use error::CliError;
use lib::io;

fn main() {
    if let Err(why) = command::execute() {
        match why {
            CliError::Essential(_)    => io::print_with_warning(0, why),
            CliError::RunningPlace(_) => io::print_with_warning(0, why),
            CliError::Usage(u)        => io::print_with_usage(u),
            _                         => io::print_with_error(1, why),
        };
    };
}
