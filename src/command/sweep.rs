use super::Command;

extern crate chrono;

use lib::fs::*;
use lib::setting::*;
use self::chrono::Local;
use std::fs;
use std::path::PathBuf;

pub struct Sweep;

impl Command for Sweep {
    fn validation(&self) -> bool { true }

    fn help_message(&self) -> &'static str {
        return "sweep!";
    }

    fn main(&self) {
        println!("Sweep ...\n");

        let now  = Local::now();
        let date = format!("{}", now.format("%Y-%m-%d"));

        let path_to_dust_box = path_buf![storage_dir(), date, "dust"];
        create_essential_dir_all(&path_to_dust_box);

        let ignore = read_ignore_file();

        let target_files = walk_dir(".")
            .difference(&ignore)
            .cloned()
            .collect::<Vec<String>>();

        for target in target_files.iter() {
            let target_path = path_buf![&target];
            let target_name = extract_file_name(&target_path);
            let target_base = extract_base(&target_path);
            let to          = path_buf![&path_to_dust_box, target_base];

            create_essential_dir_all(&to);

            println!(r#"EXECUTION: Move "{}" to dust box."#, target_name);

            // forcedly overwrite if the file exists with same name.
            match fs::rename(target, path_buf![to, target_name]) {
                Ok(_)    => println!("  OK: Moved the file to dust box."),
                Err(why) => panic!("  ERROR: {}", why),
            }
        }

        // delete all empty directories.
        let all_dirs = enumerate_only_dirs_under(".");
        for dir in all_dirs.iter().filter(|d| *d != ".") {
            println!(r#"EXECUTION: Remove "{}" directory."#, dir);

            match fs::remove_dir(dir) {
                Ok(_) => {
                    println!("  OK: Removed the directory.");
                    create_essential_dir_all(&path_buf![&path_to_dust_box, dir]);
                },
                Err(why) => match why.raw_os_error() {
                    Some(39) => println!("  NOTICE: the directory is not empty."),
                    _        => panic!("  ERROR: {}", why),
                },
            }
        }
    }
}
