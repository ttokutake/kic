use constant::*;
use std::path::Path;

pub fn working_dir_exists() -> bool {
    Path::new(WORKING_DIR_NAME)
        .to_path_buf()
        .is_dir()
}

pub fn config_file_exists() -> bool {
    Path::new(WORKING_DIR_NAME)
        .join(CONFIG_FILE_NAME)
        .is_file()
}

pub fn ignore_file_exists() -> bool {
    Path::new(WORKING_DIR_NAME)
        .join(IGNORE_FILE_NAME)
        .is_file()
}
