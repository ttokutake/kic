use super::Command;

use constant::*;
use std::fs;

pub struct Destroy;

impl Command for Destroy {
    fn validation(&self) -> bool { true }

    fn help_message(&self) -> &'static str {
        return "destroy!";
    }

    fn main(&self) {
        println!("Destroy ...");

        match fs::remove_dir_all(WORKING_DIR_NAME) {
            Ok(_)    => println!(r#"  OK: Removed "{}" directory."#, WORKING_DIR_NAME),
            Err(why) => println!("  ERROR: {}", why),
        }
    }
}
