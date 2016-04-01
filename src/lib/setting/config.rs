extern crate chrono;
extern crate regex;
extern crate toml;

use self::chrono::{Duration, NaiveTime};
use self::regex::Regex;
use self::toml::Value as Toml;

use constant::CONFIG_FILE_NAME;
use error::{CannotHappenError, CliError, ConfigError, ConfigErrorKind};
use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{Error as IoError, Read};
use std::path::PathBuf;
use std::str::FromStr;


pub enum ConfigKey {
    BurnAfter,
    SweepPeriod,
    SweepTime,
}

impl ConfigKey {
    pub fn from<S: AsRef<str>>(key: S) -> Result<ConfigKey, ConfigError> {
        match key.as_ref().trim() {
            "burn.after"   => Ok(ConfigKey::BurnAfter),
            "sweep.period" => Ok(ConfigKey::SweepPeriod),
            "sweep.time"   => Ok(ConfigKey::SweepTime),
            _              => Err(ConfigError::new(ConfigErrorKind::InvalidKey)),
        }
    }

    fn to_str(&self) -> &str {
        match *self {
            ConfigKey::BurnAfter   => "burn.after",
            ConfigKey::SweepPeriod => "sweep.period",
            ConfigKey::SweepTime   => "sweep.time",
        }
    }

    fn to_pair(&self) -> (&str, &str) {
        let key = self
            .to_str()
            .split('.')
            .collect::<Vec<&str>>();
        (key[0], key[1]) // unsafe!
    }
}


pub struct Config {
    toml: Toml,
}

impl Config {
    fn path() -> PathBuf {
        path_buf![super::working_dir(), CONFIG_FILE_NAME]
    }

    pub fn exist() -> bool {
        Self::path().is_file()
    }


    fn to_string(&self) -> String {
        toml::encode_str(&self.toml)
    }

    pub fn create(&self) -> Result<(), IoError> {
        super::create_setting_file(Self::path(), self.to_string())
    }


    fn _new(toml: Toml) -> Self {
        Config { toml: toml }
    }

    pub fn default() -> Self {
        let mut config = BTreeMap::new();
        Self::insert_deeply(&mut config, &ConfigKey::BurnAfter  , "2 weeks".to_string());
        Self::insert_deeply(&mut config, &ConfigKey::SweepPeriod, "daily"  .to_string());
        Self::insert_deeply(&mut config, &ConfigKey::SweepTime  , "00:00"  .to_string());

        Self::_new(toml::encode(&config))
    }

    pub fn read() -> Result<Self, CliError> {
        let mut f = try!(File::open(Self::path()));

        let mut contents = String::new();
        try!(f.read_to_string(&mut contents));

        let result = contents
            .parse()
            .map_err(|e: Vec<_>| e.into_iter().next());
        let toml = match result {
            Ok(toml)    => toml,
            Err(option) => match option {
                Some(e) => return Err(From::from(e)),
                None    => return Err(From::from(CannotHappenError)),
            },
        };

        Ok(Self::_new(toml))
    }


    fn get<CK: Borrow<ConfigKey>>(key: CK) -> Result<String, CliError> {
        let key    = key.borrow();
        let config = try!(Self::read()).toml;

        let result = config
            .lookup(key.to_str())
            .ok_or(ConfigError::new(match *key {
                ConfigKey::BurnAfter   => ConfigErrorKind::NotFoundBurnAfter,
                ConfigKey::SweepPeriod => unimplemented!(),
                ConfigKey::SweepTime   => unimplemented!(),
            }));
        let value = try!(result);

        value
            .as_str()
            .map(|v| v.to_string())
            .ok_or(From::from(ConfigError::new(ConfigErrorKind::NonStringValue)))
    }

    pub fn extract_burn_after() -> Result<Duration, CliError> {
        let key = ConfigKey::BurnAfter;

        let after       = try!(Self::get(&key));
        let after       = try!(Self::validate(key, after));
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


    pub fn set<CK: Borrow<ConfigKey>>(mut self, key: CK, value: String) -> Result<Self, ConfigError> {
        let mut config = match toml::decode::<BTreeMap<String, BTreeMap<String, String>>>(self.toml) {
            Some(decoded) => decoded,
            None          => return Err(ConfigError::new(ConfigErrorKind::Something)),
        };

        Self::insert_deeply(&mut config, key.borrow(), value);

        self.toml = toml::encode(&config);

        Ok(self)
    }


    fn insert_deeply<CK: Borrow<ConfigKey>>(table: &mut BTreeMap<String, BTreeMap<String, String>>, key: CK, value: String) {
        let (first, second) = key.borrow().to_pair();

        let second = second.to_string();

        if table.contains_key(first) {
            table.get_mut(first).map(|e| e.insert(second, value));
        } else {
            let mut entry = BTreeMap::new();
            entry.insert(second, value);

            table.insert(first.to_string(), entry);
        }
    }

    pub fn validate<CK: Borrow<ConfigKey>, S: AsRef<str>>(key: CK, value: S) -> Result<String, CliError> {
        let value = value.as_ref().trim();

        match *key.borrow() {
            ConfigKey::BurnAfter => {
                let pair = try!(Regex::new(r"^(?P<num>\d+)\s?(?P<unit>days?|weeks?)$"))
                    .captures(value)
                    .map_or((None, None), |caps| (caps.name("num"), caps.name("unit")));
                let (num, unit) = match pair {
                    (Some(num), Some(unit)) => (num, unit),
                    _                       => return Err(From::from(ConfigError::new(ConfigErrorKind::BurnAfter))),
                };
                Ok(format!("{} {}", num, unit))
            },
            ConfigKey::SweepPeriod => {
                match value {
                    "daily" | "weekly" => Ok(value.to_string()),
                    _                  => Err(From::from(ConfigError::new(ConfigErrorKind::SweepPeriod))),
                }
            },
            ConfigKey::SweepTime => {
                match NaiveTime::from_str(format!("{}:00", value).as_ref()) {
                    Ok(_)  => Ok(value.to_string()),
                    // should set Err(e) to Error::cause()
                    Err(_) => Err(From::from(ConfigError::new(ConfigErrorKind::SweepTime))),
                }
            },
        }
    }
}
