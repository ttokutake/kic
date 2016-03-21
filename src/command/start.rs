use error::{CliError, Usage, UsageKind};
use super::Command;

pub struct Start;

impl Command for Start {
    fn validation(&self) -> bool { true }

    fn usage(&self) -> Usage {
        return Usage::new(UsageKind::Start);
    }

    fn main(&self) -> Result<(), CliError> {
        unimplemented!();
    }
}
