extern crate chrono;
extern crate regex;
extern crate toml;

use self::chrono::{Duration, NaiveTime};
use self::regex::Regex;
use self::toml::Value as Toml;

use constant::CONFIG_FILE_NAME;
use error::{CliError, ConfigError, ConfigErrorKind};
use lib::io::*;
use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{Error as IoError, Read};
use std::path::PathBuf;
use std::str::FromStr;


const CONFIG_KEY_BURN_AFTER  : &'static str = "burn.after";
const CONFIG_KEY_SWEEP_PERIOD: &'static str = "sweep.period";
const CONFIG_KEY_SWEEP_TIME  : &'static str = "sweep.time";

const CONFIG_DEFAULT_VALUE_BURN_AFTER  : &'static str = "2 weeks";
const CONFIG_DEFAULT_VALUE_SWEEP_PERIOD: &'static str = "daily";
const CONFIG_DEFAULT_VALUE_SWEEP_TIME  : &'static str = "00:00";


#[derive(Debug)]
pub enum ConfigKey {
    BurnAfter,
    SweepPeriod,
    SweepTime,
}

impl ConfigKey {
    pub fn from<S: AsRef<str>>(key: S) -> Result<ConfigKey, ConfigError> {
        match key.as_ref().trim() {
            CONFIG_KEY_BURN_AFTER   => Ok(ConfigKey::BurnAfter),
            CONFIG_KEY_SWEEP_PERIOD => Ok(ConfigKey::SweepPeriod),
            CONFIG_KEY_SWEEP_TIME   => Ok(ConfigKey::SweepTime),
            _                       => Err(ConfigError::new(ConfigErrorKind::InvalidKey)),
        }
    }

    fn to_str(&self) -> &str {
        match *self {
            ConfigKey::BurnAfter   => CONFIG_KEY_BURN_AFTER,
            ConfigKey::SweepPeriod => CONFIG_KEY_SWEEP_PERIOD,
            ConfigKey::SweepTime   => CONFIG_KEY_SWEEP_TIME,
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

#[derive(Clone, Debug)]
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


#[derive(Debug)]
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
        editable.overwrite(ConfigKey::BurnAfter  , CONFIG_DEFAULT_VALUE_BURN_AFTER  .to_string());
        editable.overwrite(ConfigKey::SweepPeriod, CONFIG_DEFAULT_VALUE_SWEEP_PERIOD.to_string());
        editable.overwrite(ConfigKey::SweepTime  , CONFIG_DEFAULT_VALUE_SWEEP_TIME  .to_string());

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
                None    => return unreachable!("Evil thing will occur in toml-rs!!"),
            },
        };

        Ok(Self::_new(toml))
    }


    pub fn get<CK: Borrow<ConfigKey>>(&self, key: CK) -> Result<String, CliError> {
        let key = key.borrow();

        print_with_tag(Tag::Info, format!("Get the parameter for \"{}\"", key.to_str()));

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

    pub fn to_duration(value: String) -> Duration {
        let mut value   = value.split(' ');
        let (num, unit) = match (value.next(), value.next()) {
            (Some(num), Some(unit)) => (num, unit),
            _                       => unreachable!("Wrong to use to_duration()!!"),
        };
        let num = match num.parse::<u32>() {
            Ok(u)  => u as i64,
            Err(_) => unreachable!("Wrong to use to_duration()!!"),
        };

        match unit {
            "day"  | "days"  => Duration::days(num),
            "week" | "weeks" => Duration::weeks(num),
            _                => unreachable!("Wrong to use to_duration()!!"),
        }
    }


    pub fn set<CK: Borrow<ConfigKey>, S: AsRef<str>>(mut self, key: CK, value: S) -> Result<Self, CliError> {
        let key = key.borrow();

        print_with_tag(Tag::Info, format!("Set the parameter for \"{}\"", key.to_str()));

        let value = try!(Self::validate(key, value));

        let mut editable = try!(EditableToml::from(self.toml));

        editable.overwrite(key, value);

        self.toml = editable.to_toml();
        Ok(self)
    }


    fn validate<CK: Borrow<ConfigKey>, S: AsRef<str>>(key: CK, value: S) -> Result<String, CliError> {
        let value = value.as_ref().trim();

        match *key.borrow() {
            ConfigKey::BurnAfter => {
                let pair = try!(Regex::new(r"^(?P<num>\d+)\s?(?P<unit>days?|weeks?)$"))
                    .captures(value)
                    .map_or((None, None), |caps| (caps.name("num"), caps.name("unit")));
                let (num, unit) = match pair {
                    (Some(num), Some(unit)) if num != "0" => (num, unit),
                    _                                     => return Err(From::from(ConfigError::new(ConfigErrorKind::BurnAfter))),
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
fn config_key_to_pair_should_return_pair() {
    let keys = [
        (ConfigKey::BurnAfter  , CONFIG_KEY_BURN_AFTER  ),
        (ConfigKey::SweepPeriod, CONFIG_KEY_SWEEP_PERIOD),
        (ConfigKey::SweepTime  , CONFIG_KEY_SWEEP_TIME  ),
    ];
    for &(ref key, ref correct) in &keys {
        let (first, second) = key.to_pair();
        assert_eq!(correct.to_string(), format!("{}.{}", first, second));
    }
}

#[test]
fn editable_toml_overwrite_should_add_value() {
    let b_tree_map: EditableTomlCore = BTreeMap::new();
    let mut correct                  = b_tree_map.clone();
    let mut editable                 = EditableToml(b_tree_map);

    let v1 = "value1".to_string();

    let mut entry         = BTreeMap::new();
    let key1              = ConfigKey::BurnAfter;
    let (first1, second1) = key1.to_pair();
    entry.insert(second1.to_string(), v1.clone());
    correct.insert(first1.to_string(), entry);

    editable.overwrite(ConfigKey::BurnAfter, v1);
    let EditableToml(calculated) = editable.clone();

    assert_eq!(&correct, &calculated);

    let v2 = "value2".to_string();
    let v3 = "value3".to_string();

    let mut entry         = BTreeMap::new();
    let key2              = ConfigKey::SweepPeriod;
    let (first2, second2) = key2.to_pair();
    let key3              = ConfigKey::SweepTime;
    let (_, second3)      = key3.to_pair();
    entry.insert(second2.to_string(), v2.clone());
    entry.insert(second3.to_string(), v3.clone());
    correct.insert(first2.to_string(), entry);

    editable.overwrite(ConfigKey::SweepPeriod, v2);
    editable.overwrite(ConfigKey::SweepTime  , v3);
    let &EditableToml(ref calculated) = &editable;

    assert_eq!(&correct, calculated);
}

#[test]
fn default_should_return_config() {
    let correct: Toml = format!(
        r#"
            [burn]
            after = "{}"
            [sweep]
            period = "{}"
            time   = "{}"
        "#,
        CONFIG_DEFAULT_VALUE_BURN_AFTER,
        CONFIG_DEFAULT_VALUE_SWEEP_PERIOD,
        CONFIG_DEFAULT_VALUE_SWEEP_TIME
    )
        .parse()
        .unwrap();

    assert_eq!(correct, Config::default().toml);
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate chrono;
    extern crate regex;

    use self::chrono::Duration;
    use self::regex::Regex;

    use error::{CliError, ConfigError, ConfigErrorKind};
    use std::u32;

    #[test]
    fn get_should_return_ok() {
        let config = Config::default();

        assert!(config.get(ConfigKey::BurnAfter).is_ok());
    }
    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn get_should_panic() {
        let config = Config::default();

        assert!(config.get(ConfigKey::SweepPeriod).is_ok());
        assert!(config.get(ConfigKey::SweepTime  ).is_ok());
    }

    #[test]
    fn set_should_return_ok() {
        let raw_values = vec![
            "1day"  .to_string(),
            "1days" .to_string(),
            "1 day" .to_string(),
            "1 days".to_string(),
            "1week"  .to_string(),
            "1weeks" .to_string(),
            "1 week" .to_string(),
            "1 weeks".to_string(),

            format!("{}day"  , u32::MAX),
            format!("{}days" , u32::MAX),
            format!("{} day" , u32::MAX),
            format!("{} days", u32::MAX),
            format!("{}week"  , u32::MAX),
            format!("{}weeks" , u32::MAX),
            format!("{} week" , u32::MAX),
            format!("{} weeks", u32::MAX),
        ];
        let corrects_and_inputs = raw_values
            .into_iter()
            .map(|i| {
                let re = Regex::new(r"(?P<num>\d+)(?P<unit>[^\d\s]+)").unwrap();
                (re.replace(i.as_ref(), "$num $unit"), i)
            })
            .collect::<Vec<(String, String)>>();

        for (correct, input) in corrects_and_inputs.into_iter() {
            let config = Config::default()
                .set(ConfigKey::BurnAfter, input)
                .unwrap();
            assert_eq!(correct, config.get(ConfigKey::BurnAfter).unwrap());
        }
    }
    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn set_should_return_panic() {
        let raw_values = ["daily", "weekly"];
        for raw_value in &raw_values {
            let config = Config::default()
                .set(ConfigKey::SweepPeriod, &raw_value)
                .unwrap();
            assert_eq!(raw_value.to_string(), config.get(ConfigKey::SweepPeriod).unwrap())
        }

        let raw_values = ["00:00", "23:59"];
        for raw_value in &raw_values {
            let config = Config::default()
                .set(ConfigKey::SweepPeriod, &raw_value)
                .unwrap();
            assert_eq!(raw_value.to_string(), config.get(ConfigKey::SweepTime).unwrap())
        }
    }
    #[test]
    fn set_should_return_err() {
        let data_set = vec![
            (ConfigKey::BurnAfter, "-1day"   , ConfigError::new(ConfigErrorKind::BurnAfter)),
            (ConfigKey::BurnAfter, "-1days"  , ConfigError::new(ConfigErrorKind::BurnAfter)),
            (ConfigKey::BurnAfter, "-1 day"  , ConfigError::new(ConfigErrorKind::BurnAfter)),
            (ConfigKey::BurnAfter, "-1 days" , ConfigError::new(ConfigErrorKind::BurnAfter)),
            (ConfigKey::BurnAfter, "0day"    , ConfigError::new(ConfigErrorKind::BurnAfter)),
            (ConfigKey::BurnAfter, "0days"   , ConfigError::new(ConfigErrorKind::BurnAfter)),
            (ConfigKey::BurnAfter, "0 day"   , ConfigError::new(ConfigErrorKind::BurnAfter)),
            (ConfigKey::BurnAfter, "0 days"  , ConfigError::new(ConfigErrorKind::BurnAfter)),
            (ConfigKey::BurnAfter, "-1week"  , ConfigError::new(ConfigErrorKind::BurnAfter)),
            (ConfigKey::BurnAfter, "-1weeks" , ConfigError::new(ConfigErrorKind::BurnAfter)),
            (ConfigKey::BurnAfter, "-1 week" , ConfigError::new(ConfigErrorKind::BurnAfter)),
            (ConfigKey::BurnAfter, "-1 weeks", ConfigError::new(ConfigErrorKind::BurnAfter)),
            (ConfigKey::BurnAfter, "0week"   , ConfigError::new(ConfigErrorKind::BurnAfter)),
            (ConfigKey::BurnAfter, "0weeks"  , ConfigError::new(ConfigErrorKind::BurnAfter)),
            (ConfigKey::BurnAfter, "0 week"  , ConfigError::new(ConfigErrorKind::BurnAfter)),
            (ConfigKey::BurnAfter, "0 weeks" , ConfigError::new(ConfigErrorKind::BurnAfter)),
            (ConfigKey::BurnAfter, "1hour"   , ConfigError::new(ConfigErrorKind::BurnAfter)),
            (ConfigKey::BurnAfter, "1month"  , ConfigError::new(ConfigErrorKind::BurnAfter)),
            (ConfigKey::BurnAfter, "1year"   , ConfigError::new(ConfigErrorKind::BurnAfter)),

            (ConfigKey::SweepPeriod, "day"    , ConfigError::new(ConfigErrorKind::SweepPeriod)),
            (ConfigKey::SweepPeriod, "week"   , ConfigError::new(ConfigErrorKind::SweepPeriod)),
            (ConfigKey::SweepPeriod, "hourly" , ConfigError::new(ConfigErrorKind::SweepPeriod)),
            (ConfigKey::SweepPeriod, "monthly", ConfigError::new(ConfigErrorKind::SweepPeriod)),
            (ConfigKey::SweepPeriod, "yearly" , ConfigError::new(ConfigErrorKind::SweepPeriod)),

            (ConfigKey::SweepTime, "-00:01"   , ConfigError::new(ConfigErrorKind::SweepTime)),
            (ConfigKey::SweepTime,  "24:00"   , ConfigError::new(ConfigErrorKind::SweepTime)),
            (ConfigKey::SweepTime,  "00"      , ConfigError::new(ConfigErrorKind::SweepTime)),
            (ConfigKey::SweepTime,  "00:00:00", ConfigError::new(ConfigErrorKind::SweepTime)),
        ];

        for (key, value, correct) in data_set.into_iter() {
            let cli_err = Config::default()
                .set(key, value)
                .unwrap_err();
            let err = match cli_err {
                CliError::Config(e) => e,
                _                   => panic!("unexpected error"),
            };
            assert_eq!(correct, err);
        }
    }

    #[test]
    fn to_duration_should_return_duration() {
        let data_set = vec![
            ("1 day" , Duration::days(1)),
            ("1 days", Duration::days(1)),
            ("2 day" , Duration::days(2)),
            ("2 days", Duration::days(2)),

            ("1 week" , Duration::weeks(1)),
            ("1 weeks", Duration::weeks(1)),
            ("2 week" , Duration::weeks(2)),
            ("2 weeks", Duration::weeks(2)),
        ];
        for (input, correct) in data_set.into_iter() {
            assert_eq!(correct, Config::to_duration(input.to_string()));
        }
    }
}
