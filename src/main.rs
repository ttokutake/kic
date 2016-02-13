#[macro_use]
mod lib;

mod command;
mod constant;

use constant::*;
use std::env;

fn main() {
    if let Some(message) = validate_at_first() {
        return println!("{}", message);
    }

    let args = env::args()
        .skip(1)
        .collect::<Vec<String>>();

    command::execute(args);
}

fn validate_at_first() -> Option<String> {
    let current_dir = match env::current_dir() {
        Ok(dir)  => dir,
        Err(why) => return Some(format!("ERROR: {}", why)),
    };

    BANNED_DIRS
        .iter()
        .find(|d| current_dir.ends_with(d))
        .map(|d| format!(r#"ERROR: Cannot run in "{}"."#, d))
}
