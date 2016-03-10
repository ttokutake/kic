extern crate walkdir;

use constant::*;
use self::walkdir::{DirEntry as WalkDirEntry, WalkDir, WalkDirIterator};
use std::collections::BTreeSet;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::io::Error as IoError;
use std::path::{Path, PathBuf};
use std::result::Result;

macro_rules! path_buf {
    ($($x: expr),*) => {
        PathBuf::new()
            $(.join($x))*
    };
}

pub fn extract_file_name(full_path: &PathBuf) -> Option<&str> {
    full_path
        .file_name()
        .and_then(OsStr::to_str)
}

pub fn ls<P: AsRef<Path>>(path: &P) -> Result<BTreeSet<String>, IoError> {
    let dirs = try!(fs::read_dir(path));
    let dirs = dirs
        .filter_map(Result::ok)
        .map(|d| d.file_name())
        .map(OsString::into_string)
        .filter_map(Result::ok)
        .collect::<BTreeSet<String>>();
    Ok(dirs)
}

fn is_hidden(entry: &WalkDirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map_or(false, |s| s.starts_with(".") && s.len() > 1 && s != "..")
}

fn is_pinned(entry: &WalkDirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map_or(false, |s| s == WORKING_DIR_NAME || s == KEEPED_FILE_NAME)
}

fn is_target(entry: &WalkDirEntry) -> bool {
    !is_hidden(entry) && !is_pinned(entry)
}

fn to_string(entry: WalkDirEntry) -> Result<String, OsString> {
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
        .collect::<Vec<WalkDirEntry>>();

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
        .collect::<Vec<WalkDirEntry>>();

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
