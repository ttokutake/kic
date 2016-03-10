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
use std::io;
use std::path::PathBuf;

pub struct Burn;

impl Command for Burn {
    fn validation(&self) -> bool { true }

    fn usage(&self) -> Usage {
        return Usage { kind: UsageKind::Burn };
    }

    fn main(&self) {
        println!("Burn ...\n");

        let moratorium  = match read_param_for_burn() {
            Ok(m)    => m,
            Err(why) => print_with_error(1, why),
        };
        let target_dirs = match search_target_storages(moratorium) {
            Ok(dirs) => dirs,
            Err(why) => print_with_error(1, why),
        };
        for dir in &target_dirs {
            if let Err(why) = delete_dir_all(dir) {
                print_with_error(1, why);
            };
        }
    }
}

fn read_param_for_burn() -> Result<Duration, CliError> {
    let config = try!(read_config_file());
    let key    = "burn.after";

    print_with_tag(0, Tag::Execution, format!("Extract \"{}\" parameter", key));

    let after = match config.lookup(key) {
        Some(v) => v.to_string(),
        None    => return Err(From::from(ConfigError{ kind: ConfigErrorKind::NotFoundBurnAfter })),
    };

    let re = try!(Regex::new(r"(?P<num>\d+)\s*(?P<unit>days?|weeks?)"));
    let (num, unit) = match re.captures(after.as_ref()).map(|caps| (caps.name("num"), caps.name("unit"))) {
        Some((Some(num), Some(unit))) => (num, unit),
        Some((None     , Some(_)   )) => return Err(From::from(ConfigError{ kind: ConfigErrorKind::NumOfBurnAfter })),
        Some((Some(_)  , None      )) => return Err(From::from(ConfigError{ kind: ConfigErrorKind::UnitOfBurnAfter })),
        _                             => return Err(From::from(ConfigError{ kind: ConfigErrorKind::BurnAfter })),
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

fn search_target_storages(moratorium: Duration) -> Result<Vec<PathBuf>, io::Error> {
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
