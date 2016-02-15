use super::Command;

extern crate chrono;

use lib::fs::*;
use lib::setting::*;
use self::chrono::Local;
use std::path::PathBuf;

pub struct Sweep;

impl Command for Sweep {
    fn validation(&self) -> bool { true }

    fn help_message(&self) -> &'static str {
        return "sweep!";
    }

    fn main(&self) {
        println!("Sweep ...");

        let now  = Local::now();
        let date = format!("{}", now.format("%Y-%m-%d"));

        let path_to_dust_box = path_buf![storage_dir(), date];
        create_essential_dir(path_to_dust_box);

        let ignore = read_ignore_file();

        let target_files = walk_dir(".")
            .difference(&ignore)
            .cloned()
            .collect::<Vec<String>>();

        println!("{:?}", target_files);
    }
}
