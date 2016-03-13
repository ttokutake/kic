use error::*;
use super::Command;

use constant::*;
use lib::fs::*;
use lib::io::*;
use lib::setting::*;

pub struct Init;

impl Command for Init {
    fn validation(&self) -> bool { false }

    fn usage(&self) -> Usage {
        return Usage::new(UsageKind::Init);
    }

    fn main(&self) {
        println!("Initialize ...\n");

        if let Err(why) = create_working_dir() {
            print_with_error(1, why);
        };

        if let Err(why) = create_storage_dir() {
            print_with_error(1, why);
        };

        if let Err(why) = create_config_file(DEFAULT_CONFIG) {
            print_with_error(1, why);
        };

        let ignore_contents = walk_dir(".")
            .iter()
            .fold(String::new(), |c, f| c + f + "\n");
        if let Err(why) = create_ignore_file(ignore_contents) {
            print_with_error(1, why);
        };
    }
}
