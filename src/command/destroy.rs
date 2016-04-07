use error::{CliError, Usage, UsageKind};
use super::Command;

use constant::ME;
use lib::setting;

#[derive(Debug)]
pub struct Destroy;

impl Command for Destroy {
    fn validation(&self) -> bool { true }

    fn usage(&self) -> Usage {
        return Usage::new(UsageKind::Destroy);
    }

    fn main(&self) -> Result<(), CliError> {
        let message = format!("Do you want to clear all files related to \"{}\"?", ME);

        Self::run_after_confirmation(message, || setting::delete_working_dir())
    }
}
