use error::*;
use super::Command;

use constant::*;
use lib::fs::*;
use lib::setting::*;

pub struct Init;

impl Command for Init {
    fn validation(&self) -> bool { false }

    fn usage(&self) -> Usage {
        return Usage::new(UsageKind::Init);
    }

    fn main(&self) -> Result<(), CliError> {
        println!("Initialize ...\n");

        try!(create_working_dir());

        try!(create_storage_dir());

        try!(create_config_file(DEFAULT_CONFIG));

        let ignore_contents = walk_dir(".")
            .iter()
            .fold(String::new(), |c, f| c + f + "\n");
        try!(create_ignore_file(ignore_contents));

        Ok(())
    }
}
