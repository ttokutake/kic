use error::{CliError, Usage, UsageKind};
use super::Command;

use constant::MAIN_DIR;
use lib::fs::*;
use lib::setting::{Config, ConfigKey, Ignore, Storage};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Sweep {
    option1: Option<String>,
    option2: Option<String>,
}

impl Command for Sweep {
    fn usage(&self) -> Usage {
        return Usage::new(UsageKind::Sweep);
    }

    fn main(&self) -> Result<(), CliError> {
        let indeed = [&self.option1, &self.option2]
            .iter()
            .any(|option| match option {
                &&Some(ref o) => o == "indeed",
                &&None        => false,
            });
        let all = match &self.option1 {
            &Some(ref o) => match o.as_ref() {
                "all"    => true,
                "indeed" => false,
                _        => return Err(From::from(self.usage())),
            },
            &None => false,
        };

        let storage = Storage::new("sweep", indeed);
        try!(storage.create_box());

        let config     = try!(Config::read());
        let moratorium = try!(config.get(ConfigKey::SweepMoratorium));
        let moratorium = Config::to_duration(moratorium);

        let ignore = try!(Ignore::read());
        let (ignored_dirs, ignored_files) = ignore.dirs_and_files();

        let target_files = walk_dir(MAIN_DIR)
            .difference(&ignored_files)
            .filter(|f| if all { true } else { !is_recently_accessed(f, &moratorium) })
            .filter(|f| !ignored_dirs.iter().any(|d| f.starts_with(d)))
            .cloned()
            .collect::<Vec<String>>();
        try!(storage.squeeze_dusts(&target_files));

        let phantom_files = if indeed {
            Vec::new()
        } else {
            target_files
                .iter()
                .map(|f| Path::new(f).to_path_buf())
                .collect::<Vec<PathBuf>>()
        };
        let potentially_empty_dirs = potentially_empty_dirs(MAIN_DIR, phantom_files);
        storage.squeeze_empty_dirs(potentially_empty_dirs).map_err(|e| From::from(e))
    }
}

impl Sweep {
    pub fn new(option1: Option<String>, option2: Option<String>) -> Self {
        Sweep { option1: option1, option2: option2 }
    }
}
