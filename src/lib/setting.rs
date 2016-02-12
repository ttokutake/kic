use constant::*;
use lib::fs::*;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{Read, Write};
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

pub fn create_setting_file<S: AsRef<str>>(path_to_file: PathBuf, contents: S) {
    let file_name = extract_file_name(&path_to_file);

    if path_to_file.is_file() {
        println!(r#"  OK: "{}" file has already exist."#, file_name);
    } else {
        match File::create(&path_to_file) {
            Ok(mut f) => {
                println!(r#"  OK: Created "{}" file."#, file_name);
                match f.write(contents.as_ref().as_bytes()) {
                    Ok(_)    => {},
                    Err(why) => panic!("  ERROR: {}", why),
                }
            },
            Err(why) => panic!("  ERROR: {}", why),
        }
    }
}

pub fn read_ignore_file() -> BTreeSet<String> {
    let ignore_file = path_buf![WORKING_DIR_NAME, IGNORE_FILE_NAME];
    let mut f       = match File::open(ignore_file) {
        Ok(f)    => f,
        Err(why) => panic!("  ERROR: {}", why),
    };
    let mut contents = String::new();
    match f.read_to_string(&mut contents) {
        Ok(_)    => {},
        Err(why) => panic!("  ERROR: {}", why),
    }
    contents
        .lines()
        .map(|l| l.trim().to_string())
        .collect::<BTreeSet<String>>()
}
