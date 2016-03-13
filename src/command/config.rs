use error::*;
use super::Command;

pub struct Config {
    param: Option<String>,
    value: Option<String>,
}

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

impl Config {
    pub fn new(param: Option<String>, value: Option<String>) -> Config {
        Config { param: param, value: value }
    }
}
