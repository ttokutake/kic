use error::{CliError, Usage, UsageKind};
use super::Command;

use constant::MAIN_DIR;
use lib::fs::*;
use lib::setting::{Ignore, Storage};

#[derive(Debug)]
pub struct Sweep;

impl Command for Sweep {
    fn usage(&self) -> Usage {
        return Usage::new(UsageKind::Sweep);
    }

    fn main(&self) -> Result<(), CliError> {
        let storage = try!(Storage::new(false).create_box_with_log("sweep"));

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
