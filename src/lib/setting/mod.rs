mod config;
mod ignore;
mod storage;

use constant::WORKING_DIR_NAME;
use lib::io::*;
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
    print_with_tag(Tag::Info, format!("Create \"{}\" directory", WORKING_DIR_NAME));

    create_essential_dir(working_dir())
}

pub fn delete_working_dir() -> Result<(), IoError> {
    print_with_tag(Tag::Info, format!("Delete \"{}\" directory", WORKING_DIR_NAME));

    fs::remove_dir_all(working_dir())
}


fn create_essential_dir<P: AsRef<Path>>(path_to_dir: P) -> Result<(), IoError> {
    if !path_to_dir.as_ref().is_dir() {
        try!(fs::create_dir(path_to_dir));
    }

    Ok(())
}

fn create_setting_file<P: AsRef<Path>, U: AsRef<[u8]>>(path_to_file: P, contents: U) -> Result<(), IoError> {
    let mut f = try!(File::create(path_to_file));
    f.write(contents.as_ref()).map(|_| ())
}
