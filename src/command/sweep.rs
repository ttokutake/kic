use error::{CliError, Usage, UsageKind};
use super::Command;

use constant::MAIN_DIR;
use lib::fs::*;
use lib::setting::{Ignore, Storage};

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
                "all"    => Some(true),
                "indeed" => Some(false),
                _        => return Err(From::from(self.usage())),
            },
            &None => Some(false),
        };

        let storage = try!(Storage::new(all, indeed).create_box_with_log("sweep"));

        let ignore = try!(Ignore::read());

        let target_files = walk_dir(MAIN_DIR)
            .difference(ignore.files())
            .cloned()
            .collect::<Vec<String>>();
        try!(storage.squeeze_dusts(target_files));

        let all_dirs = dirs_ordered_by_descending_depth(MAIN_DIR);
        try!(storage.squeeze_empty_dirs_only(all_dirs));

        Ok(())
    }
}

impl Sweep {
    pub fn new(option1: Option<String>, option2: Option<String>) -> Self {
        Sweep { option1: option1, option2: option2 }
    }
}
