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
    ($($x: expr),+) => {
        PathBuf::new()
            $(.join($x))+
    };
}


pub fn append_current_dir_prefix_if_need(path: &String) -> String {
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

fn is_hidden_name(file_name: &str) -> bool {
    file_name.starts_with(".") && file_name.len() > 1 && file_name != ".."
}
#[test]
fn hidden_name() {
    let file_names = [
        ".a",
        ".ab",
        ".abc",
        "..a",
        "...",
    ];
    for file_name in &file_names {
        assert!(is_hidden_name(file_name));
    }
}
#[test]
fn non_hidden_name() {
    let file_names = [
        "",

        ".",
        "..",
        "a",
        "ab",
        "abc",
    ];
    for file_name in &file_names {
        assert!(!is_hidden_name(file_name));
    }
}

fn is_hidden(entry: &WalkDirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map_or(false, is_hidden_name)
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

fn to_string(entry: WalkDirEntry) -> Option<String> {
    entry
        .path()
        .to_str()
        .map(|p| p.to_string())
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
        .filter_map(to_string)
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
        .filter_map(to_string)
        .collect::<Vec<String>>()
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{MAIN_SEPARATOR, Path, PathBuf};

    fn to_string_unsafely(path: &PathBuf) -> String {
        path
            .to_str()
            .unwrap()
            .to_string()
    }

    #[test]
    fn create_path_buf() {
        use std::path::PathBuf;
        let paths = [
            (Path::new("path").to_path_buf()          , path_buf!["path"              ]),
            (Path::new("path").join("to")             , path_buf!["path", "to"        ]),
            (Path::new("path").join("to").join("file"), path_buf!["path", "to", "file"]),
        ];
        for &(ref correct, ref calculated) in &paths {
            assert_eq!(correct, calculated);
        }
    }

    #[test]
    fn append_current_dir_prefix() {
        let paths = [
            path_buf!["."                 ],
            path_buf![".."                ],
            path_buf!["..", "path"        ],
            path_buf!["path"              ],
            path_buf!["path", "to"        ],
            path_buf!["path", "to", "file"],
        ];
        for path in &paths {
            let path    = to_string_unsafely(path);
            let correct = format!(".{}{}", MAIN_SEPARATOR, path);
            assert_eq!(correct, append_current_dir_prefix_if_need(&path));
        }
    }
    #[test]
    fn not_append_current_dir_prefix() {
        let paths = [
            path_buf![".", "."                 ],
            path_buf![".", ".."                ],
            path_buf![".", "..", "path"        ],
            path_buf![".", "path"              ],
            path_buf![".", "path", "to"        ],
            path_buf![".", "path", "to", "file"],
        ];
        for path in &paths {
            let path    = to_string_unsafely(path);
            let correct = path.clone();
            assert_eq!(correct, append_current_dir_prefix_if_need(&path));
        }
    }
}
