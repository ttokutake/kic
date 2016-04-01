use constant::STORAGE_DIR_NAME;
use lib::fs::*;
use std::io::Error as IoError;
use std::path::PathBuf;


pub struct Storage;

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

    pub fn get_boxes() -> Result<Vec<String>, IoError> {
        ls(Self::path())
    }
}
