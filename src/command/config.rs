use error::{CliError, Usage, UsageKind};
use super::Command;

extern crate toml;

use error::{ConfigError, ConfigErrorKind};
use lib::config::{self, KeyKind};
use lib::setting;
use std::collections::BTreeMap;

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
        let (key, value) = (key.trim(), value.trim());

        let key             = try!(config::KeyKind::from(key));
        let (first, second) = key.to_pair();

        let value = try!(key.validate(value));

        let config     = try!(config::Config::read());
        let mut config = match toml::decode::<BTreeMap<String, BTreeMap<String, String>>>(config) {
            Some(decoded) => decoded,
            None          => return Err(From::from(ConfigError::new(ConfigErrorKind::Something))),
        };

        config
            .get_mut(first)
            .map(|c| c.insert(second.to_string(), value));

        let new_config = toml::encode_str(&config);

        try!(setting::create_config_file(new_config));

        Ok(())
    }
}

impl Config {
    pub fn new(key: Option<String>, value: Option<String>) -> Config {
        Config { key: key, value: value }
    }
}
