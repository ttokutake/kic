mod config;
mod ignore;
mod storage;

use constant::WORKING_DIR_NAME;
use std::fs::{self, File};
use std::io::{Error as IoError, Write};
use std::path::{Path, PathBuf};

pub use self::config::{Config, ConfigKey};
pub use self::ignore::Ignore;
pub use self::storage::Storage;


pub fn working_dir() -> PathBuf {
    path_buf![WORKING_DIR_NAME]
}

pub fn working_dir_exists() -> bool {
    working_dir().is_dir()
}

pub fn create_working_dir() -> Result<(), IoError> {
    create_essential_dir(working_dir())
}

pub fn delete_working_dir() -> Result<(), IoError> {
    delete_dir_all(working_dir())
}


fn create_essential_dir<P: AsRef<Path>>(path_to_dir: P) -> Result<(), IoError> {
    if !path_to_dir.as_ref().is_dir() {
        try!(fs::create_dir(path_to_dir));
    }

    Ok(())
}

fn create_essential_dir_all<P: AsRef<Path>>(path_to_dir: P) -> Result<(), IoError> {
    try!(fs::create_dir_all(path_to_dir));

    Ok(())
}


fn create_setting_file<P: AsRef<Path>, U: AsRef<[u8]>>(path_to_file: P, contents: U) -> Result<(), IoError> {
    let mut f = try!(File::create(path_to_file));
    try!(f.write(contents.as_ref()));

    Ok(())
}

fn delete_dir_all<P: AsRef<Path>>(path: P) -> Result<(), IoError> {
    try!(fs::remove_dir_all(path));

    Ok(())
}