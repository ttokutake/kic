extern crate toml;

use constant::*;
use error::*;
use lib::fs::*;
use lib::io::*;
use std::collections::BTreeSet;
use std::fmt::Debug;
use std::fs::{self, File};
use std::io::{self, Read, Write};
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


pub fn create_working_dir() -> Result<(), io::Error> {
    create_essential_dir(working_dir())
}

pub fn create_storage_dir() -> Result<(), io::Error> {
    create_essential_dir(storage_dir())
}

fn create_essential_dir(path_to_dir: PathBuf) -> Result<(), io::Error> {
    let file_name = extract_file_name(&path_to_dir).unwrap_or("<UnknownFileName>");
    print_with_tag(0, Tag::Execution, format!("Create \"{}\" directory", file_name));

    if path_to_dir.is_dir() {
        print_with_tag(1, Tag::Notice, "The directory has already exist");
    } else {
        try!(fs::create_dir(&path_to_dir));
        print_with_okay(1);
    }

    Ok(())
}

pub fn create_essential_dir_all(path_to_dir: &PathBuf) -> Result<(), io::Error> {
    let file_name = extract_file_name(path_to_dir).unwrap_or("<UnknownDirectoryName>");
    print_with_tag(0, Tag::Execution, format!("Create \"{}\" directory with its parents", file_name));

    try!(fs::create_dir_all(path_to_dir));
    print_with_okay(1);

    Ok(())
}


pub fn create_config_file<S: AsRef<str>>(contents: S) -> Result<(), io::Error> {
    create_setting_file(config_file(), contents)
}

pub fn create_ignore_file<S: AsRef<str>>(contents: S) -> Result<(), io::Error> {
    create_setting_file(ignore_file(), contents)
}

fn create_setting_file<S: AsRef<str>>(path_to_file: PathBuf, contents: S) -> Result<(), io::Error> {
    let file_name = extract_file_name(&path_to_file).unwrap_or("<UnknownFileName>");

    print_with_tag(0, Tag::Execution, format!("Create \"{}\" file", file_name));
    let mut f = try!(File::create(&path_to_file));
    print_with_okay(1);

    print_with_tag(0, Tag::Execution, "Write contents into the file");
    try!(f.write(contents.as_ref().as_bytes()));
    print_with_okay(1);

    Ok(())
}


pub fn read_config_file() -> Result<toml::Value, CliError> {
    print_with_tag(0, Tag::Execution, format!("Read \"{}\" file as TOML", CONFIG_FILE_NAME));

    let mut f = try!(File::open(config_file()));

    let mut contents = String::new();
    try!(f.read_to_string(&mut contents));

    let result = contents
        .parse()
        .map_err(|e: Vec<_>| e.into_iter().find(|_| true));
    let toml = match result {
        Ok(toml)    => toml,
        Err(option) => match option {
            Some(e) => return Err(From::from(e)),
            None    => return Err(From::from(CannotHappenError)),
        },
    };
    print_with_okay(1);

    Ok(toml)
}

pub fn read_ignore_file() -> Result<BTreeSet<String>, io::Error> {
    print_with_tag(0, Tag::Execution, format!("Read \"{}\" file", IGNORE_FILE_NAME));

    let mut f = try!(File::open(ignore_file()));

    let mut contents = String::new();
    try!(f.read_to_string(&mut contents));
    print_with_okay(1);

    let files = contents
        .lines()
        .map(|l| l.trim().to_string())
        .collect::<BTreeSet<String>>();

    Ok(files)
}


pub fn delete_all_setting_files() -> Result<(), io::Error> {
    delete_dir_all(WORKING_DIR_NAME)
}

pub fn delete_dir_all<P: AsRef<Path> + Debug>(path: P) -> Result<(), io::Error> {
    print_with_tag(0, Tag::Execution, format!("Remove all files and directories under {:?}", path));

    try!(fs::remove_dir_all(path));
    print_with_okay(1);

    Ok(())
}
