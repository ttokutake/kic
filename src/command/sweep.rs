use error::{CliError, Usage, UsageKind};
use super::Command;

extern crate chrono;

use self::chrono::Local;

use error::CannotHappenError;
use lib::fs::*;
use lib::io::*;
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
        for target in &target_files {
            try!(Self::move_file_to_dust_box(target, &path_to_dust_box));
        }

        let dirs = dirs_ordered_by_descending_depth(".");
        for target in &dirs {
            if is_empty_dir(target) {
                try!(Self::move_dir_to_dust_box(target, &path_to_dust_box));
            }
        }

        Ok(())
    }
}

impl Sweep {
    fn move_file_to_dust_box(target: &String, path_to_dust_box: &PathBuf) -> Result<(), CliError> {
        let target_path = path_buf![target];
        let target_file = try!(target_path.file_name().ok_or(CannotHappenError));
        let target_base = try!(target_path.parent().ok_or(CannotHappenError));
        let to = path_buf![&path_to_dust_box, target_base];

        let message = format!("Move \"{}\" file to \"{}\" directory", target_path.display(), path_to_dust_box.display());
        print_with_tag(Tag::Info, message);

        try!(setting::create_essential_dir_all(&to));

        // forcedly overwrite if the file exists with same name.
        try!(fs::rename(target, path_buf![to, target_file]));

        Ok(())
    }

    fn move_dir_to_dust_box(target: &String, path_to_dust_box: &PathBuf) -> Result<(), IoError> {
        try!(fs::remove_dir(target));
        try!(setting::create_essential_dir_all(&path_buf![path_to_dust_box, target]));

        Ok(())
    }
}
