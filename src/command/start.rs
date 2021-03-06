use error::{CliError, Usage, UsageKind};
use super::Command;

use lib::setting::{Config, ConfigKey, Cron};

#[derive(Debug)]
pub struct Start;

impl Command for Start {
    fn usage(&self) -> Usage {
        return Usage::new(UsageKind::Start);
    }

    #[cfg(unix)]
    fn main(&self) -> Result<(), CliError> {
        let cron = try!(Cron::read());

        let pair_for_burn = ("0 0 * * *", "burn indeed");

        let config        = try!(Config::read());
        let day_and_month = match try!(config.get(ConfigKey::SweepPeriod)).as_ref() {
            "daily"  => "* * *",
            "weekly" => "* * 0",
            _        => unreachable!("Mistake SweepPeriod's validation!!"),
        };
        let (hour, minute) = Config::to_hour_and_minute(try!(config.get(ConfigKey::SweepTime)));
        let when = format!("{} {} {}", minute, hour, day_and_month);
        let pair_for_sweep = (&when as &str, "sweep indeed");

        cron
            .update(&[pair_for_burn, pair_for_sweep])
            .and_then(|new_cron| new_cron.set())
    }
    #[cfg(windows)]
    fn main(&self) -> Result<(), CliError> {
        unimplemented!();
    }
}
