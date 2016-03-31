use constant::IGNORE_FILE_NAME;
use lib::fs::*;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{Error as IoError, Read};
use std::path::{MAIN_SEPARATOR, Path, PathBuf};


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
        super::create_setting_file(Self::path(), self.to_string())
    }


    fn _new(files: BTreeSet<String>) -> Self {
        Ignore { files: files }
    }

    pub fn new() -> Self {
        Self::_new(BTreeSet::new())
    }

    pub fn default() -> Self {
        let current_files = walk_dir(".");

        Self::_new(current_files)
    }

    pub fn read() -> Result<Self, IoError> {
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
            .map(Self::append_current_dir_prefix_if_need)
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
            .map(Self::append_current_dir_prefix_if_need)
            .collect::<BTreeSet<String>>();

        self.files = self
            .files()
            .difference(&paths_to_be_removed)
            .cloned()
            .collect::<BTreeSet<String>>();

        self
    }


    fn append_current_dir_prefix_if_need<S: AsRef<str>>(path: S) -> String {
        let path   = path.as_ref();
        let prefix = format!(".{}", MAIN_SEPARATOR);
        if path.starts_with(&prefix) {
            path.to_string()
        } else {
            prefix + path
        }
    }
}
