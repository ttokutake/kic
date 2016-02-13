use super::Command;

use lib::io::*;
use lib::setting::*;

pub struct Destroy;

impl Command for Destroy {
    fn validation(&self) -> bool { true }

    fn help_message(&self) -> &'static str {
        return "destroy!";
    }

    fn main(&self) {
        println!("Destroy ...");

        echo("Do you want to delete all related files?: ");
        delete_all_setting_files();
    }
}
