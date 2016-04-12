extern crate regex;

use self::regex::{Error as RegexError, Regex};

use constant::ME;
use error::{CliError, CronError};
use lib::io::*;
use std::process;
use std::str;


#[derive(Debug)]
pub struct Cron {
    contents: String,
}

impl Cron {
    fn base_mark(keyword: &str) -> String {
        format!(
            r#"###################################
# "{}" uses the lines {}.         #
# Please don't touch them and me! #
###################################
"#,
            ME,
            keyword,
        )
    }
    fn start_mark() -> String {
        Cron::base_mark("from this")
    }
    fn end_mark() -> String {
        Cron::base_mark("up to here")
    }

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

    pub fn update(mut self) -> Result<Self, RegexError> {
        let start_mark  = Cron::start_mark();
        let end_mark    = Cron::end_mark();
        let my_area     = format!("{}.*{}"    , &start_mark, &end_mark);
        let my_new_area = format!("{}blur\n{}", &start_mark, &end_mark);

        let re           = try!(Regex::new(&my_area));
        let new_contents = if re.is_match(&self.contents) {
            let my_new_area: &str = my_new_area.as_ref();
            re.replace(self.contents.as_ref(), my_new_area)
        } else {
            self.contents + &my_new_area
        };

        self.contents = new_contents;
        Ok(self)
    }
}
