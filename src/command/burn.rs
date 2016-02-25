use super::Command;

extern crate chrono;
extern crate regex;

use lib::io::*;
use lib::setting::*;
use self::chrono::offset::TimeZone;
use self::chrono::{Duration, Local};
use self::regex::Regex;
use std::fs;
use std::path::PathBuf;
use std::result::Result;

pub struct Burn;

impl Command for Burn {
    fn validation(&self) -> bool { true }

    fn help_message(&self) -> &'static str {
        return "burn!";
    }

    fn main(&self) {
        println!("Burn ...\n");

        let moratorium  = read_param_for_burn();
        let target_dirs = search_target_storages(moratorium);
        for dir in &target_dirs {
            delete_dir_all(dir);
        }
    }
}

fn read_param_for_burn() -> Duration {
    let config = read_config_file();
    let key    = "burn.after";

    print_with_tag(0, Tag::Execution, format!("Extract \"{}\" parameter", key));

    let after = match config.lookup(key) {
        Some(v) => v.to_string(),
        None    => panic!(format_with_tag(1, Tag::Error, "The key was not found")),
    };

    let re = match Regex::new(r"(?P<num>\d+)\s*(?P<unit>days?|weeks?)") {
        Ok(re)   => re,
        Err(why) => panic!(why.to_string()),
    };
    let (num, unit) = match re.captures(after.as_ref()).map(|caps| (caps.name("num"), caps.name("unit"))) {
        Some((Some(num), Some(unit))) => (num, unit),
        _                             => panic!(format_with_tag(1, Tag::Error, "The value is invalid"))
    };
    let num = match num.parse::<u32>() {
        Ok(n)    => n,
        Err(why) => panic!(format_with_tag(1, Tag::Error, why)),
    };

    print_with_tag(1, Tag::Okay, "Extracted the parameter");

    match unit {
        "day"  | "days"  => Duration::days(num as i64),
        "week" | "weeks" => Duration::weeks(num as i64),
        _                => panic!("Cannot happen"),
    }
}

fn search_target_storages(moratorium: Duration) -> Vec<PathBuf> {
    print_with_tag(0, Tag::Execution, "Search target dusts");

    let path_to_storage = storage_dir();
    let dirs            = match fs::read_dir(&path_to_storage) {
        Ok(rd)   => rd.filter_map(Result::ok),
        Err(why) => panic!(format_with_tag(1, Tag::Error, why)),
    };

    let today = Local::now();

    print_with_tag(1, Tag::Okay, "Searched target dusts");
    dirs
        .filter_map(|dir| dir
            .file_name()
            .into_string()
            .ok()
        )
        .filter(|date| match Local.datetime_from_str(format!("{} 00:00:00", date).as_ref(), "%Y-%m-%d %H:%M:%S") {
            Ok(created_date) => created_date + moratorium < today,
            Err(_)           => false,
        })
        .map(|dir| path_to_storage.join(dir))
        .collect::<Vec<PathBuf>>()
}
