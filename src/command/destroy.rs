use error::{CliError, Usage, UsageKind};
use super::Command;

use constant::ME;
use lib::io::*;
use lib::setting;

pub struct Destroy;

impl Command for Destroy {
    fn validation(&self) -> bool { true }

    fn usage(&self) -> Usage {
        return Usage::new(UsageKind::Destroy);
    }

    fn main(&self) -> Result<(), CliError> {
        let message = format!("Do you want to clear all files related to \"{}\"? [yes/no]: ", ME);
        echo(format_with_tag(Tag::Caution, message));

        match try!(read_line_from_stdin()).to_lowercase().as_ref() {
            "y" | "yes" => try!(setting::delete_all_setting_files()),
            _           => print_with_tag(Tag::Notice, "Interrupted by user"),
        };

        Ok(())
    }
}
