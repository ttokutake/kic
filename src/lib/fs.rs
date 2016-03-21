extern crate walkdir;

use self::walkdir::{DirEntry as WalkDirEntry, WalkDir, WalkDirIterator};

use constant::{WORKING_DIR_NAME, KEEPED_FILE_NAME};
use std::collections::BTreeSet;
use std::ffi::OsString;
use std::fs;
use std::io::Error as IoError;
use std::path::{MAIN_SEPARATOR, Path};
use std::result::Result;


macro_rules! path_buf {
    ($($x: expr),*) => {
        PathBuf::new()
            $(.join($x))*
    };
}


pub fn append_prefix_if_need(path: &String) -> String {
    let prefix       = format!(".{}", MAIN_SEPARATOR);
    let prefix: &str = prefix.as_ref();
    format!("{}{}", if path.starts_with(prefix) { "" } else { prefix }, path)
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

pub fn is_empty_dir<P: AsRef<Path>>(path: P) -> bool {
    match fs::read_dir(path) {
        Ok(rd) => rd.count() == 0,
        Err(_) => false,
    }
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
