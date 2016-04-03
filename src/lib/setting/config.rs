extern crate chrono;
extern crate regex;
extern crate toml;

use self::chrono::{Duration, NaiveTime};
use self::regex::Regex;
use self::toml::Value as Toml;

use constant::CONFIG_FILE_NAME;
use error::{CannotHappenError, CliError, ConfigError, ConfigErrorKind};
use lib::io::*;
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


type EditableTomlCore = BTreeMap<String, BTreeMap<String, String>>;

struct EditableToml(EditableTomlCore);

impl EditableToml {
    fn from(toml: Toml) -> Result<Self, ConfigError> {
        toml::decode::<EditableTomlCore>(toml)
            .map(|c| EditableToml(c))
            .ok_or(ConfigError::new(ConfigErrorKind::Something))
    }

    fn overwrite<CK: Borrow<ConfigKey>>(&mut self, key: CK, value: String) {
        let &mut EditableToml(ref mut core) = self;

        let (first, second) = key.borrow().to_pair();
        let second          = second.to_string();

        if core.contains_key(first) {
            core.get_mut(first).map(|e| e.insert(second, value));
        } else {
            let mut entry = BTreeMap::new();
            entry.insert(second, value);

            core.insert(first.to_string(), entry);
        }
    }

    fn to_toml(self) -> Toml {
        let EditableToml(core) = self;
        toml::encode(&core)
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
        print_with_tag(Tag::Info, format!("Create \"{}\" file", CONFIG_FILE_NAME));

        super::create_setting_file(Self::path(), self.to_string())
    }


    fn _new(toml: Toml) -> Self {
        Config { toml: toml }
    }

    pub fn default() -> Self {
        let mut editable = EditableToml(BTreeMap::new());
        editable.overwrite(ConfigKey::BurnAfter  , "2 weeks".to_string());
        editable.overwrite(ConfigKey::SweepPeriod, "daily"  .to_string());
        editable.overwrite(ConfigKey::SweepTime  , "00:00"  .to_string());

        Self::_new(editable.to_toml())
    }

    pub fn read() -> Result<Self, CliError> {
        print_with_tag(Tag::Info, format!("Read \"{}\" file", CONFIG_FILE_NAME));

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


    pub fn get<CK: Borrow<ConfigKey>>(&self, key: CK) -> Result<String, CliError> {
        let key = key.borrow();

        let result = self.toml
            .lookup(key.to_str())
            .ok_or(ConfigError::new(match *key {
                ConfigKey::BurnAfter   => ConfigErrorKind::NotFoundBurnAfter,
                ConfigKey::SweepPeriod => unimplemented!(),
                ConfigKey::SweepTime   => unimplemented!(),
            }));
        let value = try!(result);

        value
            .as_str()
            .ok_or(From::from(ConfigError::new(ConfigErrorKind::NonStringValue)))
            .and_then(|s| Self::validate(key, s))
    }

    pub fn to_duration(value: String) -> Result<Duration, CliError> {
        let mut value   = value.split(' ');
        let (num, unit) = match (value.next(), value.next()) {
            (Some(num), Some(unit)) => (num, unit),
            _                       => return Err(From::from(CannotHappenError)),
        };
        let num = try!(num.parse::<u32>()) as i64;

        let duration = match unit {
            "day"  | "days"  => Duration::days(num),
            "week" | "weeks" => Duration::weeks(num),
            _                => return Err(From::from(CannotHappenError)),
        };

        Ok(duration)
    }


    pub fn set<CK: Borrow<ConfigKey>>(mut self, key: CK, value: String) -> Result<Self, ConfigError> {
        let mut editable = try!(EditableToml::from(self.toml));

        editable.overwrite(key, value);

        self.toml = editable.to_toml();

        Ok(self)
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


#[test]
fn overwrite_should_add_value {
}

#[test]
fn default_should_return_config() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_should_return_ok() {
    }
    #[test]
    fn get_should_return_err() {
    }

    #[test]
    fn set_should_return_ok() {
    }
    #[test]
    fn set_should_retrun_err() {
    }

    #[test]
    fn to_duration_should_return_ok {
    }
    #[test]
    fn to_duration_should_return_err {
    }

    #[test]
    fn validate_should_return_ok() {
    }
    #[test]
    fn validate_should_return_err() {
    }
}
