use std::fs;
use std::path::Path;

pub fn initialize() {
    println!("Initialize ...");

    let dir_name = ".keepha";
    if !dir_exists(dir_name) {
        match fs::create_dir(dir_name) {
            Ok(_)    => println!("  OK: Created \"{}\" directory.", dir_name),
            Err(why) => panic!("{:?}", why),
        }
    } else {
        println!("  OK: \"{}\" directory has already exist.", dir_name);
    }
}

fn dir_exists<P: AsRef<Path>>(path: P) -> bool {
    match fs::metadata(path) {
        Ok(m)    => m.is_dir(),
        Err(why) => {
            match why.raw_os_error() {
                Some(2) => false,
                _       => panic!("{:?}", why),
            }
        }
    }
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
