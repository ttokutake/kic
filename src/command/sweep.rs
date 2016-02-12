use super::Command;

extern crate chrono;

use constant::*;
use lib::fs::*;
use self::chrono::Local;
use std::collections::BTreeSet;
use std::fs::{self, File};
use std::io::Read;
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

fn read_ignore_file() -> BTreeSet<String> {
    let ignore_file = path_buf![WORKING_DIR_NAME, IGNORE_FILE_NAME];
    let mut f       = match File::open(ignore_file) {
        Ok(f)    => f,
        Err(why) => panic!("  ERROR: {}", why),
    };
    let mut contents = String::new();
    match f.read_to_string(&mut contents) {
        Ok(_)    => {},
        Err(why) => panic!("  ERROR: {}", why),
    }
    contents
        .lines()
        .map(|l| l.trim().to_string())
        .collect::<BTreeSet<String>>()
}
