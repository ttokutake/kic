use super::Command;

use constant::*;
use lib::fs_helper::*;
use lib::setting::*;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

pub struct Init;

impl Command for Init {
    fn validation(&self) -> bool { false }

    fn help_message(&self) -> &'static str {
        return "init!";
    }

    fn main(&self) {
        println!("Initialize ...");

        let dir_name    = WORKING_DIR_NAME;
        let path_to_dir = Path::new(dir_name);
        if working_dir_exists() {
            println!("  OK: \"{}\" directory has already exist.", dir_name);
        } else {
            match fs::create_dir(path_to_dir) {
                Ok(_)    => println!("  OK: Created \"{}\" directory.", dir_name),
                Err(why) => panic!("{:?}", why),
            }
        }

        let path_to_config = path_to_dir.clone().join(CONFIG_FILE_NAME);
        create_setting_file(path_to_config, DEFAULT_CONFIG);

        let path_to_ignore = path_to_dir.clone().join(IGNORE_FILE_NAME);
        let ignore_contents = walk_dir(".").iter()
            .fold(String::new(), |c, f| c + &f + "\n");
        create_setting_file(path_to_ignore, ignore_contents);
    }
}

fn create_setting_file<S: AsRef<str>>(path_to_file: PathBuf, contents: S) {
    let file_name = path_to_file.file_name().and_then(|f| f.to_str()).unwrap();
    if path_to_file.is_file() {
        println!("  OK: \"{}\" file has already exist.", file_name);
    } else {
        match File::create(&path_to_file) {
            Ok(mut f) => {
                println!("  OK: Created \"{}\" file.", file_name);
                match f.write(contents.as_ref().as_bytes()) {
                    Ok(_)    => {},
                    Err(why) => panic!("{:?}", why),
                }
            },
            Err(why) => panic!("{:?}", why),
        }
    }
}
