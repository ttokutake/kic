use constant::*;
use std::fmt::Display;
use std::path::Path;

pub fn working_dir_exists() -> bool {
    Path::new(WORKING_DIR_NAME)
        .to_path_buf()
        .is_dir()
}
pub fn print_warning_for_working_dir() {
    print_warning(format!("\"{}\" directory", WORKING_DIR_NAME));
}

pub fn config_file_exists() -> bool {
    Path::new(WORKING_DIR_NAME)
        .join(CONFIG_FILE_NAME)
        .is_file()
}
pub fn print_warning_for_config_file() {
    print_warning(format!("\"{}\" file", CONFIG_FILE_NAME));
}

pub fn ignore_file_exists() -> bool {
    Path::new(WORKING_DIR_NAME)
        .join(IGNORE_FILE_NAME)
        .is_file()
}
pub fn print_warning_for_ignore_file() {
    print_warning(format!("\"{}\" file", IGNORE_FILE_NAME));
}

fn print_warning<S: Display>(subject: S) {
    println!(r#"  Warning: {} does not exist. Please type "kic init"."#, subject);
}
