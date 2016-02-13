use super::Command;

use constant::*;
use lib::fs::*;
use lib::setting::*;
use std::fs;

pub struct Init;

impl Command for Init {
    fn validation(&self) -> bool { false }

    fn help_message(&self) -> &'static str {
        return "init!";
    }

    fn main(&self) {
        println!("Initialize ...");

        let dir_name    = WORKING_DIR_NAME;
        let path_to_dir = working_dir();
        if working_dir_exists() {
            println!(r#"  OK: "{}" directory has already exist."#, dir_name);
        } else {
            match fs::create_dir(path_to_dir) {
                Ok(_)    => println!(r#"  OK: Created "{}" directory."#, dir_name),
                Err(why) => return println!("  ERROR: {}", why),
            }
        }

        create_config_file(DEFAULT_CONFIG);

        let ignore_contents = walk_dir(".")
            .iter()
            .fold(String::new(), |c, f| c + f + "\n");
        create_ignore_file(ignore_contents);
    }
}
