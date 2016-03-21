use error::{CliError, Usage, UsageKind};
use super::Command;

use lib::fs::*;
use lib::io::*;
use lib::setting;
use std::collections::BTreeSet;
use std::path::Path;

pub struct Ignore {
    command: Option<String>,
    paths  : Vec<String>,
}

impl Command for Ignore {
    fn validation(&self) -> bool { true }

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

        let ignores_to_be_added = paths
            .iter()
            .map(append_prefix_if_need)
            .filter(|p| Path::new(p).is_file())
            .collect::<BTreeSet<String>>();

        for file in &ignores_to_be_added {
            print_with_tag(Tag::Info, format!("\"{}\" will be ignored", file));
        }

        let original_ignores = try!(setting::read_ignore_file());

        let new_ignores = original_ignores
            .union(&ignores_to_be_added)
            .fold(String::new(), |c, ref f| c + f + "\n");

        try!(setting::create_ignore_file(new_ignores));

        Ok(())
    }

    fn remove(&self) -> Result<(), CliError> {
        let paths = &self.paths;

        if paths.len() == 0 {
            return Err(From::from(self.usage()));
        }

        let ignores_to_be_removed = paths
            .iter()
            .map(append_prefix_if_need)
            .collect::<BTreeSet<String>>();

        for file in &ignores_to_be_removed {
            print_with_tag(Tag::Info, format!("\"{}\" will not be ignored", file));
        }

        let original_ignores = try!(setting::read_ignore_file());

        let new_ignores = original_ignores
            .difference(&ignores_to_be_removed)
            .fold(String::new(), |c, ref f| c + f + "\n");

        try!(setting::create_ignore_file(new_ignores));

        Ok(())
    }

    fn ignore_current_files() -> Result<(), CliError> {
        try!(setting::create_initial_ignore_file());

        Ok(())
    }

    fn clear_ignore_file() -> Result<(), CliError> {
        try!(setting::create_ignore_file("\n"));

        Ok(())
    }
}
