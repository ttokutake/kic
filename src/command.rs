use constants::*;
use std::ffi::OsString;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

pub fn initialize() {
    println!("Initialize ...");

    let dir_name    = ".kic";
    let path_to_dir = Path::new(dir_name);
    if path_to_dir.exists() && path_to_dir.is_dir() {
        println!("  OK: \"{}\" directory has already exist.", dir_name);
    } else {
        match fs::create_dir(path_to_dir) {
            Ok(_)    => println!("  OK: Created \"{}\" directory.", dir_name),
            Err(why) => panic!("{:?}", why),
        }
    }

    let config_file    = "config";
    let path_to_config = path_to_dir.clone().join(config_file);
    create_setting_file(path_to_config, DEFAULT_CONFIG);

    let ignore_file    = "ignore";
    let path_to_ignore = path_to_dir.clone().join(ignore_file);
    let ignore_contents = walk_dir(".", &IGNORED_NAMES).iter()
        .fold(String::new(), |c, f| c + &f + "\n");
    create_setting_file(path_to_ignore, ignore_contents);
}

fn create_setting_file<S: AsRef<str>>(path_to_file: PathBuf, contents: S) {
    let file_name = path_to_file.file_name().and_then(|f| f.to_str()).unwrap();
    if path_to_file.exists() && path_to_file.is_file() {
        println!("  OK: \"{}\" file has already exist.", file_name);
    } else {
        match File::create(&path_to_file) {
            Ok(mut fp) => {
                println!("  OK: Created \"{}\" file.", file_name);
                match fp.write(contents.as_ref().as_bytes()) {
                    Ok(_)    => {},
                    Err(why) => panic!("{:?}", why),
                }
            },
            Err(why) => panic!("{:?}", why),
        }
    }
}

fn walk_dir<P: AsRef<Path>>(path: P, ignored_names: &[&'static str]) -> Vec<String> {
    fn walk_dir<P: AsRef<Path>>(path: P, ignored_names: &[&'static str]) -> Vec<OsString> {
        let dirs = match fs::read_dir(path) {
            Ok(ds)   => ds,
            Err(why) => panic!("{:?}", why),
        };
        let dirs = dirs.filter(|d| d.is_ok())
            .flat_map(|d| {
                let p = d.unwrap().path();
                if ignored_names.iter().any(|name| p.ends_with(name)) {
                    Vec::new()
                } else if p.is_file() {
                    vec![p.into_os_string()]
                } else {
                    walk_dir(&p, ignored_names)
                }
            })
            .collect::<Vec<OsString>>();
        dirs
    }

    walk_dir(path, ignored_names).iter()
        .map(|f| f.clone().into_string())
        .filter(|f| f.is_ok())
        .map(|f| f.unwrap())
        .collect::<Vec<String>>()
}

pub fn set_params() {
    println!("set parameters");
}

pub fn sweep() {
    println!("sweep");
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
    println!("destroy");
}
