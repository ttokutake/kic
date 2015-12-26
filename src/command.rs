extern crate toml;

use constants::*;
use self::toml::Table;
use std::collections::BTreeSet;
use std::ffi::OsString;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

pub fn initialize() {
    println!("Initialize ...");

    let dir_name    = WORKING_DIR_NAME;
    let path_to_dir = Path::new(dir_name);
    if path_to_dir.exists() && path_to_dir.is_dir() {
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

fn create_setting_file<S: AsRef<str>>(path_to_file: PathBuf, contents: S) {
    let file_name = path_to_file.file_name().and_then(|f| f.to_str()).unwrap();
    if path_to_file.exists() && path_to_file.is_file() {
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

fn walk_dir<P: AsRef<Path>>(path: P) -> BTreeSet<String> {
    fn walk_dir(path: PathBuf) -> Vec<OsString> {
        let paths = match fs::read_dir(path) {
            Ok(rd)   => rd,
            Err(why) => panic!("{:?}", why),
        };
        paths
            .filter(|de| de.is_ok())
            .flat_map(|de| {
                let path = de.unwrap().path();
                if is_hidden(&path) {
                    Vec::new()
                } else if path.is_file() {
                    vec![path.into_os_string()]
                } else {
                    walk_dir(path)
                }
            })
            .collect::<Vec<OsString>>()
    }

    walk_dir(path.as_ref().to_path_buf()).iter()
        .map(|f| f.clone().into_string())
        .filter(|f| f.is_ok())
        .map(|f| f.unwrap())
        .collect::<BTreeSet<String>>()
}

fn is_hidden(path: &PathBuf) -> bool {
    let file_name = path.file_name().and_then(|f| f.to_str());
    file_name.map_or(false, |f| f.starts_with("."))
}

pub fn set_params() {
    println!("set parameters");
}

pub fn sweep() {
    println!("Sweep ...");

    let config = read_config_file();
    let ignore = read_ignore_file();

    let target_files = walk_dir(".")
        .difference(&ignore)
        .cloned()
        .collect::<Vec<String>>();

    println!("{:?}", config);
    println!("{:?}", target_files);
}

fn read_config_file() -> Table {
    let config_file = Path::new(WORKING_DIR_NAME).join(CONFIG_FILE_NAME);
    let mut f       = match File::open(config_file) {
        Ok(f)    => f,
        Err(why) => panic!("{:?}", why),
    };
    let mut config = String::new();
    match f.read_to_string(&mut config) {
        Ok(_)    => {},
        Err(why) => panic!("{:?}", why),
    };

    match toml::Parser::new(&config).parse() {
        Some(toml) => toml,
        None       => panic!("Error occurs on parsing \"{}\".", CONFIG_FILE_NAME),
    }
}

fn read_ignore_file() -> BTreeSet<String> {
    let ignore_file = Path::new(WORKING_DIR_NAME).join(IGNORE_FILE_NAME);
    let mut f       = match File::open(ignore_file) {
        Ok(f)    => f,
        Err(why) => panic!("{:?}", why),
    };
    let mut contents = String::new();
    match f.read_to_string(&mut contents) {
        Ok(_)    => {},
        Err(why) => panic!("{:?}", why),
    }
    contents
        .lines()
        .map(|l| l.trim().to_string())
        .collect::<BTreeSet<String>>()
}

pub fn burn() {
    println!("burn");
}

pub fn register_with_cron() {
    println!("register");
}

pub fn unregister_cron() {
    println!("unregister");
}

pub fn destroy() {
    println!("Destroy ...");

    match fs::remove_dir_all(WORKING_DIR_NAME) {
        Ok(_)    => println!("  OK: Removed \"{}\" directory.", WORKING_DIR_NAME),
        Err(why) => panic!("{:?}", why),
    }
}
