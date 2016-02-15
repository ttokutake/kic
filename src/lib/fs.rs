extern crate walkdir;

use constant::*;
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
        Some(osstr) => {
            match osstr.to_str() {
                Some(file_name) => file_name,
                None            => panic!("ERROR: Use UTF-8 characters as file name."),
            }
        },
        None => panic!("ERROR: Invalid path."),
    }
}

pub fn extract_base(full_path: &PathBuf) -> &Path {
    match full_path.parent() {
        Some(p) => p,
        None    => panic!("ERROR: Cannot extract base name."),
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map_or(false, |s| s.starts_with(".") && s.len() > 1 && s != "..")
}

fn is_pinned(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map_or(false, |s| s == WORKING_DIR_NAME || s == KEEPED_FILE_NAME)
}

pub fn walk_dir<P: AsRef<Path>>(root: P) -> BTreeSet<String> {
    let walker = WalkDir::new(root)
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
        .filter_entry(|e| !is_pinned(e))
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
