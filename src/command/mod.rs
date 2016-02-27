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
use lib::io::*;
use lib::setting::*;

trait Command {
    fn validation(&self) -> bool;
    fn validate(&self) -> Result<(), String> {
        fn message(subject: String) -> String {
            format!("{} does not exist. Please use \"init\" command", subject)
        }

        if !working_dir_exists() {
            return Err(message(format!("\"{}\" directory", WORKING_DIR_NAME)));
        }
        if !storage_dir_exists() {
            return Err(message(format!("\"{}\" directory", STORAGE_DIR_NAME)));
        }
        if !config_file_exists() {
            return Err(message(format!("\"{}\" file", CONFIG_FILE_NAME)));
        }
        if !ignore_file_exists() {
            return Err(message(format!("\"{}\" file", IGNORE_FILE_NAME)));
        }

        Ok(())
    }

    fn help_message(&self) -> &'static str;
    fn help(&self) {
        println!("{}", self.help_message());
    }

    fn main(&self);

    fn exec(&self, help: bool) {
        if self.validation() {
            if let Err(message) = self.validate() {
                return print_with_tag(0, Tag::Warning, message);
            }
        }

        if help {
            self.help();
        } else {
            self.main();
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
            "ignore"  => Box::new(Ignore { command: args.next(), value: args.next() }),
            "sweep"   => Box::new(Sweep  ),
            "burn"    => Box::new(Burn   ),
            "start"   => Box::new(Start  ),
            "end"     => Box::new(End    ),
            "destroy" => Box::new(Destroy),
            _         => return print_usage(),
        },
        None => return print_usage(),
    };

    command.exec(help);
}

pub fn print_usage() {
    println!(
r#"Usage:
    {} <command> [--help|-h]

Command:
    init    # Register current directory.
    config  # Set config's parameters.
    ignore  # Set ignored files.
    sweep   # Sweep files in current directory.
    burn    # Burn sweeped files.
    start   # Start "{}".
    end     # End "{}".
    destroy # Destroy "{}"."#,
        ME,
        ME,
        ME,
        ME,
    );
}
