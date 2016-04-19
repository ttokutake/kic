use error::{CliError, Usage, UsageKind};
use super::Command;

use constant::ME;
use lib::setting;

#[derive(Debug)]
pub struct Destroy;

impl Command for Destroy {
    fn usage(&self) -> Usage {
        return Usage::new(UsageKind::Destroy);
    }

    fn main(&self) -> Result<(), CliError> {
        let message = format!("Do you want to clear all files related to \"{}\"?", ME);

        try!(Self::run_after_confirmation(message, || setting::delete_working_dir()));

        super::clean_up_cron()
    }
}
