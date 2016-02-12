extern crate walkdir;

use self::walkdir::{DirEntry, WalkDir, WalkDirIterator};
use std::collections::BTreeSet;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::result::Result;

macro_rules! path_buf {
    ($($x: expr),*) => {
        PathBuf::new()
            $(.join($x))*
    };
}

pub fn extract_file_name(full_path: &PathBuf) -> &str {
    match full_path.file_name() {
        Some(option_f) => {
            match option_f.to_str() {
                Some(file_name) => file_name,
                None            => panic!("ERROR: Use UTF-8 characters as file name."),
            }
        },
        None => panic!("ERROR: Invalid path."),
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map_or(false, |s| s.starts_with(".") && s.len() > 1 && s != "..")
}

pub fn walk_dir<P: AsRef<Path>>(root: P) -> BTreeSet<String> {
    let walker = WalkDir::new(root)
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
        .collect::<Vec<DirEntry>>();

    walker
        .iter()
        .map(DirEntry::path)
        .map(Path::as_os_str)
        .map(OsStr::to_os_string)
        .map(OsString::into_string)
        .filter_map(Result::ok)
        .collect::<BTreeSet<String>>()
}
