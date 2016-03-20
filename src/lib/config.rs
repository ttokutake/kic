extern crate chrono;
extern crate regex;
extern crate toml;

use self::chrono::NaiveTime;
use self::regex::Regex;
use self::toml::Value as Toml;

use error::{CannotHappenError, CliError, ConfigError, ConfigErrorKind};
use constant::CONFIG_FILE_NAME;
use lib::io::*;
use lib::setting;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

pub enum KeyKind {
    BurnAfter,
    SweepPeriod,
    SweepTime,
}
impl KeyKind {
    pub fn from<S: AsRef<str>>(key: S) -> Result<KeyKind, ConfigError> {
        match key.as_ref() {
            "burn.after"   => Ok(KeyKind::BurnAfter),
            "sweep.period" => Ok(KeyKind::SweepPeriod),
            "sweep.time"   => Ok(KeyKind::SweepTime),
            _              => Err(ConfigError::new(ConfigErrorKind::InvalidKey)),
        }
    }

    fn to_str(&self) -> &str {
        match *self {
            KeyKind::BurnAfter   => "burn.after",
            KeyKind::SweepPeriod => "sweep.period",
            KeyKind::SweepTime   => "sweep.time",
        }
    }

    pub fn to_pair(&self) -> (&str, &str) {
        let key = self
            .to_str()
            .split('.')
            .collect::<Vec<&str>>();
        (key[0], key[1]) // unsafe!
    }

    pub fn validate(&self, value: &str) -> Result<String, CliError> {
        match *self {
            KeyKind::BurnAfter => {
                let pair = try!(Regex::new(r"^(?P<num>\d+)\s?(?P<unit>days?|weeks?)$"))
                    .captures(value)
                    .map_or((None, None), |caps| (caps.name("num"), caps.name("unit")));
                let (num, unit) = match pair {
                    (Some(num), Some(unit)) => (num, unit),
                    _                       => return Err(From::from(ConfigError::new(ConfigErrorKind::BurnAfter))),
                };
                Ok(format!("{} {}", num, unit))
            },
            KeyKind::SweepPeriod => {
                match value {
                    "daily" | "weekly" => Ok(value.to_string()),
                    _                  => return Err(From::from(ConfigError::new(ConfigErrorKind::SweepPeriod))),
                }
            },
            KeyKind::SweepTime => {
                match NaiveTime::from_str(format!("{}:00", value).as_ref()) {
                    Ok(_)  => Ok(value.to_string()),
                    // should set Err(e) to Error::cause()
                    Err(_) => return Err(From::from(ConfigError::new(ConfigErrorKind::SweepTime))),
                }
            },
        }
    }
}

pub struct Config;

impl Config {
    pub fn default() -> String {
        r#"
[burn]
  after = "2 weeks"

[sweep]
  period = "daily"
  time   = "00:00"
"#
            .to_string()
    }

    pub fn read() -> Result<Toml, CliError> {
        print_with_tag(0, Tag::Execution, format!("Read \"{}\" file as TOML", CONFIG_FILE_NAME));

        let mut f = try!(File::open(setting::config_file()));

        let mut contents = String::new();
        try!(f.read_to_string(&mut contents));

        let result = contents
            .parse()
            .map_err(|e: Vec<_>| e.into_iter().find(|_| true));
        let toml = match result {
            Ok(toml)    => toml,
            Err(option) => match option {
                Some(e) => return Err(From::from(e)),
                None    => return Err(From::from(CannotHappenError)),
            },
        };

        print_with_okay(1);
        Ok(toml)
    }

    pub fn extract(kind: &KeyKind) -> Result<String, CliError> {
        let key = kind.to_str();
        print_with_tag(0, Tag::Execution, format!("Extract \"{}\" parameter", key));

        let config = try!(Self::read());
        let result = config
            .lookup(key)
            .ok_or(ConfigError::new(match *kind {
                KeyKind::BurnAfter   => ConfigErrorKind::NotFoundBurnAfter,
                KeyKind::SweepPeriod => unimplemented!(),
                KeyKind::SweepTime   => unimplemented!(),
            }));
        let value = try!(result).to_string(); // This to_string() is not documented.

        print_with_okay(1);
        Ok(value)
    }

    pub fn interpret(key: &KeyKind, value: String) -> Result<(u32, String), CliError> {
        let value       = try!(key.validate(value.as_ref()));
        let value       = value.split(' ').collect::<Vec<_>>();
        let (num, unit) = (value[0], value[1]); // unsafe!

        let num = try!(num.parse::<u32>());

        Ok((num, unit.to_string()))
    }
}
