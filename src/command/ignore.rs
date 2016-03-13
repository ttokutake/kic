use error::*;
use super::Command;

use constant::*;
use lib::io::*;
use lib::setting::*;
use std::collections::BTreeSet;
use std::path::{MAIN_SEPARATOR, Path};

pub struct Ignore {
    pub command: Option<String>,
    pub paths  : Vec<String>,
}

impl Command for Ignore {
    fn validation(&self) -> bool { true }

    fn usage(&self) -> Usage {
        return Usage::new(UsageKind::Ignore);
    }

    fn main(&self) -> Result<(), CliError> {
        match self.command {
            Some(ref c) => match c.as_ref() {
                "add"     => add(self),
                "remove"  => remove(self),
                "current" => ignore_current_files(),
                "clear"   => clear_ignore_file(),
                _         => Err(From::from(self.usage())),
            },
            None => Err(From::from(self.usage())),
        }
    }
}


fn append_prefix_if_need(path: &String) -> String {
    let prefix       = format!(".{}", MAIN_SEPARATOR);
    let prefix: &str = prefix.as_ref();
    format!("{}{}", if path.starts_with(prefix) { "" } else { prefix }, path)
}

fn add(ignore: &Ignore) -> Result<(), CliError> {
    let paths = &ignore.paths;

    if paths.len() == 0 {
        return Err(From::from(ignore.usage()));
    }

    print_with_tag(0, Tag::Execution, format!("Confirm files to be added to \"{}\"", IGNORE_FILE_NAME));

    let ignores_to_be_added = paths
        .iter()
        .map(append_prefix_if_need)
        .filter(|p| Path::new(p).is_file())
        .collect::<BTreeSet<String>>();

    for file in &ignores_to_be_added {
        print_with_tag(1, Tag::Info, format!("\"{}\" will be ignored", file));
    }

    let original_ignores = try!(read_ignore_file());

    let new_ignores = original_ignores
        .union(&ignores_to_be_added)
        .fold(String::new(), |c, ref f| c + f + "\n");

    print_with_tag(0, Tag::Execution, format!("Recreate \"{}\" file", IGNORE_FILE_NAME));

    try!(create_ignore_file(new_ignores));

    Ok(())
}

fn remove(ignore: &Ignore) -> Result<(), CliError> {
    let paths = &ignore.paths;
    if paths.len() == 0 {
        return Err(From::from(ignore.usage()));
    }
    println!("do remove command with {:?}", paths);

    Ok(())
}

fn ignore_current_files() -> Result<(), CliError> {
    println!("do current command");

    Ok(())
}

fn clear_ignore_file() -> Result<(), CliError> {
    println!("do clear command");

    Ok(())
}
