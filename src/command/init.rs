use error::{CliError, Usage, UsageKind};
use super::Command;

use lib::setting;

pub struct Init;

impl Command for Init {
    fn validation(&self) -> bool { false }

    fn usage(&self) -> Usage {
        return Usage::new(UsageKind::Init);
    }

    fn main(&self) -> Result<(), CliError> {
        try!(setting::create_working_dir());

        try!(setting::create_storage_dir());

        try!(setting::create_initial_config_file());

        try!(setting::create_initial_ignore_file());

        Ok(())
    }
}
