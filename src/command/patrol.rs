use error::{CliError, Usage, UsageKind};
use super::Command;

#[derive(Debug)]
pub struct Patrol;

impl Command for Patrol {
    fn validation(&self) -> bool { false }

    fn usage(&self) -> Usage {
        return Usage::new(UsageKind::Patrol);
    }

    fn main(&self) -> Result<(), CliError> {
        unimplemented!();
    }
}
