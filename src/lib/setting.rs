use constant::*;
use lib::fs::*;
use std::collections::BTreeSet;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;


pub fn working_dir() -> PathBuf {
    path_buf![WORKING_DIR_NAME]
}

pub fn storage_dir() -> PathBuf {
    path_buf![working_dir(), STORAGE_DIR_NAME]
}

pub fn config_file() -> PathBuf {
    path_buf![working_dir(), CONFIG_FILE_NAME]
}

pub fn ignore_file() -> PathBuf {
    path_buf![working_dir(), IGNORE_FILE_NAME]
}


pub fn working_dir_exists() -> bool {
    working_dir().is_dir()
}

pub fn storage_dir_exists() -> bool {
    storage_dir().is_dir()
}

pub fn config_file_exists() -> bool {
    config_file().is_file()
}

pub fn ignore_file_exists() -> bool {
    ignore_file().is_file()
}


pub fn create_working_dir() {
    create_essential_dir(working_dir());
}

pub fn create_storage_dir() {
    create_essential_dir(storage_dir());
}

fn create_essential_dir(path_to_dir: PathBuf) {
    let dir_name = extract_file_name(&path_to_dir);

    if path_to_dir.is_dir() {
        println!(r#"NOTICE: "{}" directory has already exist."#, dir_name);
    } else {
        match fs::create_dir(&path_to_dir) {
            Ok(_)    => println!(r#"OK: Created "{}" directory."#, dir_name),
            Err(why) => panic!("ERROR: {}", why),
        }
    }
}

pub fn create_essential_dir_all(path_to_dir: &PathBuf) {
    match fs::create_dir_all(path_to_dir) {
        Ok(_)    => println!("OK: Created {:?} directory with its parents.", path_to_dir),
        Err(why) => panic!("ERROR: {}", why),
    }
}


pub fn create_config_file<S: AsRef<str>>(contents: S) {
    create_setting_file(config_file(), contents);
}

pub fn create_ignore_file<S: AsRef<str>>(contents: S) {
    create_setting_file(ignore_file(), contents);
}

fn create_setting_file<S: AsRef<str>>(path_to_file: PathBuf, contents: S) {
    let file_name = extract_file_name(&path_to_file);

    if path_to_file.is_file() {
        println!(r#"NOTICE: "{}" file has already exist."#, file_name);
    } else {
        match File::create(&path_to_file) {
            Ok(mut f) => {
                println!(r#"OK: Created "{}" file."#, file_name);
                if let Err(why) = f.write(contents.as_ref().as_bytes()) {
                    panic!("ERROR: {}", why);
                }
            },
            Err(why) => panic!("ERROR: {}", why),
        }
    }
}


pub fn read_ignore_file() -> BTreeSet<String> {
    let mut f = match File::open(ignore_file()) {
        Ok(f)    => f,
        Err(why) => panic!("ERROR: {}", why),
    };
    let mut contents = String::new();
    if let Err(why) = f.read_to_string(&mut contents) {
        panic!("ERROR: {}", why);
    }
    contents
        .lines()
        .map(|l| l.trim().to_string())
        .collect::<BTreeSet<String>>()
}


pub fn delete_all_setting_files() {
    match fs::remove_dir_all(WORKING_DIR_NAME) {
        Ok(_)    => println!(r#"OK: Removed "{}" directory."#, WORKING_DIR_NAME),
        Err(why) => panic!("ERROR: {}", why),
    }
}
