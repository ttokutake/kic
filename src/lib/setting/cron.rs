extern crate regex;

use self::regex::{Error as RegexError, Regex};

use constant::ME;
use error::{CliError, CronError, CronErrorKind};
use lib::io::*;
use std::collections::BTreeSet;
use std::env;
use std::fmt::Display;
use std::io::Write;
use std::path::Path;
use std::process::{self, Stdio};
use std::str;


#[derive(Debug)]
pub struct Cron {
    upper  : String,
    my_area: String,
    lower  : String,
}

impl Cron {
    fn base_mark<D: Display>(keyword: &str, exec: D) -> String {
        format!(
            r#"###################################
# "{}" uses the lines {}.
# Please don't touch them and me!
###################################
{}"#,
            ME,
            keyword,
            exec,
        )
    }
    fn start_mark() -> String {
        Self::base_mark("from this", Self::patrol())
    }
    fn end_mark() -> String {
        Self::base_mark("up to here", "")
    }

    fn patrol() -> String {
        format!("0 0 * * *\t{} patrol\n", ME)
    }

    fn escape_asterisk<S: AsRef<str>>(s: S) -> String {
        let re = match Regex::new(r"\*") {
            Ok(re) => re,
            Err(_) => unreachable!("Mistake regular expression!!"),
        };
        re.replace_all(s.as_ref(), "\\*")
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

        let areas = format!(
            "^(?P<upper>(.|\n)*){}(?P<my_area>(.|\n)*){}(?P<lower>(.|\n)*)$",
            Self::escape_asterisk(Self::start_mark()),
            Self::end_mark(),
        );
        let re = match Regex::new(&areas) {
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
        let current_dir = try!(Self::current_dir_string());
        self.delete(&current_dir);

        let my_new_area = pairs
            .iter()
            .fold(String::new(), |area, &(time, command)| {
                let line = format!("{}\tcd {} && {} {}\n", time, current_dir, ME, command);
                area + &line
            });

        self.my_area = self.my_area + &my_new_area;
        Ok(self)
    }


    fn re_for_matching_line<S: AsRef<str>>(core: S) -> String {
        format!(r".*cd\s+{}\s+&&\s+{}.*\n", core.as_ref(), ME)
    }

    pub fn delete<S: AsRef<str>>(&mut self, dir: S) {
        let re = Self::re_for_matching_line(dir);
        let re = match Regex::new(re.as_ref()) {
            Ok(re) => re,
            Err(_) => unreachable!("Wrong to use delete()!!"),
        };
        self.my_area = re.replace_all(&self.my_area, "");
    }

    pub fn discard(mut self) -> Result<Self, RegexError> {
        let re = Self::re_for_matching_line(r"(?P<path>[^\s]+)");
        let target_paths = try!(Regex::new(re.as_ref()))
            .captures_iter(&self.my_area)
            .map(|caps| match caps.name("path") {
                Some(p) => p.to_string(),
                None    => unreachable!("Mistake regular expression!!"),
            })
            .filter(|path| !Path::new(path).is_dir())
            .collect::<BTreeSet<String>>();
        if target_paths.len() == 0 {
            return Ok(self);
        }

        let re_core = target_paths
            .iter()
            .fold(String::new(), |core, path| core + "|" + path); // We want to use true "Iterator.reduce()".
        let re_core = re_core.trim_left_matches('|');
        let re = Self::re_for_matching_line(format!("({})", re_core));
        let re = try!(Regex::new(re.as_ref()));
        self.my_area = re.replace_all(&self.my_area, "");
        Ok(self)
    }

    pub fn set(self) -> Result<(), CliError> {
        print_with_tag(Tag::Info, "Set new cron");

        let result = process::Command::new("crontab")
            .stdin(Stdio::piped())
            .spawn();
        let mut child = try!(result);

        let contents = if self.my_area_is_empty() {
            self.upper + &self.lower
        } else {
            self.upper
                + &Self::start_mark()
                + &self.my_area
                + &Self::end_mark()
                + &self.lower
        };
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


    pub fn current_dir_string() -> Result<String, CronError> {
        let current_dir = match env::current_dir() {
            Ok(d)  => d,
            Err(_) => unreachable!("There is a problem for current directory!!"),
        };
        current_dir
            .to_str()
            .ok_or(CronError::new(CronErrorKind::InvalidPath))
            .map(|s| s.to_string())
    }


    fn my_area_is_empty(&self) -> bool {
        let re = match Regex::new(r"^\s*$") {
            Ok(re) => re,
            Err(_) => unreachable!("Mistake the regular expression!!"),
        };
        re.is_match(self.my_area.as_ref())
    }
}


#[test]
fn my_area_is_empty_should_return_true() {
    let empties = [
        "",
        " ",
        "\t",
        "\r",
        "\n",
        " \t\r\n",
    ];

    for empty in &empties {
        let cron = Cron {
            upper: ""     .to_string(),
            my_area: empty.to_string(),
            lower: ""     .to_string(),
        };
        assert!(cron.my_area_is_empty());
    }
}
#[test]
fn my_area_is_empty_should_return_false() {
    let non_empties = [
        "a",
        "ab",
        " ab",
        "a b",
        "ab ",
        "\tab",
        "a\tb",
        "ab\t",
        "\rab",
        "a\rb",
        "ab\r",
        "\nab",
        "a\nb",
        "ab\n",
        " \t\r\nab",
        "a \t\r\nb",
        "ab \t\r\n",
    ];

    for non_empty in &non_empties {
        let cron = Cron {
            upper  : ""       .to_string(),
            my_area: non_empty.to_string(),
            lower  : ""       .to_string(),
        };
        assert!(!cron.my_area_is_empty());
    }
}

#[test]
fn delete_should_success() {
    fn wrap_by_extra_content<S: AsRef<str>>(target: S) -> String {
        let extra_content = "when cd path_to_extra && kic command\n";
        format!("{}{}{}", extra_content, target.as_ref(), extra_content)
    }

    let mut cron = Cron {
        upper  : "upper\n".to_string(),
        my_area: ""       .to_string(),
        lower  : "lower\n".to_string(),
    };

    let dir = "path_to_dir";

    let areas = vec![
        ("".to_string(), format!("cd {} && kic\n"             , dir)),
        ("".to_string(), format!("when cd {} && kic\n"        , dir)),
        ("".to_string(), format!("cd {} && kic command\n"     , dir)),
        ("".to_string(), format!("when cd {} && kic command\n", dir)),

        (wrap_by_extra_content(""), wrap_by_extra_content(format!("cd {} && kic\n"             , dir))),
        (wrap_by_extra_content(""), wrap_by_extra_content(format!("when cd {} && kic\n"        , dir))),
        (wrap_by_extra_content(""), wrap_by_extra_content(format!("cd {} && kic command\n"     , dir))),
        (wrap_by_extra_content(""), wrap_by_extra_content(format!("when cd {} && kic command\n", dir))),
    ];

    for (correct, area) in areas.into_iter() {
        cron.my_area = area;
        cron.delete(&dir);
        assert_eq!(correct, cron.my_area);
    }
}
#[test]
#[should_panic(expect = "entered unreachable code")]
fn delete_should_fail() {
    let mut cron = Cron {
        upper  : "upper\n".to_string(),
        my_area: ""       .to_string(),
        lower  : "lower\n".to_string(),
    };

    cron.delete("() <= mistaken Regular Expression!");
}
