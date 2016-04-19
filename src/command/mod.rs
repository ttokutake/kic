mod init;
mod config;
mod ignore;
mod sweep;
mod burn;
mod start;
mod end;
mod destroy;
mod patrol;

use self::init::Init;
use self::config::Config;
use self::ignore::Ignore;
use self::sweep::Sweep;
use self::burn::Burn;
use self::start::Start;
use self::end::End;
use self::destroy::Destroy;
use self::patrol::Patrol;

use constant::BANNED_DIRS;
use error::{CliError, EssentialLack, EssentialKind, RunningPlaceError, Usage, UsageKind};
use lib::io::*;
use lib::setting;
use std::env;
use std::fmt::Display;
use std::io::Error as IoError;

trait Command {
    fn allow_to_check_current_dir(&self) -> bool { true }
    fn check_current_dir(&self) -> Result<(), CliError> {
        let current_dir = try!(env::current_dir());

        match BANNED_DIRS.iter().find(|d| current_dir.ends_with(d)) {
            Some(d) => Err(From::from(RunningPlaceError::new(d.to_string()))),
            None    => Ok(()),
        }
    }

    fn allow_to_check_settings(&self) -> bool { true }
    fn check_settings(&self) -> Result<(), EssentialLack> {
        if !setting::working_dir_exists() {
            return Err(EssentialLack::new(EssentialKind::WorkingDir));
        }
        if !setting::Storage::exist() {
            return Err(EssentialLack::new(EssentialKind::StorageDir));
        }
        if !setting::Config::exist() {
            return Err(EssentialLack::new(EssentialKind::ConfigFile));
        }
        if !setting::Ignore::exist() {
            return Err(EssentialLack::new(EssentialKind::IgnoreFile));
        }

        Ok(())
    }

    fn usage(&self) -> Usage;

    fn main(&self) -> Result<(), CliError>;

    fn exec(&self, need_help: bool) -> Result<(), CliError> {
        if need_help {
            return Err(From::from(self.usage()));
        }

        if self.allow_to_check_current_dir() {
            try!(self.check_current_dir());
        }

        if self.allow_to_check_settings() {
            try!(self.check_settings());
        }

        self.main()
    }

    fn run_after_confirmation<D: Display, F>(message: D, danger_exec: F) -> Result<(), CliError>
        where Self: Sized, F: FnOnce() -> Result<(), IoError>
    {
        echo(format_with_tag(Tag::Caution, format!("{} [yes/no]: ", message)));

        let input = try!(read_line_from_stdin()).to_lowercase();
        match input.as_ref() {
            "y" | "yes" => try!(danger_exec()),
            _           => print_with_tag(Tag::Notice, "Interrupted by user"),
        };

        Ok(())
    }
}

pub fn execute() -> Result<(), CliError> {
    let args = env::args()
        .skip(1)
        .collect::<Vec<String>>();

    let mut args = args.into_iter();

    let command = try!(args.next().ok_or(Usage::new(UsageKind::Nothing)));

    let (command, need_help) = if &command == "help" {
        (try!(args.next().ok_or(Usage::new(UsageKind::Help))), true)
    } else {
        (command, false)
    };

    match command.as_ref() {
        "init"    => Init                                                 .exec(need_help),
        "config"  => Config::new(args.next(), args.next()   , args.next()).exec(need_help),
        "ignore"  => Ignore::new(args.next(), args.collect()             ).exec(need_help),
        "sweep"   => Sweep                                                .exec(need_help),
        "burn"    => Burn                                                 .exec(need_help),
        "start"   => Start                                                .exec(need_help),
        "end"     => End                                                  .exec(need_help),
        "destroy" => Destroy                                              .exec(need_help),
        "patrol"  => Patrol                                               .exec(need_help),
        _         => Err(From::from(Usage::new(UsageKind::Nothing))),
    }
}

pub fn clean_up_cron() -> Result<(), CliError> {
    if cfg!(unix) {
        End.main()
    } else {
        Ok(())
    }
}
