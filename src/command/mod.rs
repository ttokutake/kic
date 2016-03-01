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
    fn validate(&self) -> Result<(), ()> {
        fn message(subject: String) -> String {
            format!("{} does not exist. Please use \"init\" command", subject)
        }
        fn message_for_dir(dir_name: &'static str) -> String {
            message(format!("\"{}\" directory", dir_name))
        }
        fn message_for_file(file_name: &'static str) -> String {
            message(format!("\"{}\" file", file_name))
        }

        if !working_dir_exists() {
            print_with_tag(0, Tag::Warning, message_for_dir(WORKING_DIR_NAME));
            return Err(());
        }
        if !storage_dir_exists() {
            print_with_tag(0, Tag::Warning, message_for_dir(STORAGE_DIR_NAME));
            return Err(());
        }
        if !config_file_exists() {
            print_with_tag(0, Tag::Warning, message_for_file(CONFIG_FILE_NAME));
            return Err(());
        }
        if !ignore_file_exists() {
            print_with_tag(0, Tag::Warning, message_for_file(IGNORE_FILE_NAME));
            return Err(());
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
            if let Err(_) = self.validate() {
                return;
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
            "ignore"  => Box::new(Ignore { command: args.next(), paths: args.collect() }),
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
