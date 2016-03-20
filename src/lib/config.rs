extern crate chrono;
extern crate regex;
extern crate toml;

use self::chrono::{Duration, NaiveTime};
use self::regex::Regex;
use self::toml::Value as Toml;

use error::{CannotHappenError, CliError, ConfigError, ConfigErrorKind};
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
        match key.as_ref().trim() {
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

        Ok(toml)
    }

    fn extract(key: &KeyKind) -> Result<String, CliError> {
        let config = try!(Self::read());

        let result = config
            .lookup(key.to_str())
            .ok_or(ConfigError::new(match *key {
                KeyKind::BurnAfter   => ConfigErrorKind::NotFoundBurnAfter,
                KeyKind::SweepPeriod => unimplemented!(),
                KeyKind::SweepTime   => unimplemented!(),
            }));
        let value = try!(result).to_string(); // This to_string() is not documented.

        Ok(value)
    }

    pub fn validate<S: AsRef<str>>(key: &KeyKind, value: S) -> Result<String, CliError> {
        let value = value.as_ref().trim();

        match *key {
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

    pub fn extract_burn_after() -> Result<Duration, CliError> {
        let key = KeyKind::BurnAfter;

        let after       = try!(Config::extract(&key));
        let after       = try!(Self::validate(&key, after));
        let after       = after.split(' ').collect::<Vec<&str>>();
        let (num, unit) = (after[0], after[1]);                    // unsafe!
        let num         = try!(num.parse::<u32>()) as i64;

        let duration = match unit {
            "day"  | "days"  => Duration::days(num),
            "week" | "weeks" => Duration::weeks(num),
            _                => return Err(From::from(CannotHappenError)),
        };

        Ok(duration)
    }
}
