use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn initialize() {
    println!("Initialize ...");

    let dir_name    = ".kic";
    let path_to_dir = Path::new(dir_name);
    if dir_exists(path_to_dir) {
        println!("  OK: \"{}\" directory has already exist.", dir_name);
    } else {
        match fs::create_dir(path_to_dir) {
            Ok(_)    => println!("  OK: Created \"{}\" directory.", dir_name),
            Err(why) => panic!("{:?}", why),
        }
    }

    let config_file    = "config";
    let path_to_config = Path::new(dir_name).join(config_file);
    if file_exists(path_to_config.as_path()) {
        println!("  OK: \"{}\" file has already exist.", config_file);
    } else {
        match File::create(path_to_config) {
            Ok(mut fp) => {
                println!("  OK: Created \"{}\" file.", config_file);
                match fp.write(b"Hello, world!!\n") {
                    Ok(_)    => {},
                    Err(why) => panic!("{:?}", why),
                }
            },
            Err(why) => panic!("{:?}", why),
        }
    }
}

fn dir_exists(p: &Path) -> bool {
    p.exists() && p.is_dir()
}

fn file_exists(p: &Path) -> bool {
    p.exists() && p.is_file()
}

pub fn set_params() {
    println!("set parameters");
}

pub fn sweep() {
    println!("sweep");
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
