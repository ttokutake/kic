use error::{CliError, Usage, UsageKind};
use super::Command;

use lib::setting::{Config, ConfigKey, Storage};

#[derive(Debug)]
pub struct Burn;

impl Command for Burn {
    fn validation(&self) -> bool { true }

    fn usage(&self) -> Usage {
        return Usage::new(UsageKind::Burn);
    }

    fn main(&self) -> Result<(), CliError> {
        let config         = try!(Config::read());
        let raw_moratorium = try!(config.get(ConfigKey::BurnAfter));
        let moratorium     = Config::to_duration(raw_moratorium);

        let storage = Storage::new();
        try!(storage.delete_expired_boxes(moratorium));

        Ok(())
    }
}
