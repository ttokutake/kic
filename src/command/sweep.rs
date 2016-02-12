use super::Command;

extern crate chrono;

use constant::*;
use lib::fs::*;
use lib::setting::*;
use self::chrono::Local;
use std::fs;
use std::path::PathBuf;

pub struct Sweep;

impl Command for Sweep {
    fn validation(&self) -> bool { true }

    fn help_message(&self) -> &'static str {
        return "sweep!";
    }

    fn main(&self) {
        println!("Sweep ...");

        let now           = Local::now();
        let trash_name    = format!("trash_{}", now.format("%Y-%m-%d"));
        let path_to_trash = path_buf![WORKING_DIR_NAME, &trash_name];
        if !path_to_trash.is_dir() {
            match fs::create_dir(path_to_trash) {
                Ok(_)    => println!(r#"  OK: Created "{}" directory."#, trash_name),
                Err(why) => return println!("  ERROR: {}", why),
            }
        }

        let ignore = read_ignore_file();

        let target_files = walk_dir(".")
            .difference(&ignore)
            .cloned()
            .collect::<Vec<String>>();

        println!("{:?}", target_files);
    }
}
