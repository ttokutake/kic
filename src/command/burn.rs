use error::{CliError, Usage, UsageKind};
use super::Command;

use lib::setting::{Config, ConfigKey, Storage};

#[derive(Debug)]
pub struct Burn {
    option: Option<String>,
}

impl Command for Burn {
    fn usage(&self) -> Usage {
        return Usage::new(UsageKind::Burn);
    }

    fn main(&self) -> Result<(), CliError> {
        let indeed = match &self.option {
            &Some(ref option) => match option.as_ref() {
                "indeed" => true,
                _        => return Err(From::from(self.usage())),
            },
            &None => false,
        };

        let config         = try!(Config::read());
        let raw_moratorium = try!(config.get(ConfigKey::BurnAfter));
        let moratorium     = Config::to_duration(raw_moratorium);

        let storage = try!(Storage::new(None, indeed).create_box_with_log("burn"));
        try!(storage.delete_expired_boxes(moratorium));

        Ok(())
    }
}

impl Burn {
    pub fn new(option: Option<String>) -> Self {
        Burn { option: option }
    }
}
