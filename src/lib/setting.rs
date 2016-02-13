use constant::*;
use lib::fs::*;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;


pub fn working_dir() -> PathBuf {
    path_buf![WORKING_DIR_NAME]
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

pub fn config_file_exists() -> bool {
    config_file().is_file()
}

pub fn ignore_file_exists() -> bool {
    ignore_file().is_file()
}


pub fn create_config_file<S: AsRef<str>>(contents: S) {
    create_setting_file(config_file(), contents);
}

pub fn create_ignore_file<S: AsRef<str>>(contents: S) {
    create_setting_file(ignore_file(), contents);
}

fn create_setting_file<S: AsRef<str>>(path_to_file: PathBuf, contents: S) {
    let file_name = extract_file_name(&path_to_file);

    if path_to_file.is_file() {
        println!(r#"  OK: "{}" file has already exist."#, file_name);
    } else {
        match File::create(&path_to_file) {
            Ok(mut f) => {
                println!(r#"  OK: Created "{}" file."#, file_name);
                if let Err(why) = f.write(contents.as_ref().as_bytes()) {
                    panic!("  ERROR: {}", why);
                }
            },
            Err(why) => panic!("  ERROR: {}", why),
        }
    }
}


pub fn read_ignore_file() -> BTreeSet<String> {
    let mut f       = match File::open(ignore_file()) {
        Ok(f)    => f,
        Err(why) => panic!("  ERROR: {}", why),
    };
    let mut contents = String::new();
    if let Err(why) = f.read_to_string(&mut contents) {
        panic!("  ERROR: {}", why);
    }
    contents
        .lines()
        .map(|l| l.trim().to_string())
        .collect::<BTreeSet<String>>()
}
