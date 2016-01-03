extern crate chrono;

use self::chrono::*;
use constants::*;
use std::collections::BTreeSet;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use lib::fs_helper::*;

pub fn initialize() {
    println!("Initialize ...");

    let dir_name    = WORKING_DIR_NAME;
    let path_to_dir = Path::new(dir_name);
    if path_to_dir.exists() && path_to_dir.is_dir() {
        println!("  OK: \"{}\" directory has already exist.", dir_name);
    } else {
        match fs::create_dir(path_to_dir) {
            Ok(_)    => println!("  OK: Created \"{}\" directory.", dir_name),
            Err(why) => panic!("{:?}", why),
        }
    }

    let path_to_config = path_to_dir.clone().join(CONFIG_FILE_NAME);
    create_setting_file(path_to_config, DEFAULT_CONFIG);

    let path_to_ignore = path_to_dir.clone().join(IGNORE_FILE_NAME);
    let ignore_contents = walk_dir(".").iter()
        .fold(String::new(), |c, f| c + &f + "\n");
    create_setting_file(path_to_ignore, ignore_contents);
}

fn create_setting_file<S: AsRef<str>>(path_to_file: PathBuf, contents: S) {
    let file_name = path_to_file.file_name().and_then(|f| f.to_str()).unwrap();
    if path_to_file.exists() && path_to_file.is_file() {
        println!("  OK: \"{}\" file has already exist.", file_name);
    } else {
        match File::create(&path_to_file) {
            Ok(mut f) => {
                println!("  OK: Created \"{}\" file.", file_name);
                match f.write(contents.as_ref().as_bytes()) {
                    Ok(_)    => {},
                    Err(why) => panic!("{:?}", why),
                }
            },
            Err(why) => panic!("{:?}", why),
        }
    }
}

pub fn set_params() {
    println!("set parameters");
}

pub fn sweep() {
    let now           = Local::now();
    let trash_name    = format!("trash_{}", now.format("%Y-%m-%d"));
    let path_to_trash = Path::new(WORKING_DIR_NAME).join(trash_name);
    if !(path_to_trash.exists() && path_to_trash.is_dir()) {
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

    let path_to_dir = Path::new(WORKING_DIR_NAME).to_path_buf();

    if path_to_dir.exists() {
        match fs::remove_dir_all(WORKING_DIR_NAME) {
            Ok(_)    => println!("  OK: Removed \"{}\" directory.", WORKING_DIR_NAME),
            Err(why) => panic!("{:?}", why),
        }
    } else {
        println!("  OK: \"{}\" does not exist.", WORKING_DIR_NAME);
    }
}
