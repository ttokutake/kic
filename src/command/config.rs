use error::{CliError, Usage, UsageKind};
use super::Command;

extern crate chrono;
extern crate regex;
extern crate toml;

use self::chrono::NaiveTime;

use error::{ConfigError, ConfigErrorKind};
use lib::config::{self, KeyKind};
use lib::setting;
use std::collections::BTreeMap;
use std::str::FromStr;

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

        // validate value
        let value = match key {
            KeyKind::SweepPeriod => {
                match value {
                    "daily" | "weekly" => value.to_string(),
                    _                  => return Err(From::from(ConfigError::new(ConfigErrorKind::SweepPeriod))),
                }
            },
            KeyKind::SweepTime => {
                match NaiveTime::from_str(format!("{}:00", value).as_ref()) {
                    Ok(_)  => value.to_string(),
                    // should set Err(e) to Error::cause()
                    Err(_) => return Err(From::from(ConfigError::new(ConfigErrorKind::SweepTime))),
                }
            },
            KeyKind::BurnAfter => {
                let (num, unit) = try!(config::Config::interpret(value));
                format!("{} {}", num, unit)
            },
        };

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
