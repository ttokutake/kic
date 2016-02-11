mod init;
mod set;
mod sweep;
mod burn;
mod start;
mod end;
mod destroy;

use self::init::Init;
use self::set::Set;
use self::sweep::Sweep;
use self::burn::Burn;
use self::start::Start;
use self::end::End;
use self::destroy::Destroy;

use constant::*;
use lib::cmd_helper::*;
use std::env;

trait Command {
    fn validation(&self) -> bool;
    fn validate(&self) -> Option<String> {
        fn message(subject: String) -> String {
            format!(r#"  Warning: {} does not exist. Please type "kic init"."#, subject)
        }

        if !working_dir_exists() {
            return Some(message(format!(r#""{}" directory"#, WORKING_DIR_NAME)));
        }
        if !config_file_exists() {
            return Some(message(format!(r#""{}" file"#, CONFIG_FILE_NAME)));
        }
        if !ignore_file_exists() {
            return Some(message(format!(r#""{}" file"#, IGNORE_FILE_NAME)));
        }

        None
    }

    fn help_message(&self) -> &'static str;
    fn help(&self) {
        println!("{}", self.help_message());
    }

    fn main(&self);

    fn exec(&self, need_help: bool) {
        if self.validation() {
            if let Some(message) = self.validate() {
                println!("{}", message);
                return;
            }
        }

        if need_help {
            self.help();
        } else {
            self.main();
        }
    }
}

pub fn execute(args: Vec<String>) {
    if args.is_empty() {
        print_usage();
        return;
    }

    let need_help = args.iter().any(|a| *a == "-h" || *a == "--help");

    let command: Box<Command> = match args[0].as_ref() {
        "init"    => Box::new(Init   ),
        "set"     => Box::new(Set    ),
        "sweep"   => Box::new(Sweep  ),
        "burn"    => Box::new(Burn   ),
        "start"   => Box::new(Start  ),
        "end"     => Box::new(End    ),
        "destroy" => Box::new(Destroy),
        _         => {
            print_usage();
            return;
        },
    };

    command.exec(need_help);
}

pub fn print_usage() {
    let full_path_to_bin = env::current_exe().unwrap();
    let bin_name         = full_path_to_bin
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();

    println!(
"Usage:
    {} <command>

Commands:
    init    # Register current directory.
    set     # Set parameters.
    sweep   # Sweep files in current directory.
    burn    # Burn sweeped files.
    start   # Start \"{}\".
    end     # End \"{}\".
    destroy # Destroy \"{}\".",
        bin_name,
        bin_name,
        bin_name,
        bin_name,
    );
}
