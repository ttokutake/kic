use error::{CliError, Usage, UsageKind};
use super::Command;

extern crate toml;

use constant::CONFIG_FILE_NAME;
use lib::setting;

pub struct Config {
    command: Option<String>,
    key    : Option<String>,
    value  : Option<String>,
}

impl Command for Config {
    fn validation(&self) -> bool { true }

    fn usage(&self) -> Usage {
        return Usage::new(UsageKind::Config);
    }

    fn main(&self) -> Result<(), CliError> {
        match self.command {
            Some(ref c) => match c.as_ref() {
                "set"  => self.set(),
                "init" => self.init(),
                _      => Err(From::from(self.usage())),
            },
            None => Err(From::from(self.usage())),
        }
    }
}

impl Config {
    pub fn new(command: Option<String>, key: Option<String>, value: Option<String>) -> Config {
        Config { command: command, key: key, value: value }
    }

    fn set(&self) -> Result<(), CliError> {
        let (key, value) = match (&self.key, &self.value) {
            (&Some(ref k), &Some(ref v)) => (k, v),
            _                            => return Err(From::from(self.usage())),
        };

        let key    = try!(setting::ConfigKey::from(key));
        let config = try!(setting::Config::read());
        let config = try!(config.set(key, value));

        try!(config.create());

        Ok(())
    }

    fn init(&self) -> Result<(), CliError> {
        let message = format!("Do you want to initialize \"{}\"?", CONFIG_FILE_NAME);

        Self::run_after_confirmation(message,|| setting::Config::default().create())
    }
}
