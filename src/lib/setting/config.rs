extern crate chrono;
extern crate regex;
extern crate toml;

use self::chrono::{Duration, NaiveTime, ParseError as TimeParseError, Timelike};
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


const CONFIG_KEY_BURN_MORATORIUM : &'static str = "burn.moratorium";
const CONFIG_KEY_SWEEP_MORATORIUM: &'static str = "sweep.moratorium";
const CONFIG_KEY_SWEEP_PERIOD    : &'static str = "sweep.period";
const CONFIG_KEY_SWEEP_TIME      : &'static str = "sweep.time";

const CONFIG_DEFAULT_VALUE_BURN_MORATORIUM : &'static str = "2 weeks";
const CONFIG_DEFAULT_VALUE_SWEEP_MORATORIUM: &'static str = "10 minutes";
const CONFIG_DEFAULT_VALUE_SWEEP_PERIOD    : &'static str = "daily";
const CONFIG_DEFAULT_VALUE_SWEEP_TIME      : &'static str = "00:00";


#[derive(Debug)]
pub enum ConfigKey {
    BurnMoratorium,
    SweepMoratorium,
    SweepPeriod,
    SweepTime,
}

impl ConfigKey {
    pub fn from<S: AsRef<str>>(key: S) -> Result<ConfigKey, ConfigError> {
        match key.as_ref().trim() {
            CONFIG_KEY_BURN_MORATORIUM  => Ok(ConfigKey::BurnMoratorium),
            CONFIG_KEY_SWEEP_MORATORIUM => Ok(ConfigKey::SweepMoratorium),
            CONFIG_KEY_SWEEP_PERIOD     => Ok(ConfigKey::SweepPeriod),
            CONFIG_KEY_SWEEP_TIME       => Ok(ConfigKey::SweepTime),
            _                           => Err(ConfigError::new(ConfigErrorKind::InvalidKey)),
        }
    }

    fn to_str(&self) -> &str {
        match *self {
            ConfigKey::BurnMoratorium  => CONFIG_KEY_BURN_MORATORIUM,
            ConfigKey::SweepMoratorium => CONFIG_KEY_SWEEP_MORATORIUM,
            ConfigKey::SweepPeriod     => CONFIG_KEY_SWEEP_PERIOD,
            ConfigKey::SweepTime       => CONFIG_KEY_SWEEP_TIME,
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


    fn new(toml: Toml) -> Self {
        Config { toml: toml }
    }

    pub fn default() -> Self {
        let mut editable = EditableToml(BTreeMap::new());
        editable.overwrite(ConfigKey::BurnMoratorium , CONFIG_DEFAULT_VALUE_BURN_MORATORIUM .to_string());
        editable.overwrite(ConfigKey::SweepMoratorium, CONFIG_DEFAULT_VALUE_SWEEP_MORATORIUM.to_string());
        editable.overwrite(ConfigKey::SweepPeriod    , CONFIG_DEFAULT_VALUE_SWEEP_PERIOD    .to_string());
        editable.overwrite(ConfigKey::SweepTime      , CONFIG_DEFAULT_VALUE_SWEEP_TIME      .to_string());

        Self::new(editable.to_toml())
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

        Ok(Self::new(toml))
    }


    pub fn get<CK: Borrow<ConfigKey>>(&self, key: CK) -> Result<String, ConfigError> {
        let key = key.borrow();

        print_with_tag(Tag::Info, format!("Get the parameter for \"{}\"", key.to_str()));

        let result = self.toml
            .lookup(key.to_str())
            .ok_or(ConfigError::new(match *key {
                ConfigKey::BurnMoratorium  => ConfigErrorKind::NotFoundBurnMoratorium,
                ConfigKey::SweepMoratorium => ConfigErrorKind::NotFoundSweepMoratorium,
                ConfigKey::SweepPeriod     => ConfigErrorKind::NotFoundSweepPeriod,
                ConfigKey::SweepTime       => ConfigErrorKind::NotFoundSweepTime,
            }));
        let value = try!(result);

        value
            .as_str()
            .ok_or(ConfigError::new(ConfigErrorKind::NonStringValue))
            .and_then(|s| Self::validate(key, s))
    }

    pub fn to_duration(value: String) -> Duration {
        let mut value   = value.split(' ');
        let (num, unit) = match (value.next(), value.next()) {
            (Some(num), Some(unit)) => (num, unit),
            _                       => unreachable!("Wrong to use to_duration()!!"),
        };
        let num = match num.parse::<u32>() {
            Ok(u) => u as i64,
            _     => unreachable!("Wrong to use to_duration()!!"),
        };

        match unit {
            "minute" | "minutes" => Duration::minutes(num),
            "hour"   | "hours"   => Duration::hours(num),
            "day"    | "days"    => Duration::days(num),
            "week"   | "weeks"   => Duration::weeks(num),
            _                    => unreachable!("Wrong to use to_duration()!!"),
        }
    }

    pub fn to_hour_and_minute(value: String) -> (u32, u32) {
        let time = match Self::to_naive_time(value) {
            Ok(t)  => t,
            Err(_) => unreachable!("Wrong to use to_hour_and_minute()!!"),
        };
        (time.hour(), time.minute())
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


    fn to_naive_time<S: AsRef<str>>(value: S) -> Result<NaiveTime, TimeParseError> {
        NaiveTime::from_str(format!("{}:00", value.as_ref()).as_ref())
    }

    fn capture_moratorium<'a>(value: &'a str, allowed_units: &'a str) -> (Option<&'a str>, Option<&'a str>) {
        let re = format!(r"^(?P<num>\d+)\s?(?P<unit>{})$", allowed_units);
        match Regex::new(&re) {
            Ok(re) => re
                .captures(value)
                .map_or((None, None), |caps| (caps.name("num"), caps.name("unit"))),
            Err(_) => unreachable!("Wrong to use this function!!"),
        }
    }

    fn validate<CK: Borrow<ConfigKey>, S: AsRef<str>>(key: CK, value: S) -> Result<String, ConfigError> {
        let value = value.as_ref().trim();

        match *key.borrow() {
            ConfigKey::BurnMoratorium => {
                let (num, unit) = match Self::capture_moratorium(value, "days?|weeks?") {
                    (Some(num), Some(unit)) if num != "0" => (num, unit),
                    _                                     => return Err(ConfigError::new(ConfigErrorKind::BurnMoratorium)),
                };
                Ok(format!("{} {}", num, unit))
            },
            ConfigKey::SweepMoratorium => {
                let (num, unit) = match Self::capture_moratorium(value, "minutes?|hours?|days?|weeks?") {
                    (Some(num), Some(unit)) => (num, unit),
                    _                       => return Err(ConfigError::new(ConfigErrorKind::SweepMoratorium))
                };
                Ok(format!("{} {}", num, unit))
            },
            ConfigKey::SweepPeriod => {
                match value {
                    "daily" | "weekly" => Ok(value.to_string()),
                    _                  => Err(ConfigError::new(ConfigErrorKind::SweepPeriod)),
                }
            },
            ConfigKey::SweepTime => {
                match Self::to_naive_time(value) {
                    Ok(_)  => Ok(value.to_string()),
                    Err(_) => Err(ConfigError::new(ConfigErrorKind::SweepTime)),
                }
            },
        }
    }
}


#[test]
fn config_key_to_pair_should_return_pair() {
    let keys = [
        (ConfigKey::BurnMoratorium , CONFIG_KEY_BURN_MORATORIUM ),
        (ConfigKey::SweepMoratorium, CONFIG_KEY_SWEEP_MORATORIUM),
        (ConfigKey::SweepPeriod    , CONFIG_KEY_SWEEP_PERIOD    ),
        (ConfigKey::SweepTime      , CONFIG_KEY_SWEEP_TIME      ),
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
    let key1              = ConfigKey::BurnMoratorium;
    let (first1, second1) = key1.to_pair();
    entry.insert(second1.to_string(), v1.clone());
    correct.insert(first1.to_string(), entry);

    editable.overwrite(ConfigKey::BurnMoratorium, v1);
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
            moratorium = "{}"
            [sweep]
            moratorium = "{}"
            period = "{}"
            time   = "{}"
        "#,
        CONFIG_DEFAULT_VALUE_BURN_MORATORIUM,
        CONFIG_DEFAULT_VALUE_SWEEP_MORATORIUM,
        CONFIG_DEFAULT_VALUE_SWEEP_PERIOD,
        CONFIG_DEFAULT_VALUE_SWEEP_TIME
    )
        .parse()
        .unwrap();

    assert_eq!(correct, Config::default().toml);
}

#[test]
fn to_naive_time_should_return_naivetime() {
    let data_set = vec![
        (NaiveTime::from_hms(0 , 0 , 0), "00:00"),
        (NaiveTime::from_hms(23, 59, 0), "23:59"),
    ];
    for (correct, input) in data_set.into_iter() {
        let result = Config::to_naive_time(input.to_string());
        assert!(result.is_ok());
        assert_eq!(correct, result.unwrap());
    }
}
#[test]
fn to_naive_time_should_return_err() {
    let data_set = [
        "-00:00",
        "00",
        "00:00:00",
        "23:59:59",
        "24:00",
        "invalid value",
    ];
    for input in &data_set {
        let result = Config::to_naive_time(input.to_string());
        assert!(result.is_err());
    }
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

        assert!(config.get(ConfigKey::BurnMoratorium ).is_ok());
        assert!(config.get(ConfigKey::SweepMoratorium).is_ok());
        assert!(config.get(ConfigKey::SweepPeriod    ).is_ok());
        assert!(config.get(ConfigKey::SweepTime      ).is_ok());
    }

    #[test]
    fn set_should_replace_value_with_new_value() {
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
                .set(ConfigKey::BurnMoratorium, input)
                .unwrap();
            assert_eq!(correct, config.get(ConfigKey::BurnMoratorium).unwrap());
        }

        let raw_values = vec![
            "0minute"  .to_string(),
            "0minutes" .to_string(),
            "0 minute" .to_string(),
            "0 minutes".to_string(),
            "0hour"  .to_string(),
            "0hours" .to_string(),
            "0 hour" .to_string(),
            "0 hours".to_string(),
            "0day"  .to_string(),
            "0days" .to_string(),
            "0 day" .to_string(),
            "0 days".to_string(),
            "0week"  .to_string(),
            "0weeks" .to_string(),
            "0 week" .to_string(),
            "0 weeks".to_string(),

            format!("{}minute"  , u32::MAX),
            format!("{}minutes" , u32::MAX),
            format!("{} minute" , u32::MAX),
            format!("{} minutes", u32::MAX),
            format!("{}hour"  , u32::MAX),
            format!("{}hours" , u32::MAX),
            format!("{} hour" , u32::MAX),
            format!("{} hours", u32::MAX),
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
                .set(ConfigKey::SweepMoratorium, input)
                .unwrap();
            assert_eq!(correct, config.get(ConfigKey::SweepMoratorium).unwrap());
        }

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
                .set(ConfigKey::SweepTime, &raw_value)
                .unwrap();
            assert_eq!(raw_value.to_string(), config.get(ConfigKey::SweepTime).unwrap())
        }
    }
    #[test]
    fn set_should_return_err() {
        let data_set = vec![
            (ConfigKey::BurnMoratorium, "-1day"   , ConfigError::new(ConfigErrorKind::BurnMoratorium)),
            (ConfigKey::BurnMoratorium, "-1days"  , ConfigError::new(ConfigErrorKind::BurnMoratorium)),
            (ConfigKey::BurnMoratorium, "-1 day"  , ConfigError::new(ConfigErrorKind::BurnMoratorium)),
            (ConfigKey::BurnMoratorium, "-1 days" , ConfigError::new(ConfigErrorKind::BurnMoratorium)),
            (ConfigKey::BurnMoratorium, "0day"    , ConfigError::new(ConfigErrorKind::BurnMoratorium)),
            (ConfigKey::BurnMoratorium, "0days"   , ConfigError::new(ConfigErrorKind::BurnMoratorium)),
            (ConfigKey::BurnMoratorium, "0 day"   , ConfigError::new(ConfigErrorKind::BurnMoratorium)),
            (ConfigKey::BurnMoratorium, "0 days"  , ConfigError::new(ConfigErrorKind::BurnMoratorium)),
            (ConfigKey::BurnMoratorium, "-1week"  , ConfigError::new(ConfigErrorKind::BurnMoratorium)),
            (ConfigKey::BurnMoratorium, "-1weeks" , ConfigError::new(ConfigErrorKind::BurnMoratorium)),
            (ConfigKey::BurnMoratorium, "-1 week" , ConfigError::new(ConfigErrorKind::BurnMoratorium)),
            (ConfigKey::BurnMoratorium, "-1 weeks", ConfigError::new(ConfigErrorKind::BurnMoratorium)),
            (ConfigKey::BurnMoratorium, "0week"   , ConfigError::new(ConfigErrorKind::BurnMoratorium)),
            (ConfigKey::BurnMoratorium, "0weeks"  , ConfigError::new(ConfigErrorKind::BurnMoratorium)),
            (ConfigKey::BurnMoratorium, "0 week"  , ConfigError::new(ConfigErrorKind::BurnMoratorium)),
            (ConfigKey::BurnMoratorium, "0 weeks" , ConfigError::new(ConfigErrorKind::BurnMoratorium)),
            (ConfigKey::BurnMoratorium, "1hour"   , ConfigError::new(ConfigErrorKind::BurnMoratorium)),
            (ConfigKey::BurnMoratorium, "1month"  , ConfigError::new(ConfigErrorKind::BurnMoratorium)),
            (ConfigKey::BurnMoratorium, "1year"   , ConfigError::new(ConfigErrorKind::BurnMoratorium)),

            (ConfigKey::SweepMoratorium, "-1minute"  , ConfigError::new(ConfigErrorKind::SweepMoratorium)),
            (ConfigKey::SweepMoratorium, "-1minutes" , ConfigError::new(ConfigErrorKind::SweepMoratorium)),
            (ConfigKey::SweepMoratorium, "-1 minute" , ConfigError::new(ConfigErrorKind::SweepMoratorium)),
            (ConfigKey::SweepMoratorium, "-1 minutes", ConfigError::new(ConfigErrorKind::SweepMoratorium)),
            (ConfigKey::SweepMoratorium, "-1hour"    , ConfigError::new(ConfigErrorKind::SweepMoratorium)),
            (ConfigKey::SweepMoratorium, "-1hours"   , ConfigError::new(ConfigErrorKind::SweepMoratorium)),
            (ConfigKey::SweepMoratorium, "-1 hour"   , ConfigError::new(ConfigErrorKind::SweepMoratorium)),
            (ConfigKey::SweepMoratorium, "-1 hours"  , ConfigError::new(ConfigErrorKind::SweepMoratorium)),
            (ConfigKey::SweepMoratorium, "-1day"     , ConfigError::new(ConfigErrorKind::SweepMoratorium)),
            (ConfigKey::SweepMoratorium, "-1days"    , ConfigError::new(ConfigErrorKind::SweepMoratorium)),
            (ConfigKey::SweepMoratorium, "-1 day"    , ConfigError::new(ConfigErrorKind::SweepMoratorium)),
            (ConfigKey::SweepMoratorium, "-1 days"   , ConfigError::new(ConfigErrorKind::SweepMoratorium)),
            (ConfigKey::SweepMoratorium, "-1week"    , ConfigError::new(ConfigErrorKind::SweepMoratorium)),
            (ConfigKey::SweepMoratorium, "-1weeks"   , ConfigError::new(ConfigErrorKind::SweepMoratorium)),
            (ConfigKey::SweepMoratorium, "-1 week"   , ConfigError::new(ConfigErrorKind::SweepMoratorium)),
            (ConfigKey::SweepMoratorium, "-1 weeks"  , ConfigError::new(ConfigErrorKind::SweepMoratorium)),
            (ConfigKey::SweepMoratorium, "1month"    , ConfigError::new(ConfigErrorKind::SweepMoratorium)),
            (ConfigKey::SweepMoratorium, "1year"     , ConfigError::new(ConfigErrorKind::SweepMoratorium)),

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
                _                   => panic!("test failed!!!"),
            };
            assert_eq!(correct, err);
        }
    }

    #[test]
    fn to_duration_should_return_duration() {
        let data_set = vec![
            ("0 minute" , Duration::minutes(0)),
            ("0 minutes", Duration::minutes(0)),
            ("1 minute" , Duration::minutes(1)),
            ("1 minutes", Duration::minutes(1)),

            ("0 hour" , Duration::hours(0)),
            ("0 hours", Duration::hours(0)),
            ("1 hour" , Duration::hours(1)),
            ("1 hours", Duration::hours(1)),

            ("0 day" , Duration::days(0)),
            ("0 days", Duration::days(0)),
            ("1 day" , Duration::days(1)),
            ("1 days", Duration::days(1)),

            ("0 week" , Duration::weeks(0)),
            ("0 weeks", Duration::weeks(0)),
            ("1 week" , Duration::weeks(1)),
            ("1 weeks", Duration::weeks(1)),
        ];
        for (input, correct) in data_set.into_iter() {
            assert_eq!(correct, Config::to_duration(input.to_string()));
        }
    }
    #[test]
    #[should_panic(expect = "entered unreachable code")]
    fn to_duration_should_panic_for_0_day() {
        Config::to_duration("-1 day".to_string());
    }
    #[test]
    #[should_panic(expect = "entered unreachable code")]
    fn to_duration_should_panic_for_0_week() {
        Config::to_duration("-1 week".to_string());
    }
    #[test]
    #[should_panic(expect = "entered unreachable code")]
    fn to_duration_should_panic_for_incoherent() {
        Config::to_duration("invalid value".to_string());
    }

    #[test]
    fn to_hour_and_minute_should_return_tuple() {
        let data_set = vec![
            ("00:00", ( 0,  0)),
            ("23:59", (23, 59)),
        ];
        for (input, correct) in data_set.into_iter() {
            assert_eq!(correct, Config::to_hour_and_minute(input.to_string()));
        }
    }
    #[test]
    #[should_panic(expect = "entered unreachable code")]
    fn to_hour_and_minute_should_panic_for_incoherent() {
        Config::to_hour_and_minute("invalid value".to_string());
    }
}
