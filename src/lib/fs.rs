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

    use std::collections::BTreeSet;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::{Path, PathBuf};


    struct Helper {
        d1: String,
        d2: String,
        d3: String,
        f1: String,
        f2: String,
    }
    impl Helper {
        fn new(suffix: &str) -> Self {
            Helper {
                d1: format!("directory1_{}", suffix),
                d2: format!("directory2_{}", suffix),
                d3: format!("directory3_{}", suffix),
                f1: format!("file1_{}"     , suffix),
                f2: format!("file2_{}"     , suffix),
            }
        }

        fn path_to_d1(&self) -> PathBuf {
            PathBuf::new().join(&self.d1)
        }
        fn path_to_d2(&self) -> PathBuf {
            self.path_to_d1().join(&self.d2)
        }
        fn path_to_d3(&self) -> PathBuf {
            self.path_to_d2().join(&self.d3)
        }

        fn path_to_f1(&self) -> PathBuf {
            self.path_to_d1().join(&self.f1)
        }
        fn path_to_f2(&self) -> PathBuf {
            self.path_to_d2().join(&self.f2)
        }

        fn remove_dirs_and_files(&self) {
            fs::remove_dir_all(self.path_to_d1()).ok();
        }

        fn create_dirs_and_files(&self) {
            self.remove_dirs_and_files();

            fs::create_dir_all(self.path_to_d3()).ok();

            let mut f = File::create(self.path_to_f1()).unwrap();
            f.write("\n".as_ref()).ok();
            fs::copy(self.path_to_f1(), self.path_to_f2()).ok();
        }

        fn to_string_forcely(path: PathBuf) -> String {
            path.to_str().unwrap().to_string()
        }
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
        let helper = Helper::new("ls_Ok");
        helper.create_dirs_and_files();

        let empty_vec: Vec<String> = Vec::new();

        assert_eq!(vec![helper.d2.clone(), helper.f1.clone()], ls(helper.path_to_d1()).unwrap());
        assert_eq!(vec![helper.d3.clone(), helper.f2.clone()], ls(helper.path_to_d2()).unwrap());
        assert_eq!(empty_vec                                 , ls(helper.path_to_d3()).unwrap());

        helper.remove_dirs_and_files();
    }

    #[test]
    fn is_empty_dir_should_return_true() {
        let helper = Helper::new("is_empty_dir_true");
        helper.create_dirs_and_files();

        assert!(is_empty_dir(helper.path_to_d3()));

        helper.remove_dirs_and_files();
    }
    #[test]
    fn is_empty_dir_should_return_false() {
        let helper = Helper::new("is_empty_dir_false");
        helper.create_dirs_and_files();

        assert!(!is_empty_dir(helper.path_to_d1()));
        assert!(!is_empty_dir(helper.path_to_d2()));

        helper.remove_dirs_and_files();
    }

    #[test]
    fn walk_dir_should_return_b_tree_set() {
        let helper = Helper::new("walk_dir_BTreeSet");
        helper.create_dirs_and_files();

        let mut correct = BTreeSet::new();
        correct.insert(Helper::to_string_forcely(helper.path_to_f1()));
        correct.insert(Helper::to_string_forcely(helper.path_to_f2()));

        assert_eq!(correct, walk_dir(helper.path_to_d1()));

        helper.remove_dirs_and_files();
    }

    #[test]
    fn dirs_ordered_by_descending_depth_should_return_vec() {
        let helper = Helper::new("dirs_ordered_by_descending_depth_Vec");
        helper.create_dirs_and_files();

        let correct = vec![helper.path_to_d3(), helper.path_to_d2(), helper.path_to_d1()];

        assert_eq!(correct, dirs_ordered_by_descending_depth(helper.path_to_d1()));

        helper.remove_dirs_and_files();
    }
}
