extern crate toml;

use constant::*;
use lib::fs::*;
use lib::io::*;
use std::collections::BTreeSet;
use std::fmt::Debug;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};


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
    let file_name = extract_file_name(&path_to_dir).unwrap_or("<UnknownFileName>");
    print_with_tag(0, Tag::Execution, format!("Create \"{}\" directory", file_name));

    if path_to_dir.is_dir() {
        print_with_tag(1, Tag::Notice, "The directory has already exist");
    } else {
        match fs::create_dir(&path_to_dir) {
            Ok(_)    => print_with_okay(1),
            Err(why) => print_with_error(1, why),
        };
    }
}

pub fn create_essential_dir_all(path_to_dir: &PathBuf) {
    let file_name = extract_file_name(path_to_dir).unwrap_or("<UnknownDirectoryName>");
    print_with_tag(0, Tag::Execution, format!("Create \"{}\" directory with its parents", file_name));

    match fs::create_dir_all(path_to_dir) {
        Ok(_)    => print_with_okay(1),
        Err(why) => print_with_error(1, why),
    };
}


pub fn create_config_file<S: AsRef<str>>(contents: S) {
    create_setting_file(config_file(), contents);
}

pub fn create_ignore_file<S: AsRef<str>>(contents: S) {
    create_setting_file(ignore_file(), contents);
}

fn create_setting_file<S: AsRef<str>>(path_to_file: PathBuf, contents: S) {
    let file_name = extract_file_name(&path_to_file).unwrap_or("<UnknownFileName>");
    print_with_tag(0, Tag::Execution, format!("Create \"{}\" file", file_name));

    match File::create(&path_to_file) {
        Ok(mut f) => {
            print_with_okay(1);

            print_with_tag(0, Tag::Execution, "Write contents into the file");
            match f.write(contents.as_ref().as_bytes()) {
                Ok(_)    => print_with_okay(1),
                Err(why) => print_with_error(1, why),
            };
        },
        Err(why) => print_with_error(1, why),
    };
}


pub fn read_config_file() -> toml::Value {
    print_with_tag(0, Tag::Execution, format!("Read \"{}\" file as TOML", CONFIG_FILE_NAME));

    let mut f = match File::open(config_file()) {
        Ok(f)    => f,
        Err(why) => print_with_error(1, why),
    };

    let mut contents = String::new();
    if let Err(why) = f.read_to_string(&mut contents) {
        print_with_error(1, why);
    }

    match contents.parse() {
        Ok(v) => {
            print_with_okay(1);
            v
        },
        Err(why) => print_with_error(1, format!("{:?}", why)),
    }
}

pub fn read_ignore_file() -> BTreeSet<String> {
    print_with_tag(0, Tag::Execution, format!("Read \"{}\" file", IGNORE_FILE_NAME));

    let mut f = match File::open(ignore_file()) {
        Ok(f)    => f,
        Err(why) => print_with_error(1, why),
    };

    let mut contents = String::new();
    match f.read_to_string(&mut contents) {
        Ok(_)    => print_with_okay(1),
        Err(why) => print_with_error(1, why),
    };

    contents
        .lines()
        .map(|l| l.trim().to_string())
        .collect::<BTreeSet<String>>()
}


pub fn delete_all_setting_files() {
    delete_dir_all(WORKING_DIR_NAME);
}

pub fn delete_dir_all<P: AsRef<Path> + Debug>(path: P) {
    print_with_tag(0, Tag::Execution, format!("Remove all files and directories under {:?}", path));

    match fs::remove_dir_all(path) {
        Ok(_)    => print_with_okay(1),
        Err(why) => print_with_error(1, why),
    };
}
