use super::Command;

use constant::*;
use lib::io::*;
use lib::setting::*;

pub struct Destroy;

impl Command for Destroy {
    fn validation(&self) -> bool { true }

    fn help_message(&self) -> &'static str {
        return "destroy!";
    }

    fn main(&self) {
        println!("Destroy ...\n");

        let caution = format!(r#"CAUTION: Do you want to clear all files related to "{}"? [yes/no]: "#, ME);
        echo(caution);

        let input = read_line_from_stdin();

        match input.to_lowercase().as_ref() {
            "y" | "yes" => delete_all_setting_files(),
            _           => println!("  NOTICE: Interrupted by user."),
        }
    }
}
