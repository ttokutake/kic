extern crate walkdir;

use self::walkdir::{DirEntry as WalkDirEntry, WalkDir, WalkDirIterator};

use std::collections::BTreeSet;
use std::ffi::OsString;
use std::fs;
use std::io::Error as IoError;
use std::path::{Path, PathBuf};
use std::result::Result;


macro_rules! path_buf {
    ($($x: expr),+) => {
        PathBuf::new()
            $(.join($x))+
    };
}


pub fn ls<P: AsRef<Path>>(path: P) -> Result<Vec<String>, IoError> {
    let dirs = try!(fs::read_dir(path));
    let dirs = dirs
        .filter_map(Result::ok)
        .map(|d| d.file_name())
        .map(OsString::into_string)
        .filter_map(Result::ok)
        .collect::<Vec<String>>();
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
fn is_hidden_name_should_return_true() {
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
fn is_hidden_name_should_return_false() {
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

pub fn walk_dir<P: AsRef<Path>>(root: P) -> BTreeSet<String> {
    let walker = WalkDir::new(root)
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
        .collect::<Vec<WalkDirEntry>>();

    walker
        .into_iter()
        .filter_map(|e| e.path().to_str().map(|s| s.to_string()))
        .collect::<BTreeSet<String>>()
}

pub fn dirs_ordered_by_descending_depth<P: AsRef<Path>>(root: P) -> Vec<PathBuf> {
    let mut walker = WalkDir::new(root)
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
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
        .map(|e| e.path().to_path_buf())
        .collect::<Vec<PathBuf>>()
}


#[cfg(test)]
mod tests {
    use super::*;

    use std::fs::{self, File};
    use std::io::Write;
    use std::path::{Path, PathBuf};


    const D1: &'static str = "directory1";
    const D2: &'static str = "directory2";
    const D3: &'static str = "directory3";
    const F1: &'static str = "file1";
    const F2: &'static str = "file2";
    const F3: &'static str = "file3";

    fn remove_dirs_and_files() {
        let path_to_d1 = Path::new(D1);
        let path_to_f1 = Path::new(F1);

        fs::remove_dir_all(path_to_d1).ok();
        if path_to_f1.is_file() {
            fs::remove_file(path_to_f1).ok();
        }
    }

    fn create_dirs_and_files() {
        remove_dirs_and_files();

        let path_to_d1 = Path::new(D1);
        let path_to_d2 = path_to_d1.join(D2);
        let path_to_d3 = path_to_d2.join(D3);
        let path_to_f1 = Path::new(F1);
        let path_to_f2 = path_to_d1.join(F2);
        let path_to_f3 = path_to_d2.join(F3);

        fs::create_dir_all(path_to_d3).ok();

        let mut f = File::create(path_to_f1).unwrap();
        f.write("\n".as_ref()).ok();
        fs::copy(path_to_f1, path_to_f2).ok();
        fs::copy(path_to_f1, path_to_f3).ok();
    }


    #[test]
    fn path_buf_macro_should_create_path_buf() {
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
    fn ls_should_return_ok() {
    }
    #[test]
    fn ls_should_return_err() {
    }

    #[test]
    fn is_empty_dir_should_return_true() {
    }
    #[test]
    fn is_empty_dir_should_return_false() {
    }

    #[test]
    fn walk_dir_should_return_b_tree_set() {
    }

    #[test]
    fn dirs_ordered_by_descending_depth_should_return_vec() {
    }
}
