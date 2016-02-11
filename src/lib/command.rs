extern crate chrono;

use self::chrono::*;
use constant::*;
use std::collections::BTreeSet;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use super::fs_helper::*;

pub fn set_params() {
    println!("set parameters");
}

pub fn sweep() {
    let now           = Local::now();
    let trash_name    = format!("trash_{}", now.format("%Y-%m-%d"));
    let path_to_trash = Path::new(WORKING_DIR_NAME).join(trash_name);
    if !path_to_trash.is_dir() {
        match fs::create_dir(path_to_trash) {
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

pub fn burn() {
    println!("burn");
}

pub fn register_with_cron() {
    println!("register");
}

pub fn unregister_cron() {
    println!("unregister");
}

pub fn destroy() {
    println!("Destroy ...");

    match fs::remove_dir_all(WORKING_DIR_NAME) {
        Ok(_)    => println!("  OK: Removed \"{}\" directory.", WORKING_DIR_NAME),
        Err(why) => panic!("{:?}", why),
    }
}
