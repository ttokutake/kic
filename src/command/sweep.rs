use super::Command;

extern crate chrono;

use lib::fs::*;
use lib::io::*;
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
        let date = now.format("%Y-%m-%d").to_string();

        let path_to_dust_box = path_buf![storage_dir(), date, "dusts"];
        create_essential_dir_all(&path_to_dust_box);

        let ignore = read_ignore_file();

        let target_files = walk_dir(".")
            .difference(&ignore)
            .cloned()
            .collect::<Vec<String>>();
        move_files_to_dust_box(target_files, &path_to_dust_box);

        move_empty_dir_to_dust_box(&path_to_dust_box);
    }
}

fn move_files_to_dust_box(target_files: Vec<String>, path_to_dust_box: &PathBuf) {
    for target in &target_files {
        let target_path = path_buf![&target];
        let target_name = extract_file_name(&target_path);
        let target_base = match target_path.parent() {
            Some(base_name) => base_name,
            None            => panic!(format_with_tag(1, Tag::Error, "Cannot extract base name")),
        };
        let to = path_buf![&path_to_dust_box, target_base];

        create_essential_dir_all(&to);

        print_with_tag(0, Tag::Execution, format!("Move \"{}\" to dust box", target_name));

        // forcedly overwrite if the file exists with same name.
        match fs::rename(target, path_buf![to, target_name]) {
            Ok(_)    => print_with_tag(1, Tag::Okay, "Moved the file to dust box"),
            Err(why) => panic!(format_with_tag(1, Tag::Error, why)),
        }
    }
}

fn move_empty_dir_to_dust_box(path_to_dust_box: &PathBuf) {
    let all_dirs = dirs_ordered_by_descending_depth(".");
    for dir in all_dirs.iter().filter(|d| *d != ".") {
        print_with_tag(0, Tag::Execution, format!("EXECUTION: Remove \"{}\" directory", dir));

        match fs::remove_dir(dir) {
            Ok(_) => {
                print_with_tag(1, Tag::Okay, "Removed the directory");
                create_essential_dir_all(&path_buf![&path_to_dust_box, dir]);
            },
            Err(why) => match why.raw_os_error() {
                Some(39) => print_with_tag(1, Tag::Notice, "The directory is not empty. Cancelled to remove it"),
                _        => panic!(format_with_tag(1, Tag::Error, why)),
            },
        }
    }
}
