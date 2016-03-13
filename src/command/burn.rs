use error::*;
use super::Command;

extern crate chrono;
extern crate regex;

use lib::fs::*;
use lib::io::*;
use lib::setting::*;
use self::chrono::offset::TimeZone;
use self::chrono::{Duration, Local};
use self::regex::Regex;
use std::io::Error as IoError;
use std::path::PathBuf;

pub struct Burn;

impl Command for Burn {
    fn validation(&self) -> bool { true }

    fn usage(&self) -> Usage {
        return Usage::new(UsageKind::Burn);
    }

    fn main(&self) -> Result<(), CliError> {
        println!("Burn ...\n");

        let moratorium  = try!(read_param_for_burn());
        let target_dirs = try!(search_target_storages(moratorium));
        for dir in &target_dirs {
            try!(delete_dir_all(dir));
        };

        Ok(())
    }
}

fn read_param_for_burn() -> Result<Duration, CliError> {
    let config = try!(read_config_file());
    let key    = "burn.after";

    print_with_tag(0, Tag::Execution, format!("Extract \"{}\" parameter", key));

    let after = try!(config.lookup(key).ok_or(ConfigError::new(ConfigErrorKind::NotFoundBurnAfter))).to_string();

    let re = try!(Regex::new(r"(?P<num>\d+)\s*(?P<unit>days?|weeks?)"));
    let (num, unit) = match re.captures(after.as_ref()).map(|caps| (caps.name("num"), caps.name("unit"))) {
        Some((Some(num), Some(unit))) => (num, unit),
        Some((None     , Some(_)   )) => return Err(From::from(ConfigError::new(ConfigErrorKind::NumOfBurnAfter))),
        Some((Some(_)  , None      )) => return Err(From::from(ConfigError::new(ConfigErrorKind::UnitOfBurnAfter))),
        _                             => return Err(From::from(ConfigError::new(ConfigErrorKind::BurnAfter))),
    };
    let num = try!(num.parse::<u32>());

    let duration = match unit {
        "day"  | "days"  => Duration::days(num as i64),
        "week" | "weeks" => Duration::weeks(num as i64),
        _                => return Err(From::from(CannotHappenError)),
    };

    print_with_okay(1);
    Ok(duration)
}

fn search_target_storages(moratorium: Duration) -> Result<Vec<PathBuf>, IoError> {
    print_with_tag(0, Tag::Execution, "Search target dusts");

    let path_to_storage = storage_dir();
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

    print_with_okay(1);
    Ok(targets)
}
