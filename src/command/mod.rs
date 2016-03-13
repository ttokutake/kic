mod init;
mod config;
mod ignore;
mod sweep;
mod burn;
mod start;
mod end;
mod destroy;

use self::init::Init;
use self::config::Config;
use self::ignore::Ignore;
use self::sweep::Sweep;
use self::burn::Burn;
use self::start::Start;
use self::end::End;
use self::destroy::Destroy;

use constant::*;
use error::*;
use lib::setting::*;
use std::env;

trait Command {
    fn validation(&self) -> bool;
    fn validate(&self) -> Result<(), EssentialLack> {
        if !working_dir_exists() {
            return Err(EssentialLack::new(EssentialKind::WorkingDir));
        }
        if !storage_dir_exists() {
            return Err(EssentialLack::new(EssentialKind::StorageDir));
        }
        if !config_file_exists() {
            return Err(EssentialLack::new(EssentialKind::ConfigFile));
        }
        if !ignore_file_exists() {
            return Err(EssentialLack::new(EssentialKind::IgnoreFile));
        }

        Ok(())
    }

    fn usage(&self) -> Usage;

    fn main(&self) -> Result<(), CliError>;

    fn exec(&self, help: bool) -> Result<(), CliError> {
        if self.validation() {
            try!(self.validate());
        }

        if help {
            Err(From::from(self.usage()))
        } else {
            self.main()
        }
    }
}

fn validate_at_first() -> Result<(), CliError> {
    let current_dir = try!(env::current_dir());

    match BANNED_DIRS.iter().find(|d| current_dir.ends_with(d)) {
        Some(d) => Err(From::from(RunningPlaceError::new(d.to_string()))),
        None    => Ok(()),
    }
}

pub fn execute() -> Result<(), CliError> {
    try!(validate_at_first());

    let args = env::args()
        .skip(1)
        .collect::<Vec<String>>();

    let help = args.iter().any(|a| *a == "-h" || *a == "--help");

    let mut args = args.into_iter();

    let command: Box<Command> = match args.next() {
        Some(first) => match first.as_ref() {
            "init"    => Box::new(Init   ),
            "config"  => Box::new(Config ),
            "ignore"  => Box::new(Ignore { command: args.next(), paths: args.collect() }),
            "sweep"   => Box::new(Sweep  ),
            "burn"    => Box::new(Burn   ),
            "start"   => Box::new(Start  ),
            "end"     => Box::new(End    ),
            "destroy" => Box::new(Destroy),
            _         => return Err(From::from(Usage::new(UsageKind::Nothing))),
        },
        None => return Err(From::from(Usage::new(UsageKind::Nothing))),
    };

    command.exec(help)
}
