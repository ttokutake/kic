use error::{CliError, Usage, UsageKind};
use super::Command;

use lib::setting::Cron;

#[derive(Debug)]
pub struct End;

impl Command for End {
    fn usage(&self) -> Usage {
        return Usage::new(UsageKind::End);
    }

    #[cfg(unix)]
    fn main(&self) -> Result<(), CliError> {
        let mut cron = try!(Cron::read());

        let current_dir = try!(Cron::current_dir_string());
        cron.delete(current_dir);

        cron.set()
    }
    #[cfg(windows)]
    fn main(&self) -> Result<(), CliError> {
        unimplemented!();
    }
}
