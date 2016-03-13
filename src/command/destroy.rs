use error::*;
use super::Command;

use constant::*;
use lib::io::*;
use lib::setting::*;

pub struct Destroy;

impl Command for Destroy {
    fn validation(&self) -> bool { true }

    fn usage(&self) -> Usage {
        return Usage::new(UsageKind::Destroy);
    }

    fn main(&self) -> Result<(), CliError> {
        println!("Destroy ...\n");

        let message = format!("Do you want to clear all files related to \"{}\"? [yes/no]: ", ME);
        echo(format_with_tag(0, Tag::Caution, message));

        match try!(read_line_from_stdin()).to_lowercase().as_ref() {
            "y" | "yes" => try!(delete_all_setting_files()),
            _           => print_with_tag(1, Tag::Notice, "Interrupted by user"),
        };

        Ok(())
    }
}
