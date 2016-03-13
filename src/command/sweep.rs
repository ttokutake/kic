use error::*;
use super::Command;

extern crate chrono;

use lib::fs::*;
use lib::io::*;
use lib::setting::*;
use self::chrono::Local;
use std::fs;
use std::io::Error as IoError;
use std::path::PathBuf;

pub struct Sweep;

impl Command for Sweep {
    fn validation(&self) -> bool { true }

    fn usage(&self) -> Usage {
        return Usage::new(UsageKind::Sweep);
    }

    fn main(&self) -> Result<(), CliError> {
        println!("Sweep ...\n");

        let now  = Local::now();
        let date = now.format("%Y-%m-%d").to_string();

        let path_to_dust_box = path_buf![storage_dir(), date, "dusts"];
        try!(create_essential_dir_all(&path_to_dust_box));

        let ignore = try!(read_ignore_file());

        let target_files = walk_dir(".")
            .difference(&ignore)
            .cloned()
            .collect::<Vec<String>>();
        try!(move_files_to_dust_box(target_files, &path_to_dust_box));

        try!(move_empty_dir_to_dust_box(&path_to_dust_box));

        Ok(())
    }
}

fn move_files_to_dust_box(target_files: Vec<String>, path_to_dust_box: &PathBuf) -> Result<(), CliError> {
    for target in &target_files {
        print_with_tag(0, Tag::Execution, "Analyze the path to file");

        let target_path = path_buf![&target];
        let target_name = try!(extract_file_name(&target_path).ok_or(CannotHappenError));
        let target_base = try!(target_path.parent().ok_or(CannotHappenError));
        let to = path_buf![&path_to_dust_box, target_base];

        print_with_okay(1);

        try!(create_essential_dir_all(&to));

        print_with_tag(0, Tag::Execution, format!("Move \"{}\" to dust box", target_name));

        // forcedly overwrite if the file exists with same name.
        try!(fs::rename(target, path_buf![to, target_name]));
        print_with_okay(1);
    }

    Ok(())
}

fn move_empty_dir_to_dust_box(path_to_dust_box: &PathBuf) -> Result<(), IoError> {
    let all_dirs = dirs_ordered_by_descending_depth(".");
    for dir in all_dirs.iter().filter(|d| *d != ".") {
        print_with_tag(0, Tag::Execution, format!("Remove \"{}\" directory", dir));

        match fs::remove_dir(dir) {
            Ok(_) => {
                print_with_okay(1);
                try!(create_essential_dir_all(&path_buf![&path_to_dust_box, dir]));
            },
            Err(why) => match why.raw_os_error() {
                Some(39) => print_with_tag(1, Tag::Notice, "The directory is not empty. Cancelled to remove it"),
                _        => return Err(why),
            },
        };
    }

    Ok(())
}
