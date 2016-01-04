use std::collections::BTreeSet;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};

pub fn walk_dir<P: AsRef<Path>>(path: P) -> BTreeSet<String> {
    fn walk_dir(path: PathBuf) -> Vec<OsString> {
        let paths = match fs::read_dir(path) {
            Ok(rd)   => rd,
            Err(why) => panic!("{:?}", why),
        };
        paths
            .filter(|de| de.is_ok())
            .flat_map(|de| {
                let path = de.unwrap().path();
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

    walk_dir(path.as_ref().to_path_buf()).iter()
        .map(|f| f.clone().into_string())
        .filter(|f| f.is_ok())
        .map(|f| f.unwrap())
        .collect::<BTreeSet<String>>()
}

fn is_hidden(path: &PathBuf) -> bool {
    let file_name = path.file_name().and_then(|f| f.to_str());
    file_name.map_or(false, |f| f.starts_with(".") && f.len() > 1 && f != "..")
}
