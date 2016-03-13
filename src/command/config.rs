use error::*;
use super::Command;

pub struct Config;

impl Command for Config {
    fn validation(&self) -> bool { true }

    fn usage(&self) -> Usage {
        return Usage::new(UsageKind::Config);
    }

    fn main(&self) -> Result<(), CliError> {
        unimplemented!();
        Ok(())
    }
}
