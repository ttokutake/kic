use constant;
use lib::config::Config;
use lib::fs::*;
use std::collections::BTreeSet;
use std::fs::{self, File};
use std::io::{Error as IoError, Read, Write};
use std::path::{Path, PathBuf};


pub fn working_dir() -> PathBuf {
    path_buf![constant::WORKING_DIR_NAME]
}

pub fn storage_dir() -> PathBuf {
    path_buf![working_dir(), constant::STORAGE_DIR_NAME]
}

pub fn config_file() -> PathBuf {
    path_buf![working_dir(), constant::CONFIG_FILE_NAME]
}

pub fn ignore_file() -> PathBuf {
    path_buf![working_dir(), constant::IGNORE_FILE_NAME]
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


pub fn create_working_dir() -> Result<(), IoError> {
    create_essential_dir(working_dir())
}

pub fn create_storage_dir() -> Result<(), IoError> {
    create_essential_dir(storage_dir())
}

fn create_essential_dir(path_to_dir: PathBuf) -> Result<(), IoError> {
    if !path_to_dir.is_dir() {
        try!(fs::create_dir(&path_to_dir));
    }

    Ok(())
}

pub fn create_essential_dir_all(path_to_dir: &PathBuf) -> Result<(), IoError> {
    try!(fs::create_dir_all(path_to_dir));

    Ok(())
}


pub fn create_initial_config_file() -> Result<(), IoError> {
    create_config_file(Config::default().to_string())
}
pub fn create_config_file<S: AsRef<str>>(contents: S) -> Result<(), IoError> {
    create_setting_file(config_file(), contents)
}

pub fn create_initial_ignore_file() -> Result<(), IoError> {
    let ignore_contents = walk_dir(".")
        .iter()
        .fold(String::new(), |c, f| c + f + "\n");
    create_ignore_file(ignore_contents)
}
pub fn create_ignore_file<S: AsRef<str>>(contents: S) -> Result<(), IoError> {
    create_setting_file(ignore_file(), contents)
}

fn create_setting_file<S: AsRef<str>>(path_to_file: PathBuf, contents: S) -> Result<(), IoError> {
    let mut f = try!(File::create(&path_to_file));
    try!(f.write(contents.as_ref().as_bytes()));

    Ok(())
}


pub fn read_ignore_file() -> Result<BTreeSet<String>, IoError> {
    let mut f = try!(File::open(ignore_file()));

    let mut contents = String::new();
    try!(f.read_to_string(&mut contents));

    let files = contents
        .lines()
        .map(|l| l.trim().to_string())
        .collect::<BTreeSet<String>>();

    Ok(files)
}


pub fn delete_all_setting_files() -> Result<(), IoError> {
    delete_dir_all(constant::WORKING_DIR_NAME)
}

pub fn delete_dir_all<P: AsRef<Path>>(path: P) -> Result<(), IoError> {
    try!(fs::remove_dir_all(path));

    Ok(())
}
