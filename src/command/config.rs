use error::{CliError, Usage, UsageKind};
use super::Command;

extern crate chrono;
extern crate regex;
extern crate toml;

use self::chrono::NaiveTime;

use error::{CannotHappenError, ConfigError, ConfigErrorKind};
use lib::config;
use lib::setting;
use std::collections::BTreeMap;
use std::str::FromStr;

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
        let (param, value) = match (self.param.clone(), self.value.clone()) {
            (Some(p), Some(v)) => (p, v),
            _                  => return Err(From::from(self.usage())),
        };
        let (param, value) = (param.trim(), value.trim());

        let (first, second) = match param {
            p @ "sweep.period" | p @ "sweep.time" | p @ "burn.after" => {
                let params = p.split('.').collect::<Vec<_>>();
                (params[0], params[1])
            },
            _ => return Err(From::from(ConfigError::new(ConfigErrorKind::InvalidParam))),
        };

        // validate value
        let value = match param {
            "sweep.period" => {
                match value {
                    "daily" | "weekly" => value.to_string(),
                    _                  => return Err(From::from(ConfigError::new(ConfigErrorKind::SweepPeriod))),
                }
            },
            "sweep.time" => {
                match NaiveTime::from_str(format!("{}:00", value).as_ref()) {
                    Ok(_)  => value.to_string(),
                    // should set Err(e) to Error::cause()
                    Err(_) => return Err(From::from(ConfigError::new(ConfigErrorKind::SweepTime))),
                }
            },
            "burn.after" => {
                let (num, unit) = try!(config::Config::interpret(value));
                format!("{} {}", num, unit)
            },
            _ => return Err(From::from(CannotHappenError)),
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
    pub fn new(param: Option<String>, value: Option<String>) -> Config {
        Config { param: param, value: value }
    }
}
