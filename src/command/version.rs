use error::{CliError, Usage, UsageKind};
use super::Command;

use constant::VERSION;

#[derive(Debug)]
pub struct Version;

impl Command for Version {
    fn allow_to_check_current_dir(&self) -> bool { false }
    fn allow_to_check_settings(&self) -> bool { false }

    fn usage(&self) -> Usage {
        return Usage::new(UsageKind::Version);
    }

    fn main(&self) -> Result<(), CliError> {
        println!("{}", VERSION);
        Ok(())
    }
}
