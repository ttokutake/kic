extern crate toml;

use self::toml::Value as Toml;

use error::{CannotHappenError, CliError, ConfigError, ConfigErrorKind};
use constant::CONFIG_FILE_NAME;
use lib::io::*;
use lib::setting;
use std::fs::File;
use std::io::Read;

pub enum ParamKind {
    BurnAfter,
}
impl ParamKind {
    fn to_str(&self) -> &str {
        match *self {
            ParamKind::BurnAfter => "burn.after",
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

    pub fn extract(kind: ParamKind) -> Result<String, CliError> {
        let key = kind.to_str();
        print_with_tag(0, Tag::Execution, format!("Extract \"{}\" parameter", key));

        let config = try!(Self::read());
        let result = config
            .lookup(key)
            .ok_or(ConfigError::new(match kind {
                ParamKind::BurnAfter => ConfigErrorKind::NotFoundBurnAfter,
            }));
        let value = try!(result).to_string(); // This to_string() is not documented.

        print_with_okay(1);
        Ok(value)
    }
}
