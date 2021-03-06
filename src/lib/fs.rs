extern crate chrono;
extern crate walkdir;

use self::chrono::{Duration, UTC};
use self::walkdir::{DirEntry as WalkDirEntry, WalkDir, WalkDirIterator};

use std::borrow::Borrow;
use std::collections::{BTreeSet, VecDeque};
use std::fs::{self, DirEntry};
use std::io::Error as IoError;
use std::os::unix::fs::MetadataExt;
use std::path::{Component, MAIN_SEPARATOR, Path, PathBuf};
use std::result::Result;


macro_rules! path_buf {
    ($($x: expr),+) => ({
        let mut path = PathBuf::new();
        $(path.push($x);)+
        path
    });
}


fn is_hidden_name(file_name: &str) -> bool {
    file_name.starts_with(".") && file_name.len() > 1 && file_name != ".."
}

trait DirEntryExt {
    fn file_name_string(&self) -> Option<String> {
        unimplemented!();
    }

    fn is_hidden(&self) -> bool;
}
impl DirEntryExt for DirEntry {
    fn file_name_string(&self) -> Option<String> {
        self
            .file_name()
            .into_string()
            .ok()
    }

    fn is_hidden(&self) -> bool {
        self
            .file_name()
            .to_str()
            .map_or(false, is_hidden_name)
    }
}
impl DirEntryExt for WalkDirEntry {
    fn is_hidden(&self) -> bool {
        self
            .file_name()
            .to_str()
            .map_or(false, is_hidden_name)
    }
}


pub fn supply_current_dir_prefix<S: AsRef<str>>(path_name: S) -> String {
    let path_name = path_name.as_ref();

    match Path::new(path_name).components().next() {
        Some(entry) => match entry {
            Component::CurDir | Component::RootDir | Component::Prefix(_) => path_name.to_string(),
            _                                                             => format!(".{}{}", MAIN_SEPARATOR, path_name),
        },
        None => unreachable!("Wrong to use this function!!"),
    }
}

pub fn trim_end_separator<S: AsRef<str>>(path_name: S) -> String {
    path_name
        .as_ref()
        .trim_right_matches(MAIN_SEPARATOR)
        .to_string()
}

pub fn la<P: AsRef<Path>>(path: P) -> Result<Vec<String>, IoError> {
    let dirs = try!(fs::read_dir(path));
    let dirs = dirs
        .filter_map(Result::ok)
        .filter_map(|e| e.file_name_string())
        .collect::<Vec<String>>();
    Ok(dirs)
}

#[cfg(unix)]
pub fn is_recently_accessed<P: AsRef<Path>, D: Borrow<Duration>>(p: P, moratorium: D) -> bool {
    let threshold = UTC::now() - *moratorium.borrow();

    let accessed_time = match p.as_ref().metadata() {
        Ok(m)  => m.atime(),
        Err(_) => unreachable!("Wrong to use this function!!"),
    };

    accessed_time > threshold.timestamp()
}
#[cfg(windows)]
pub fn is_recently_accessed<P: AsRef<Path>, D: Borrow<Duration>>(_p: P, _moratorium: D) -> bool {
    false
}

pub fn walk_dir<P: AsRef<Path>>(root: P) -> BTreeSet<String> {
    let walker = WalkDir::new(root)
        .into_iter()
        .filter_entry(|e| !e.is_hidden())
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
        .collect::<Vec<WalkDirEntry>>();

    walker
        .into_iter()
        .filter_map(|e| e.path().to_str().map(|p| p.to_string()))
        .collect::<BTreeSet<String>>()
}

pub fn potentially_empty_dirs<P: AsRef<Path>>(root: P, phantom_entries: Vec<PathBuf>) -> BTreeSet<PathBuf> {
    fn potentially_empty_dirs(mut result: BTreeSet<PathBuf>, mut target_dirs: VecDeque<PathBuf>, phantom_entries: Vec<PathBuf>) -> BTreeSet<PathBuf> {
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
                    .filter(|e| phantom_entries.iter().all(|pe| *pe != e.path()))
                    .any(|e| e.file_type().ok().map_or(true, |t| t.is_file()) || e.is_hidden());

                if ignore || include_file_or_hidden_dir {
                    loop {
                        if !(result.remove(&target_dir) && target_dir.pop()) {
                            break;
                        }
                    }
                }

                let dirs = entries
                    .iter()
                    .filter(|e| e.file_type().ok().map_or(false, |t| t.is_dir()) && !e.is_hidden())
                    .map(|e| e.path())
                    .collect::<BTreeSet<PathBuf>>();
                for dir in dirs.clone().into_iter() {
                    result.insert(dir);
                }

                let mut dirs = dirs.into_iter().collect::<VecDeque<PathBuf>>();
                target_dirs.append(&mut dirs);

                potentially_empty_dirs(result, target_dirs, phantom_entries)
            },
        }
    }

    let mut result     : BTreeSet<PathBuf> = BTreeSet::new();
    let mut target_dirs: VecDeque<PathBuf> = VecDeque::new();

    let root = root.as_ref().to_path_buf();
    result.insert(root.clone());
    target_dirs.push_back(root);

    potentially_empty_dirs(result, target_dirs, phantom_entries)
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
    fn supply_current_dir_prefix_should_return_added() {
        let path_names = [
            ("./a"  , "a"  ),
            ("./a/" , "a/" ),
            ("./a/b", "a/b"),
        ];
        for &(correct, input) in &path_names {
            assert_eq!(correct.to_string(), supply_current_dir_prefix(input));
        }
    }
    #[test]
    fn supply_current_dir_prefix_should_return_as_it_is() {
        let path_names = [
            "/",
            "/a",
            ".",
            "./",
            "./a",
            "././a",
        ];
        for path_name in &path_names {
            assert_eq!(path_name.to_string(), supply_current_dir_prefix(path_name))
        }
    }
    #[test]
    #[should_panic(expect = "entered unreachable code")]
    fn supply_current_dir_prefix_should_panic() {
        supply_current_dir_prefix("");
    }

    #[test]
    fn trim_end_separator_should_return_trimmed() {
        let path_names = [
            ("a"  , "a/"   ),
            ("a"  , "a//"  ),
            ("./a", "./a/" ),
            ("./a", "./a//"),
        ];
        for &(correct, input) in &path_names {
            assert_eq!(correct.to_string(), trim_end_separator(input));
        }
    }
    #[test]
    fn trim_end_separator_should_return_as_it_is() {
        let path_names = [
            "a",
            "./a",
            "./a/b",
        ];
        for path_name in &path_names {
            assert_eq!(path_name.to_string(), trim_end_separator(path_name));
        }
    }

    #[test]
    fn la_should_return_ok() {
        fn to_btree_set(vec: Vec<String>) -> BTreeSet<String> {
            vec.into_iter().collect::<BTreeSet<String>>()
        }

        let helper = Helper::new("la_Ok");
        helper.create_dirs_and_files();

        let data_set = vec![
            (vec![helper.d2.clone(), helper.d5.clone(), helper.d6.clone(), helper.f1.clone()], helper.path_to_d1()),
            (vec![helper.d3.clone(), helper.f2.clone()]                                      , helper.path_to_d2()),
            (vec![helper.d4.clone(), helper.f3.clone()]                                      , helper.path_to_d3()),
            (Vec::new()                                                                      , helper.path_to_d4()),
            (vec![helper.f4.clone()]                                                         , helper.path_to_d5()),
            (Vec::new()                                                                      , helper.path_to_d6()),
        ];

        for (correct, target_dir) in data_set.into_iter() {
            assert_eq!(to_btree_set(correct), to_btree_set(la(target_dir).unwrap()));
        }

        helper.remove_dirs_and_files();
    }

    #[test]
    fn walk_dir_should_return_b_tree_set() {
        let helper = Helper::new("walk_dir_BTreeSet");
        helper.create_dirs_and_files();

        let mut correct = BTreeSet::new();
        let files = [helper.path_to_f1(), helper.path_to_f2(), helper.path_to_f3()];
        for file in &files {
            correct.insert(file.to_str().unwrap().to_string());
        }

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
        let mut phantom_files = Vec::new();
        let data_set = vec![
            (helper.path_to_d3(), helper.path_to_f3()),
            (helper.path_to_d2(), helper.path_to_f2()),
            (helper.path_to_d5(), helper.path_to_f4()),
        ];
        for (part_of_correct, phantom_file) in data_set.into_iter() {
            correct.insert(part_of_correct);
            phantom_files.push(phantom_file);
            assert_eq!(correct, potentially_empty_dirs(&root, phantom_files.clone()));
        }

        correct.clear();
        correct.insert(helper.path_to_d4());
        let data_set = vec![
            (helper.path_to_d4(), None),
            (helper.path_to_d3(), Some(helper.path_to_f3())),
            (helper.path_to_d2(), Some(helper.path_to_f2())),
            (helper.path_to_d5(), Some(helper.path_to_f4())),
        ];
        for (part_of_correct, removed_file) in data_set.into_iter() {
            correct.insert(part_of_correct);
            if let Some(file) = removed_file {
                fs::remove_file(file).ok();
            }
            assert_eq!(correct, potentially_empty_dirs(&root, Vec::new()));
        }

        helper.remove_dirs_and_files();
    }
}
