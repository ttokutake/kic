pub mod config;
pub mod ignore;

use constant::{WORKING_DIR_NAME, STORAGE_DIR_NAME};
use std::fs::{self, File};
use std::io::{Error as IoError, Write};
use std::path::{Path, PathBuf};

pub use self::config::{Config, ConfigKey};
pub use self::ignore::Ignore;


pub fn working_dir() -> PathBuf {
    path_buf![WORKING_DIR_NAME]
}

pub fn storage_dir() -> PathBuf {
    path_buf![working_dir(), STORAGE_DIR_NAME]
}


pub fn working_dir_exists() -> bool {
    working_dir().is_dir()
}

pub fn storage_dir_exists() -> bool {
    storage_dir().is_dir()
}


pub fn create_working_dir() -> Result<(), IoError> {
    create_essential_dir(working_dir())
}

pub fn create_storage_dir() -> Result<(), IoError> {
    create_essential_dir(storage_dir())
}

fn create_essential_dir<P: AsRef<Path>>(path_to_dir: P) -> Result<(), IoError> {
    if !path_to_dir.as_ref().is_dir() {
        try!(fs::create_dir(path_to_dir));
    }

    Ok(())
}

pub fn create_essential_dir_all<P: AsRef<Path>>(path_to_dir: P) -> Result<(), IoError> {
    try!(fs::create_dir_all(path_to_dir));

    Ok(())
}


fn create_setting_file<P: AsRef<Path>, U: AsRef<[u8]>>(path_to_file: P, contents: U) -> Result<(), IoError> {
    let mut f = try!(File::create(path_to_file));
    try!(f.write(contents.as_ref()));

    Ok(())
}


pub fn delete_all_setting_files() -> Result<(), IoError> {
    delete_dir_all(working_dir())
}

pub fn delete_dir_all<P: AsRef<Path>>(path: P) -> Result<(), IoError> {
    try!(fs::remove_dir_all(path));

    Ok(())
}
