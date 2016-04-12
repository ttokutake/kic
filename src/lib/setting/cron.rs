use error::{CliError, CronError};
use lib::io::*;
use std::process;
use std::str;


#[derive(Debug)]
pub struct Cron {
    contents: String,
}

impl Cron {
    pub fn read() -> Result<Self, CliError> {
        print_with_tag(Tag::Info, "Read cron");

        let result = process::Command::new("crontab")
            .arg("-l")
            .output();
        let output = try!(result);

        let contents = if output.status.success() {
            try!(str::from_utf8(&output.stdout).map_err(|_| CronError))
        } else {
            ""
        };

        Ok(Cron { contents: contents.to_string() })
    }
}
