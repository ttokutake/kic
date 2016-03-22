use error::{CliError, Usage, UsageKind};
use super::Command;

extern crate toml;

use lib::config;
use lib::setting;

pub struct Config {
    key  : Option<String>,
    value: Option<String>,
}

impl Command for Config {
    fn validation(&self) -> bool { true }

    fn usage(&self) -> Usage {
        return Usage::new(UsageKind::Config);
    }

    fn main(&self) -> Result<(), CliError> {
        let (key, value) = match (self.key.clone(), self.value.clone()) {
            (Some(k), Some(v)) => (k, v),
            _                  => return Err(From::from(self.usage())),
        };

        let key   = try!(config::KeyKind::from(key));
        let value = try!(config::Config::validate(&key, value));

        let config = try!(config::Config::read());
        let config = try!(config.set(&key, value));

        try!(setting::create_config_file(config.to_string()));

        Ok(())
    }
}

impl Config {
    pub fn new(key: Option<String>, value: Option<String>) -> Config {
        Config { key: key, value: value }
    }
}
