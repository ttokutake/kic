use error::{CliError, Usage, UsageKind};
use super::Command;

extern crate chrono;

use self::chrono::Local;

use error::CannotHappenError;
use lib::fs::*;
use lib::setting;
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
        let now  = Local::now();
        let date = now.format("%Y-%m-%d").to_string();

        let path_to_dust_box = path_buf![setting::storage_dir(), date, "dusts"];
        try!(setting::create_essential_dir_all(&path_to_dust_box));

        let ignore = try!(setting::read_ignore_file());

        let target_files = walk_dir(".")
            .difference(&ignore)
            .cloned()
            .collect::<Vec<String>>();
        try!(Self::move_files_to_dust_box(target_files, &path_to_dust_box));

        try!(Self::move_empty_dir_to_dust_box(&path_to_dust_box));

        Ok(())
    }
}

impl Sweep {
    fn move_files_to_dust_box(target_files: Vec<String>, path_to_dust_box: &PathBuf) -> Result<(), CliError> {
        for target in &target_files {
            let target_path = path_buf![&target];
            let target_file = try!(target_path.file_name().ok_or(CannotHappenError));
            let target_base = try!(target_path.parent().ok_or(CannotHappenError));
            let to = path_buf![&path_to_dust_box, target_base];

            try!(setting::create_essential_dir_all(&to));

            // forcedly overwrite if the file exists with same name.
            try!(fs::rename(target, path_buf![to, target_file]));
        }

        Ok(())
    }

    fn move_empty_dir_to_dust_box(path_to_dust_box: &PathBuf) -> Result<(), IoError> {
        let all_dirs = dirs_ordered_by_descending_depth(".");
        for dir in all_dirs.iter().filter(|d| *d != ".") {
            if is_empty_dir(dir) {
                try!(fs::remove_dir(dir));
                try!(setting::create_essential_dir_all(&path_buf![&path_to_dust_box, dir]));
            }
        }

        Ok(())
    }
}
