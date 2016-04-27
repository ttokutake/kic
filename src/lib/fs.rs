extern crate chrono;
extern crate walkdir;

use self::chrono::{Duration, UTC};
use self::walkdir::{DirEntry as WalkDirEntry, WalkDir, WalkDirIterator};

use std::borrow::Borrow;
use std::collections::{BTreeSet, VecDeque};
use std::fs::{self, DirEntry};
use std::io::Error as IoError;
use std::os::unix::fs::MetadataExt;
use std::path::{MAIN_SEPARATOR, Path, PathBuf};
use std::result::Result;


macro_rules! path_buf {
    ($($x: expr),+) => ({
        let mut path = PathBuf::new();
        $(path.push($x);)+
        path
    });
}


pub fn trim_current_dir_prefix<S: AsRef<str>>(path_name: S) -> String {
    let pattern = format!(".{}", MAIN_SEPARATOR);

    path_name
        .as_ref()
        .trim_left_matches(&pattern)
        .to_string()
}

fn into_string<D: Borrow<DirEntry>>(entry: D) -> Option<String> {
    entry
        .borrow()
        .file_name()
        .into_string()
        .ok()
}

pub fn la<P: AsRef<Path>>(path: P) -> Result<Vec<String>, IoError> {
    let dirs = try!(fs::read_dir(path));
    let dirs = dirs
        .filter_map(Result::ok)
        .filter_map(into_string)
        .collect::<Vec<String>>();
    Ok(dirs)
}

pub fn is_recently_accessed<P: AsRef<Path>, D: Borrow<Duration>>(p: P, moratorium: D) -> bool {
    let threshold = UTC::now() - *moratorium.borrow();

    let accessed_time = match p.as_ref().metadata() {
        Ok(m)  => m.atime(),
        Err(_) => unreachable!("Wrong to use this function!!"),
    };

    accessed_time > threshold.timestamp()
}

fn is_hidden_name<S: AsRef<str>>(file_name: S) -> bool {
    let file_name = file_name.as_ref();
    file_name.starts_with(".") && file_name.len() > 1 && file_name != ".."
}

fn is_hidden_for_std(entry: &DirEntry) -> bool {
    into_string(entry).map_or(false, is_hidden_name)
}

fn is_hidden_for_walkdir(entry: &WalkDirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map_or(false, is_hidden_name)
}

pub fn walk_dir<P: AsRef<Path>>(root: P) -> BTreeSet<String> {
    let walker = WalkDir::new(root)
        .into_iter()
        .filter_entry(|e| !is_hidden_for_walkdir(e))
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
        .collect::<Vec<WalkDirEntry>>();

    walker
        .into_iter()
        .filter_map(|e| e.path().to_str().map(trim_current_dir_prefix))
        .collect::<BTreeSet<String>>()
}

pub fn potentially_empty_dirs<P: AsRef<Path>>(root: P) -> BTreeSet<PathBuf> {
    fn potentially_empty_dirs(mut result: BTreeSet<PathBuf>, mut target_dirs: VecDeque<PathBuf>) -> BTreeSet<PathBuf> {
        match target_dirs.pop_front() {
            None => result,
            Some(mut target_dir) => {
                let read_result = fs::read_dir(&target_dir);
                let ignore      = read_result.is_err();

                let entries = match read_result {
                    Ok(rd) => rd
                        .filter_map(Result::ok)
                        .collect::<Vec<DirEntry>>(),
                    Err(_) => Vec::new(),
                };
                let include_file_or_hidden_dir = entries
                    .iter()
                    .any(|e| e.file_type().ok().map_or(true, |t| t.is_file()) || is_hidden_for_std(e));

                if ignore || include_file_or_hidden_dir {
                    loop {
                        if !(result.remove(&target_dir) && target_dir.pop()) {
                            break;
                        }
                    }
                }

                let dirs = entries
                    .iter()
                    .filter(|e| e.file_type().ok().map_or(false, |t| t.is_dir()) && !is_hidden_for_std(e))
                    .map(|e| e.path())
                    .collect::<BTreeSet<PathBuf>>();
                for dir in dirs.clone().into_iter() {
                    result.insert(dir);
                }

                let mut dirs = dirs.into_iter().collect::<VecDeque<PathBuf>>();
                target_dirs.append(&mut dirs);

                potentially_empty_dirs(result, target_dirs)
            },
        }
    }

    let mut result     : BTreeSet<PathBuf> = BTreeSet::new();
    let mut target_dirs: VecDeque<PathBuf> = VecDeque::new();

    let root = root.as_ref().to_path_buf();
    result.insert(root.clone());
    target_dirs.push_back(root);

    potentially_empty_dirs(result, target_dirs)
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
        d4: String,
        d5: String,
        d6: String,
        f1: String,
        f2: String,
        f3: String,
        f4: String,
    }
    impl Helper {
        fn new(suffix: &str) -> Self {
            Helper {
                d1: format!("directory1_{}" , suffix),
                d2: format!("directory2_{}" , suffix),
                d3: format!("directory3_{}" , suffix),
                d4: format!("directory4_{}" , suffix),
                d5: format!("directory5_{}" , suffix),
                d6: format!(".directory6_{}", suffix),
                f1: format!("file1_{}"      , suffix),
                f2: format!("file2_{}"      , suffix),
                f3: format!("file3_{}"      , suffix),
                f4: format!(".file4_{}"     , suffix),
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
        fn path_to_d4(&self) -> PathBuf {
            self.path_to_d3().join(&self.d4)
        }
        fn path_to_d5(&self) -> PathBuf {
            self.path_to_d1().join(&self.d5)
        }
        fn path_to_d6(&self) -> PathBuf {
            self.path_to_d1().join(&self.d6)
        }

        fn path_to_f1(&self) -> PathBuf {
            self.path_to_d1().join(&self.f1)
        }
        fn path_to_f2(&self) -> PathBuf {
            self.path_to_d2().join(&self.f2)
        }
        fn path_to_f3(&self) -> PathBuf {
            self.path_to_d3().join(&self.f3)
        }
        fn path_to_f4(&self) -> PathBuf {
            self.path_to_d5().join(&self.f4)
        }

        fn remove_dirs_and_files(&self) {
            fs::remove_dir_all(self.path_to_d1()).ok();
        }

        fn create_dirs_and_files(&self) {
            self.remove_dirs_and_files();

            fs::create_dir_all(self.path_to_d4()).ok();
            fs::create_dir_all(self.path_to_d5()).ok();
            fs::create_dir_all(self.path_to_d6()).ok();

            let mut f = File::create(self.path_to_f1()).unwrap();
            f.write("\n".as_ref()).ok();
            fs::copy(self.path_to_f1(), self.path_to_f2()).ok();
            fs::copy(self.path_to_f1(), self.path_to_f3()).ok();
            fs::copy(self.path_to_f1(), self.path_to_f4()).ok();
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
    fn trim_current_dir_prefix_should_return_trimmed() {
        let path_names = [
            (""   , "./"    ),
            (""   , "././"  ),
            (""   , "./././"),
            ("a"  , "./a"   ),
            ("a/" , "./a/"  ),
            ("a/b", "./a/b" ),
        ];
        for &(ref correct, ref input) in &path_names {
            assert_eq!(correct.to_string(), trim_current_dir_prefix(input));
        }
    }
    #[test]
    fn trim_current_dir_prefix_should_return_as_it_is() {
        let path_names = [
            "",
            "/",
            "/a",
            "/a/b",
            "a",
            "a/",
            "a/b",
        ];
        for path_name in &path_names {
            assert_eq!(path_name.to_string(), trim_current_dir_prefix(path_name));
        }
    }

    #[test]
    fn la_should_return_ok() {
        let helper = Helper::new("la_Ok");
        helper.create_dirs_and_files();

        let empty_vec: Vec<String> = Vec::new();

        assert_eq!(vec![helper.f1.clone(), helper.d5.clone(), helper.d2.clone(), helper.d6.clone()], la(helper.path_to_d1()).unwrap());
        assert_eq!(vec![helper.d3.clone(), helper.f2.clone()]                                      , la(helper.path_to_d2()).unwrap());
        assert_eq!(vec![helper.d4.clone(), helper.f3.clone()]                                      , la(helper.path_to_d3()).unwrap());
        assert_eq!(empty_vec                                                                       , la(helper.path_to_d4()).unwrap());
        assert_eq!(vec![helper.f4.clone()]                                                         , la(helper.path_to_d5()).unwrap());
        assert_eq!(empty_vec                                                                       , la(helper.path_to_d6()).unwrap());

        helper.remove_dirs_and_files();
    }

    #[test]
    fn walk_dir_should_return_b_tree_set() {
        let helper = Helper::new("walk_dir_BTreeSet");
        helper.create_dirs_and_files();

        let mut correct = BTreeSet::new();
        correct.insert(Helper::to_string_forcely(helper.path_to_f1()));
        correct.insert(Helper::to_string_forcely(helper.path_to_f2()));
        correct.insert(Helper::to_string_forcely(helper.path_to_f3()));

        assert_eq!(correct, walk_dir(helper.path_to_d1()));

        helper.remove_dirs_and_files();
    }

    #[test]
    fn potentially_empty_dirs_should_return_btree_set() {
        let helper = Helper::new("potentially_empty_dirs_BTreeSet");
        helper.create_dirs_and_files();

        let root = helper.path_to_d1();
        let mut correct = BTreeSet::new();

        correct.insert(helper.path_to_d4());
        assert_eq!(correct, potentially_empty_dirs(&root));

        fs::remove_file(helper.path_to_f3()).ok();
        correct.insert(helper.path_to_d3());
        assert_eq!(correct, potentially_empty_dirs(&root));

        fs::remove_file(helper.path_to_f2()).ok();
        correct.insert(helper.path_to_d2());
        assert_eq!(correct, potentially_empty_dirs(&root));

        fs::remove_file(helper.path_to_f4()).ok();
        correct.insert(helper.path_to_d5());
        assert_eq!(correct, potentially_empty_dirs(&root));

        helper.remove_dirs_and_files();
    }
}
