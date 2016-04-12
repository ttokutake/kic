use error::{CliError, Usage, UsageKind};
use super::Command;

use lib::setting::Cron;

#[derive(Debug)]
pub struct Start;

impl Command for Start {
    fn validation(&self) -> bool { true }

    fn usage(&self) -> Usage {
        return Usage::new(UsageKind::Start);
    }

    fn main(&self) -> Result<(), CliError> {
        let cron = try!(Cron::read());

        let new_cron = try!(cron.update());

        println!("{:?}", new_cron);

        Ok(())
    }
}
