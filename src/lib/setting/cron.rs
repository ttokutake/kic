extern crate regex;

use self::regex::Regex;

use constant::ME;
use error::{CliError, CronError, CronErrorKind};
use lib::io::*;
use std::env;
use std::io::Write;
use std::process::{self, Stdio};
use std::str;


#[derive(Debug)]
pub struct Cron {
    upper  : String,
    my_area: String,
    lower  : String,
}

impl Cron {
    fn base_mark(keyword: &str) -> String {
        format!(
            r#"###################################
# "{}" uses the lines {}.
# Please don't touch them and me!
###################################
"#,
            ME,
            keyword,
        )
    }
    fn start_mark() -> String {
        Self::base_mark("from this")
    }
    fn end_mark() -> String {
        Self::base_mark("up to here")
    }

    pub fn read() -> Result<Self, CliError> {
        print_with_tag(Tag::Info, "Read cron");

        let result = process::Command::new("crontab")
            .arg("-l")
            .output();
        let output = try!(result);

        let contents = if output.status.success() {
            try!(str::from_utf8(&output.stdout).map_err(|_| CronError::new(CronErrorKind::InvalidCharacterCode)))
        } else {
            ""
        };

        let areas = format!("^(?P<upper>(.|\n)*){}(?P<my_area>(.|\n)*){}(?P<lower>(.|\n)*)$", Self::start_mark(), Self::end_mark());
        let re    = match Regex::new(&areas) {
            Ok(re) => re,
            Err(_) => unreachable!("Mistake the regular expression!!"),
        };

        let (upper, my_area, lower) = match re.captures(contents) {
            Some(caps) => match (caps.name("upper"), caps.name("my_area"), caps.name("lower")) {
                (Some(u), Some(m), Some(l)) => (u, m, l),
                _                           => unreachable!("Mistake regular expression!!"),
            },
            None => (contents, "", ""),
        };

        Ok(Cron { upper: upper.to_string(), my_area: my_area.to_string(), lower: lower.to_string() })
    }

    pub fn update(mut self, pairs: &[(&str, &str); 2]) -> Result<Self, CliError> {
        let current_dir = try!(env::current_dir());
        let current_dir = try!(current_dir.to_str().ok_or(CronError::new(CronErrorKind::InvalidPath)));

        self.delete(current_dir);

        let my_new_area = pairs
            .iter()
            .fold(String::new(), |area, &(ref time, ref command)| {
                let line = format!("{}\tcd {} && kic {}\n", time, current_dir, command);
                area + &line
            });

        self.my_area = self.my_area + &my_new_area;
        Ok(self)
    }

    pub fn set(self) -> Result<(), CliError> {
        print_with_tag(Tag::Info, "Set new cron");

        let result = process::Command::new("crontab")
            .stdin(Stdio::piped())
            .spawn();
        let mut child = try!(result);

        let contents = self.upper
            + &Self::start_mark()
            + &self.my_area
            + &Self::end_mark()
            + &self.lower;
        match &mut child.stdin {
            &mut Some(ref mut stdin) => try!(stdin.write_all(contents.as_bytes())),
            &mut None                => unreachable!("Please set Stdio::piped()!!"),
        };

        let output = try!(child.wait_with_output());
        if output.status.success() {
            Ok(())
        } else {
            Err(From::from(CronError::new(CronErrorKind::FailedToWrite)))
        }
    }


    fn delete(&mut self, dir: &str) {
        let re = match Regex::new(&format!(r".*cd\s+{}\s+&&\s+kic.*\n", dir)) {
            Ok(re) => re,
            Err(_) => unreachable!("Mistake the regular expression!!"),
        };
        self.my_area = re.replace_all(&self.my_area, "");
    }
}
