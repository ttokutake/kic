use error::{CliError, Usage, UsageKind};
use super::Command;

extern crate chrono;

use self::chrono::offset::TimeZone;
use self::chrono::{Duration, Local};

use lib::fs::*;
use lib::io::*;
use lib::setting::{self, Config, ConfigKey, Storage};
use std::io::Error as IoError;
use std::path::PathBuf;

pub struct Burn;

impl Command for Burn {
    fn validation(&self) -> bool { true }

    fn usage(&self) -> Usage {
        return Usage::new(UsageKind::Burn);
    }

    fn main(&self) -> Result<(), CliError> {
        let config         = try!(Config::read());
        let raw_moratorium = try!(config.get(ConfigKey::BurnAfter));
        let moratorium     = try!(Config::to_duration(raw_moratorium));

        let target_dirs = try!(Self::search_target_storages(moratorium));
        for dir in &target_dirs {
            print_with_tag(Tag::Info, format!("Remove \"{}\" directory", dir.display()));
            try!(setting::delete_dir_all(dir));
        };

        Ok(())
    }
}

impl Burn {
    fn search_target_storages(moratorium: Duration) -> Result<Vec<PathBuf>, IoError> {
        let path_to_storage = Storage::path();
        let dirs            = try!(ls(&path_to_storage));
        let today           = Local::now();

        let targets = dirs
            .into_iter()
            .filter(|date| match Local.datetime_from_str(format!("{} 00:00:00", date).as_ref(), "%Y-%m-%d %H:%M:%S") {
                Ok(created_date) => created_date + moratorium < today,
                Err(_)           => false,
            })
            .map(|dir| path_to_storage.join(dir))
            .collect::<Vec<PathBuf>>();

        Ok(targets)
    }
}
