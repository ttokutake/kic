use constant::{IGNORE_FILE_NAME, MAIN_DIR};
use lib::io::*;
use lib::fs::*;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{Error as IoError, Read};
use std::path::{Path, PathBuf};


#[derive(Debug)]
pub struct Ignore {
    files: BTreeSet<String>,
}

impl Ignore {
    fn path() -> PathBuf {
        path_buf![super::working_dir(), IGNORE_FILE_NAME]
    }

    pub fn exist() -> bool {
        Self::path().is_file()
    }


    fn to_string(&self) -> String {
        self.files
            .iter()
            .fold(String::new(), |contents, file| contents + file + "\n")
    }

    pub fn create(&self) -> Result<(), IoError> {
        print_with_tag(Tag::Info, format!("Create \"{}\" file", IGNORE_FILE_NAME));

        super::create_setting_file(Self::path(), self.to_string())
    }


    fn _new(files: BTreeSet<String>) -> Self {
        Ignore { files: files }
    }

    pub fn new() -> Self {
        Self::_new(BTreeSet::new())
    }

    pub fn default() -> Self {
        let current_files = walk_dir(MAIN_DIR);

        Self::_new(current_files)
    }

    pub fn read() -> Result<Self, IoError> {
        print_with_tag(Tag::Info, format!("Read \"{}\" file", IGNORE_FILE_NAME));

        let mut f = try!(File::open(Self::path()));

        let mut contents = String::new();
        try!(f.read_to_string(&mut contents));

        let files = contents
            .lines()
            .map(|l| l.trim().to_string())
            .collect::<BTreeSet<String>>();

        Ok(Self::_new(files))
    }


    pub fn files(&self) -> &BTreeSet<String> {
        &self.files
    }


    pub fn add(mut self, paths: &Vec<String>) -> Self {
        let paths_to_be_added = paths
            .iter()
            .map(trim_current_dir_prefix)
            .filter(|p| Path::new(p).is_file())
            .collect::<BTreeSet<String>>();

        self.files = self
            .files()
            .union(&paths_to_be_added)
            .cloned()
            .collect::<BTreeSet<String>>();

        self
    }

    pub fn remove(mut self, paths: &Vec<String>) -> Self {
        let paths_to_be_removed = paths
            .iter()
            .map(trim_current_dir_prefix)
            .collect::<BTreeSet<String>>();

        self.files = self
            .files()
            .difference(&paths_to_be_removed)
            .cloned()
            .collect::<BTreeSet<String>>();

        self
    }
}


#[test]
fn remove_should_remove_specified_files() {
    let f1 = "a"  .to_string();
    let f2 = "b"  .to_string();
    let f3 = "c/d".to_string();
    let f4 = "c/e".to_string();

    let mut files = BTreeSet::new();
    files.insert(f1.clone());
    files.insert(f2.clone());
    files.insert(f3.clone());
    files.insert(f4.clone());

    let ignore = Ignore::_new(files.clone());

    files.remove(&f1);
    let ignore = ignore.remove(&vec![f1]);
    assert_eq!(&files, ignore.files());

    files.remove(&f3);
    files.remove(&f4);
    let ignore = ignore.remove(&vec![f3, f4]);
    assert_eq!(&files, ignore.files());
}
