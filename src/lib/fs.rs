use std::collections::BTreeSet;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::result::Result;

macro_rules! path_buf {
    ($($x: expr),*) => {
        PathBuf::new()
            $(.join($x))*
    }
}

pub fn walk_dir<P: AsRef<Path>>(path: P) -> BTreeSet<String> {
    fn walk_dir(path: PathBuf) -> Vec<OsString> {
        let paths = match fs::read_dir(path) {
            Ok(rd)   => rd,
            Err(why) => panic!("{:?}", why),
        };
        paths
            .filter_map(Result::ok)
            .flat_map(|de| {
                let path = de.path();
                if is_hidden(&path) {
                    Vec::new()
                } else if path.is_file() {
                    vec![path.into_os_string()]
                } else {
                    walk_dir(path)
                }
            })
            .collect::<Vec<OsString>>()
    }

    walk_dir(path.as_ref().to_path_buf()).into_iter()
        .map(|f| f.into_string())
        .filter_map(Result::ok)
        .collect::<BTreeSet<String>>()
}

fn is_hidden(path: &PathBuf) -> bool {
    let file_name = path.file_name().and_then(|f| f.to_str());
    file_name.map_or(false, |f| f.starts_with(".") && f.len() > 1 && f != "..")
}
