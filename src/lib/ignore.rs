use lib::setting;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{Error as IoError, Read};


pub struct Ignore;

impl Ignore {
    pub fn read() -> Result<BTreeSet<String>, IoError> {
        let mut f = try!(File::open(setting::ignore_file()));

        let mut contents = String::new();
        try!(f.read_to_string(&mut contents));

        let files = contents
            .lines()
            .map(|l| l.trim().to_string())
            .collect::<BTreeSet<String>>();

        Ok(files)
    }
}
