use error::{CliError, Usage, UsageKind};
use super::Command;

use lib::setting::Cron;

#[derive(Debug)]
pub struct Patrol;

impl Command for Patrol {
    fn validation(&self) -> bool { false }

    fn usage(&self) -> Usage {
        return Usage::new(UsageKind::Patrol);
    }

    fn main(&self) -> Result<(), CliError> {
        let cron = try!(Cron::read());
        let cron = try!(cron.discard());
        cron.set()
    }
}
