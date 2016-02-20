extern crate walkdir;

use constant::*;
use self::walkdir::{WalkDir, WalkDirIterator};
use std::collections::BTreeSet;
use std::ffi::OsString;
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

fn is_hidden(entry: &walkdir::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map_or(false, |s| s.starts_with(".") && s.len() > 1 && s != "..")
}

fn is_pinned(entry: &walkdir::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map_or(false, |s| s == WORKING_DIR_NAME || s == KEEPED_FILE_NAME)
}

fn is_target(entry: &walkdir::DirEntry) -> bool {
    !is_hidden(entry) && !is_pinned(entry)
}

fn to_string(entry: walkdir::DirEntry) -> Result<String, OsString> {
    entry
        .path()
        .as_os_str()
        .to_os_string()
        .into_string()
}

pub fn walk_dir<P: AsRef<Path>>(root: P) -> BTreeSet<String> {
    let walker = WalkDir::new(root)
        .into_iter()
        .filter_entry(is_target)
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
        .collect::<Vec<walkdir::DirEntry>>();

    walker
        .into_iter()
        .map(to_string)
        .filter_map(Result::ok)
        .collect::<BTreeSet<String>>()
}

pub fn dirs_ordered_by_descending_depth<P: AsRef<Path>>(root: P) -> Vec<String> {
    let mut walker = WalkDir::new(root)
        .into_iter()
        .filter_entry(is_target)
        .filter_entry(|e| e.file_type().is_dir())
        .filter_map(Result::ok)
        .collect::<Vec<walkdir::DirEntry>>();

    walker.sort_by(|l, r| {
        let l_depth = &l.depth();
        let r_depth = &r.depth();
        r_depth.cmp(l_depth)
    });

    walker
        .into_iter()
        .map(to_string)
        .filter_map(Result::ok)
        .collect::<Vec<String>>()
}
