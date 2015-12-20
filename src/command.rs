use constants::*;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

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

    let config_file    = "config.yml";
    let path_to_config = path_to_dir.clone().join(config_file);
    if path_to_config.exists() && path_to_config.is_file() {
        println!("  OK: \"{}\" file has already exist.", config_file);
    } else {
        match File::create(path_to_config) {
            Ok(mut fp) => {
                println!("  OK: Created \"{}\" file.", config_file);
                write_default_config(&mut fp);
            },
            Err(why) => panic!("{:?}", why),
        }
    }
}

fn write_default_config(fp: &mut File) {
    let contents = DEFAULT_CONFIG.to_string() +
"ignore:
  - Cargo.lock
  - Cargo.toml
  - README.md
  - src/
  - target/
";
    match fp.write(contents.as_bytes()) {
        Ok(_)    => {},
        Err(why) => panic!("{:?}", why),
    }
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
