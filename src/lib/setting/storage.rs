extern crate chrono;

use self::chrono::{DateTime, Local};

use constant::STORAGE_DIR_NAME;
use lib::fs::*;
use std::io::Error as IoError;
use std::path::PathBuf;


pub struct Storage {
    now : DateTime<Local>,
    date: String,
}

impl Storage {
    pub fn path() -> PathBuf {
        path_buf![super::working_dir(), STORAGE_DIR_NAME]
    }

    pub fn exist() -> bool {
        Self::path().is_dir()
    }

    pub fn create() -> Result<(), IoError> {
        super::create_essential_dir(Self::path())
    }


    pub fn new() -> Self {
        let now  = Local::now();
        let date = now.format("%Y-%m-%d").to_string();
        Storage { now: now, date: date }
    }


    pub fn path_to_box(&self) -> PathBuf {
        path_buf![Storage::path(), &self.date]
    }

    pub fn path_to_dust_box(&self) -> PathBuf {
        path_buf![self.path_to_box(), "dusts"]
    }

    pub fn create_box(&self) -> Result<(), IoError> {
        super::create_essential_dir_all(self.path_to_dust_box())
    }

    pub fn get_boxes() -> Result<Vec<String>, IoError> {
        ls(Self::path())
    }
}
