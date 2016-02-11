use constant::*;
use std::path::PathBuf;

pub fn working_dir_exists() -> bool {
    path_buf![WORKING_DIR_NAME].is_dir()
}

pub fn config_file_exists() -> bool {
    path_buf![WORKING_DIR_NAME, CONFIG_FILE_NAME].is_file()
}

pub fn ignore_file_exists() -> bool {
    path_buf![WORKING_DIR_NAME, IGNORE_FILE_NAME].is_file()
}
