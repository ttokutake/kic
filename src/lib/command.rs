use constant::*;
use std::fs;

pub fn destroy() {
    println!("Destroy ...");

    match fs::remove_dir_all(WORKING_DIR_NAME) {
        Ok(_)    => println!("  OK: Removed \"{}\" directory.", WORKING_DIR_NAME),
        Err(why) => panic!("{:?}", why),
    }
}
