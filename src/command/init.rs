use error::{CliError, Usage, UsageKind};
use super::Command;

use lib::setting::{self, Config, Ignore, Storage};

pub struct Init;

impl Command for Init {
    fn validation(&self) -> bool { false }

    fn usage(&self) -> Usage {
        return Usage::new(UsageKind::Init);
    }

    fn main(&self) -> Result<(), CliError> {
        try!(setting::create_working_dir());

        try!(Storage::create());

        if !Config::exist() {
            try!(Config::default().create());
        }

        if !Ignore::exist() {
            try!(Ignore::default().create());
        }

        Ok(())
    }
}
