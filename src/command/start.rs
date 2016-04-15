use error::{CliError, Usage, UsageKind};
use super::Command;

use lib::setting::{Config, ConfigKey};
use lib::setting::Cron;

#[derive(Debug)]
pub struct Start;

impl Command for Start {
    fn validation(&self) -> bool { true }

    fn usage(&self) -> Usage {
        return Usage::new(UsageKind::Start);
    }

    fn main(&self) -> Result<(), CliError> {
        let cron = try!(Cron::read());

        let pair_for_burn  = ("0 0 * * *", "burn");

        let config = try!(Config::read());
        let day    = match try!(config.get(ConfigKey::SweepPeriod)).as_ref() {
            "daily"  => "* * *",
            "weekly" => "* * 0",
            _        => unreachable!("Mistake SweepPeriod's validation!!"),
        };
        let (hour, minute) = Config::to_hour_and_minute(try!(config.get(ConfigKey::SweepTime)));
        let time = format!("{} {} {}", minute, hour, day);
        let pair_for_sweep = (&time as &str, "sweep");

        let new_cron = try!(cron.update(&[pair_for_burn, pair_for_sweep]));
        try!(new_cron.set());

        Ok(())
    }
}
