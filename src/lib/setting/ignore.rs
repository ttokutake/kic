use constant::{IGNORE_FILE_NAME, MAIN_DIR};
use lib::io::*;
use lib::fs::*;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{Error as IoError, Read};
use std::path::{Path, PathBuf};


#[derive(Debug)]
pub struct Ignore {
    entries: BTreeSet<String>,
}

impl Ignore {
    fn path() -> PathBuf {
        path_buf![super::working_dir(), IGNORE_FILE_NAME]
    }

    pub fn exist() -> bool {
        Self::path().is_file()
    }


    fn to_string(entries: Vec<String>) -> String {
        entries
            .iter()
            .fold(String::new(), |contents, entry| contents + entry + "\n")
    }

    pub fn create(self) -> Result<(), IoError> {
        print_with_tag(Tag::Info, format!("Create \"{}\" file", IGNORE_FILE_NAME));

        let (dirs, files): (Vec<String>, Vec<String>) = self.entries
            .into_iter()
            .partition(|s| Path::new(s).is_dir());

        let files = files
            .into_iter()
            .filter(|f| !dirs.iter().any(|d| f.starts_with(d)))
            .collect::<Vec<String>>();

        super::create_setting_file(Self::path(), Self::to_string(dirs) + &Self::to_string(files))
    }


    fn _new(entries: BTreeSet<String>) -> Self {
        Ignore { entries: entries }
    }

    pub fn new() -> Self {
        Self::_new(BTreeSet::new())
    }

    pub fn default() -> Self {
        let current_entries = walk_dir(MAIN_DIR);

        Self::_new(current_entries)
    }

    pub fn read() -> Result<Self, IoError> {
        print_with_tag(Tag::Info, format!("Read \"{}\" file", IGNORE_FILE_NAME));

        let mut f = try!(File::open(Self::path()));

        let mut contents = String::new();
        try!(f.read_to_string(&mut contents));

        let entries = contents
            .lines()
            .map(|l| l.trim().to_string())
            .collect::<BTreeSet<String>>();

        Ok(Self::_new(entries))
    }


    pub fn entries(&self) -> &BTreeSet<String> {
        &self.entries
    }


    pub fn add(mut self, paths: &Vec<String>) -> Self {
        let paths_to_be_added = paths
            .iter()
            .filter(|p| Path::new(p).exists())
            .map(supply_current_dir_prefix)
            .map(supply_dir_suffix)
            .collect::<BTreeSet<String>>();

        self.entries = self
            .entries()
            .union(&paths_to_be_added)
            .cloned()
            .collect::<BTreeSet<String>>();

        self
    }

    pub fn remove(mut self, paths: &Vec<String>) -> Self {
        let paths_to_be_removed = paths
            .iter()
            .map(supply_current_dir_prefix)
            .map(supply_dir_suffix)
            .collect::<BTreeSet<String>>();

        self.entries = self
            .entries()
            .difference(&paths_to_be_removed)
            .cloned()
            .collect::<BTreeSet<String>>();

        self
    }
}


#[test]
fn remove_should_remove_specified_entries() {
    let e1 = "./a"  .to_string();
    let e2 = "./b"  .to_string();
    let e3 = "./c/d".to_string();
    let e4 = "./c/e".to_string();

    let mut entries = BTreeSet::new();
    entries.insert(e1.clone());
    entries.insert(e2.clone());
    entries.insert(e3.clone());
    entries.insert(e4.clone());

    let ignore = Ignore::_new(entries.clone());

    entries.remove(&e1);
    let ignore = ignore.remove(&vec![e1]);
    assert_eq!(&entries, ignore.entries());

    entries.remove(&e3);
    entries.remove(&e4);
    let ignore = ignore.remove(&vec![e3, e4]);
    assert_eq!(&entries, ignore.entries());
}
