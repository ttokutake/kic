use lib::fs::*;
use lib::setting;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{Error as IoError, Read};


pub struct Ignore {
    files: BTreeSet<String>,
}

impl Ignore {
    pub fn to_string(&self) -> String {
        self.files
            .iter()
            .fold(String::new(), |contents, file| contents + file + "\n")
    }

    fn new(files: BTreeSet<String>) -> Self {
        Ignore { files: files }
    }

    pub fn default() -> Self {
        let current_files = walk_dir(".");

        Self::new(current_files)
    }

    pub fn read() -> Result<Self, IoError> {
        let mut f = try!(File::open(setting::ignore_file()));

        let mut contents = String::new();
        try!(f.read_to_string(&mut contents));

        let files = contents
            .lines()
            .map(|l| l.trim().to_string())
            .collect::<BTreeSet<String>>();

        Ok(Self::new(files))
    }

    pub fn files(&self) -> &BTreeSet<String> {
        &self.files
    }
}
