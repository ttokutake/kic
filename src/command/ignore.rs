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

    fn help_message(&self) -> &'static str {
        return "ignore!";
    }

    fn main(&self) {
        match self.command {
            Some(ref c) => match c.as_ref() {
                "add"     => add(self),
                "remove"  => remove(self),
                "current" => ignore_current_files(),
                "clear"   => clear_ignore_file(),
                _         => self.help(),
            },
            None => self.help(),
        };
    }
}


fn append_prefix_if_need(path: &String) -> String {
    let prefix       = format!(".{}", MAIN_SEPARATOR);
    let prefix: &str = prefix.as_ref();
    format!("{}{}", if path.starts_with(prefix) { "" } else { prefix }, path)
}

fn add(ignore: &Ignore) {
    let paths = &ignore.paths;

    if paths.len() == 0 {
        return ignore.help();
    }

    print_with_tag(0, Tag::Execution, format!("Confirm files to be added to \"{}\"", IGNORE_FILE_NAME));

    let ignores_to_be_added = paths
        .iter()
        .map(append_prefix_if_need)
        .filter(|p| Path::new(p).is_file())
        .collect::<BTreeSet<String>>();

    if ignores_to_be_added.len() == 0 {
        return print_with_tag(1, Tag::Warning, "The files do not exist to be added");
    }

    for file in &ignores_to_be_added {
        print_with_tag(1, Tag::Info, format!("\"{}\" will be ignored", file));
    }

    let new_ignores = read_ignore_file()
        .union(&ignores_to_be_added)
        .fold(String::new(), |c, ref f| c + f + "\n");

    print_with_tag(0, Tag::Execution, format!("Recreate \"{}\" file", IGNORE_FILE_NAME));

    create_ignore_file(new_ignores);
}

fn remove(ignore: &Ignore) {
    let paths = &ignore.paths;
    if paths.len() == 0 {
        return ignore.help();
    }
    println!("do remove command with {:?}", paths);
}

fn ignore_current_files() {
    println!("do current command");
}

fn clear_ignore_file() {
    println!("do clear command");
}
