extern crate chrono;

use self::chrono::{DateTime, Duration, Local};
use self::chrono::offset::TimeZone;

use constant::STORAGE_DIR_NAME;
use error::{CannotHappenError, CliError};
use lib::fs::*;
use lib::io::*;
use std::fs;
use std::io::{Error as IoError, ErrorKind as IoErrorKind};
use std::path::{Path, PathBuf};


pub struct Storage {
    now : DateTime<Local>,
    date: String,
}

impl Storage {
    pub fn path() -> PathBuf {
        path_buf![super::working_dir(), STORAGE_DIR_NAME]
    }

    pub fn exist() -> bool {
        Self::path().is_dir()
    }

    pub fn create() -> Result<(), IoError> {
        print_with_tag(Tag::Info, format!("Create \"{}\" directory", STORAGE_DIR_NAME));

        super::create_essential_dir(Self::path())
    }


    pub fn new() -> Self {
        let now  = Local::now();
        let date = now.format("%Y-%m-%d").to_string();
        Storage { now: now, date: date }
    }


    pub fn path_to_box(&self) -> PathBuf {
        path_buf![Storage::path(), &self.date]
    }

    pub fn path_to_dust_box(&self) -> PathBuf {
        path_buf![self.path_to_box(), "dusts"]
    }

    pub fn create_box(&self) -> Result<(), IoError> {
        print_with_tag(Tag::Info, format!(r#"Create "{}" directory in "{}""#, self.date, STORAGE_DIR_NAME));

        fs::create_dir_all(self.path_to_dust_box())
    }


    pub fn squeeze_dusts<P: AsRef<Path>>(&self, paths_to_dust: Vec<P>) -> Result<(), CliError> {
        let path_to_dust_box = self.path_to_dust_box();

        print_with_tag(Tag::Info, format!("Move dusts to \"{}\"", path_to_dust_box.display()));

        for path_to_dust in &paths_to_dust {
            let path_to_dust = path_to_dust.as_ref();
            let target_file  = try!(path_to_dust.file_name().ok_or(CannotHappenError));
            let target_base  = try!(path_to_dust.parent().ok_or(CannotHappenError));

            let to = path_buf![&path_to_dust_box, target_base];

            let message = format!("  => \"{}\"", path_to_dust.display());
            print_with_tag(Tag::Info, message);

            try!(fs::create_dir_all(&to));

            // forcedly overwrite if the file exists with same name.
            match fs::rename(path_to_dust, path_buf![to, target_file]) {
                Ok(_)  => (),
                Err(e) => match e.kind() {
                    IoErrorKind::PermissionDenied => print_with_tag(Tag::Info, "     Interrupted for permission"),
                    _                             => return Err(From::from(e)),
                },
            };
        }

        Ok(())
    }

    pub fn squeeze_empty_dirs_only<P: AsRef<Path>>(&self, paths_to_dir: Vec<P>) -> Result<(), IoError> {
        let path_to_dust_box = self.path_to_dust_box();

        print_with_tag(Tag::Info, format!("Move empty dirs to \"{}\"", path_to_dust_box.display()));

        for path_to_dir in &paths_to_dir {
            if is_empty_dir(path_to_dir) {
                let path_to_dir = path_to_dir.as_ref();

                let message = format!("  => \"{}\"", path_to_dir.display());
                print_with_tag(Tag::Info, message);

                match fs::remove_dir(path_to_dir) {
                    Ok(_)  => try!(fs::create_dir_all(path_buf![&path_to_dust_box, path_to_dir])),
                    Err(e) => match e.kind() {
                        IoErrorKind::PermissionDenied => print_with_tag(Tag::Info, "     Interrupted for permission"),
                        _                             => return Err(From::from(e)),
                    },
                };
            }
        }

        Ok(())
    }


    pub fn delete_expired_boxes(&self, moratorium: Duration) -> Result<(), IoError> {
        let path_to_storage = Self::path();

        print_with_tag(Tag::Info, "Delete expired dusts");

        let target_boxes = try!(ls(&path_to_storage))
            .into_iter()
            .filter(|date| match Local.datetime_from_str(format!("{} 00:00:00", date).as_ref(), "%Y-%m-%d %H:%M:%S") {
                Ok(created_date) => created_date + moratorium < self.now,
                Err(_)           => false,
            })
            .map(|dir| path_to_storage.join(dir))
            .collect::<Vec<PathBuf>>();

        for target_box in &target_boxes {
            print_with_tag(Tag::Info, format!("  => \"{}\"", target_box.display()));
            try!(fs::remove_dir_all(target_box));
        };

        Ok(())
    }
}
