use lib::fs::*;
use std::env;

pub fn exe_name() -> String {
    let full_path = match env::current_exe() {
        Ok(p)    => p,
        Err(why) => panic!("{}", why),
    };
    extract_file_name(&full_path).to_string()
}
