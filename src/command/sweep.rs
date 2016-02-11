use super::Command;

extern crate chrono;

use constant::*;
use lib::fs_helper::*;
use self::chrono::*;
use std::collections::BTreeSet;
use std::fs::{File, create_dir};
use std::io::Read;
use std::path::Path;

pub struct Sweep;

impl Command for Sweep {
    fn validation(&self) -> bool { true }

    fn help_message(&self) -> &'static str {
        return "sweep!";
    }

    fn main(&self) {
        let now           = Local::now();
        let trash_name    = format!("trash_{}", now.format("%Y-%m-%d"));
        let path_to_trash = Path::new(WORKING_DIR_NAME).join(trash_name);
        if !path_to_trash.is_dir() {
            match create_dir(path_to_trash) {
                Ok(_)    => {},
                Err(why) => panic!("{:?}", why),
            }
        }

        println!("Sweep ...");

        let ignore = read_ignore_file();

        let target_files = walk_dir(".")
            .difference(&ignore)
            .cloned()
            .collect::<Vec<String>>();

        println!("{:?}", target_files);
    }
}

fn read_ignore_file() -> BTreeSet<String> {
    let ignore_file = Path::new(WORKING_DIR_NAME).join(IGNORE_FILE_NAME);
    let mut f       = match File::open(ignore_file) {
        Ok(f)    => f,
        Err(why) => panic!("{:?}", why),
    };
    let mut contents = String::new();
    match f.read_to_string(&mut contents) {
        Ok(_)    => {},
        Err(why) => panic!("{:?}", why),
    }
    contents
        .lines()
        .map(|l| l.trim().to_string())
        .collect::<BTreeSet<String>>()
}
