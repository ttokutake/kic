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

use error::*;
use lib::io::*;
use lib::setting::*;
use std::process;

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
    fn help(&self) -> ! {
        print_usage(self.usage());
    }

    fn main(&self) -> Result<(), CliError>;

    fn exec(&self, help: bool) {
        if self.validation() {
            if let Err(why) = self.validate() {
                print_with_warning(0, why);
            }
        }

        if help {
            self.help();
        } else {
            if let Err(why) = self.main() {
                print_with_error(1, why);
            }
        }
    }
}

pub fn execute(args: Vec<String>) {
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
            _         => print_usage(Usage::new(UsageKind::Nothing)),
        },
        None => print_usage(Usage::new(UsageKind::Nothing)),
    };

    command.exec(help);
}

pub fn print_usage(u: Usage) -> ! {
    println!("{}", u);
    process::exit(1)
}
