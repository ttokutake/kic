use error::{CliError, Usage, UsageKind};
use super::Command;

use constant::IGNORE_FILE_NAME;
use lib::setting;

#[derive(Debug)]
pub struct Ignore {
    command: Option<String>,
    paths  : Vec<String>,
}

impl Command for Ignore {
    fn usage(&self) -> Usage {
        return Usage::new(UsageKind::Ignore);
    }

    fn main(&self) -> Result<(), CliError> {
        match self.command {
            Some(ref c) => match c.as_ref() {
                "add"     => self.add(),
                "remove"  => self.remove(),
                "current" => Self::ignore_current_files(),
                "clear"   => Self::clear_ignore_file(),
                _         => Err(From::from(self.usage())),
            },
            None => Err(From::from(self.usage())),
        }
    }
}

impl Ignore {
    pub fn new(command: Option<String>, paths: Vec<String>) -> Ignore {
        Ignore { command: command, paths: paths }
    }

    fn add(&self) -> Result<(), CliError> {
        let paths = &self.paths;

        if paths.len() == 0 {
            return Err(From::from(self.usage()));
        }

        let ignore = try!(setting::Ignore::read()).add(paths);

        try!(ignore.create());

        Ok(())
    }

    fn remove(&self) -> Result<(), CliError> {
        let paths = &self.paths;

        if paths.len() == 0 {
            return Err(From::from(self.usage()));
        }

        let ignore = try!(setting::Ignore::read()).remove(paths);

        try!(ignore.create());

        Ok(())
    }

    fn ignore_current_files() -> Result<(), CliError> {
        let message = "Do you want to preserve current state?";

        Self::run_after_confirmation(message, || setting::Ignore::default().create().map_err(|e| From::from(e)))
    }

    fn clear_ignore_file() -> Result<(), CliError> {
        let message = format!("Do you want to clear \"{}\"?", IGNORE_FILE_NAME);

        Self::run_after_confirmation(message, || setting::Ignore::new().create().map_err(|e| From::from(e)))
    }
}
