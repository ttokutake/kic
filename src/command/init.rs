use error::*;
use super::Command;

use constant::*;
use lib::fs::*;
use lib::setting::*;

pub struct Init;

impl Command for Init {
    fn validation(&self) -> bool { false }

    fn usage(&self) -> Usage {
        return Usage { kind: UsageKind::Init };
    }

    fn main(&self) {
        println!("Initialize ...\n");

        create_working_dir();

        create_storage_dir();

        create_config_file(DEFAULT_CONFIG);

        let ignore_contents = walk_dir(".")
            .iter()
            .fold(String::new(), |c, f| c + f + "\n");
        create_ignore_file(ignore_contents);
    }
}
