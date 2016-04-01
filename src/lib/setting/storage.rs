extern crate chrono;

use self::chrono::{DateTime, Duration, Local};
use self::chrono::offset::TimeZone;

use constant::STORAGE_DIR_NAME;
use lib::io::*;
use lib::fs::*;
use std::io::Error as IoError;
use std::path::PathBuf;


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
        super::create_essential_dir_all(self.path_to_dust_box())
    }


    pub fn delete_expired_boxes(&self, moratorium: Duration) -> Result<(), IoError> {
        let path_to_storage = Self::path();

        let target_boxes = try!(ls(&path_to_storage))
            .into_iter()
            .filter(|date| match Local.datetime_from_str(format!("{} 00:00:00", date).as_ref(), "%Y-%m-%d %H:%M:%S") {
                Ok(created_date) => created_date + moratorium < self.now,
                Err(_)           => false,
            })
            .map(|dir| path_to_storage.join(dir))
            .collect::<Vec<PathBuf>>();

        for target_box in &target_boxes {
            print_with_tag(Tag::Info, format!("Remove \"{}\" directory", target_box.display()));
            try!(super::delete_dir_all(target_box));
        };

        Ok(())
    }
}
