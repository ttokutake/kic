use error::*;
use super::Command;

use constant::*;
use lib::io::*;
use lib::setting::*;

pub struct Destroy;

impl Command for Destroy {
    fn validation(&self) -> bool { true }

    fn usage(&self) -> Usage {
        return Usage { kind: UsageKind::Destroy };
    }

    fn main(&self) {
        println!("Destroy ...\n");

        let message = format!("Do you want to clear all files related to \"{}\"? [yes/no]: ", ME);
        echo(format_with_tag(0, Tag::Caution, message));

        match read_line_from_stdin() {
            Ok(input) => match input.to_lowercase().as_ref() {
                "y" | "yes" => delete_all_setting_files(),
                _           => print_with_tag(1, Tag::Notice, "Interrupted by user"),
            },
            Err(why) => print_with_error(1, why),
        };
    }
}
