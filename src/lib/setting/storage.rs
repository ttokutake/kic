extern crate chrono;

use self::chrono::{DateTime, Duration, Local};
use self::chrono::offset::TimeZone;

use constant::STORAGE_DIR_NAME;
use lib::fs::*;
use lib::io::*;
use std::fs::{self, OpenOptions};
use std::io::{Error as IoError, ErrorKind as IoErrorKind, Write};
use std::path::{Path, PathBuf};


pub struct Storage {
    now     : DateTime<Local>,
    date    : String,
    log_file: Option<String>,
}

impl Storage {
    fn path() -> PathBuf {
        path_buf![super::working_dir(), STORAGE_DIR_NAME]
    }

    pub fn exist() -> bool {
        Self::path().is_dir()
    }

    pub fn create() -> Result<(), IoError> {
        print_with_tag(Tag::Info, format!("Create \"{}\" directory", STORAGE_DIR_NAME));

        super::create_essential_dir(Self::path())
    }


    pub fn new() -> Self {
        let now  = Local::now();
        let date = now.format("%Y-%m-%d").to_string();
        Storage { now: now, date: date, log_file: None }
    }


    fn path_to_box(&self) -> PathBuf {
        path_buf![Storage::path(), &self.date]
    }

    fn path_to_dust_box(&self) -> PathBuf {
        path_buf![self.path_to_box(), "dusts"]
    }

    fn create_box(&self) -> Result<(), IoError> {
        print_with_tag(Tag::Info, format!(r#"Create "{}" directory in "{}""#, self.date, Self::path().display()));

        fs::create_dir_all(self.path_to_dust_box())
    }

    fn path_to_log(&self) -> PathBuf {
        let file_name = match &self.log_file {
            &Some(ref s) => s,
            &None        => unreachable!("Wrong to use path_to_log()"),
        };
        path_buf![self.path_to_box(), &file_name]
    }

    fn write_log<S: AsRef<str>>(&self, content: S) -> Result<usize, IoError> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.path_to_log());
        let mut file = try!(file);

        file.write(format!("{}\n", content.as_ref()).as_bytes())
    }

    fn start_mark_for_log(&self) -> String {
        let datetime = self.now.format("%H:%M:%S").to_string();
        format!(
            "############
# {} #
############",
            datetime,
        )
    }

    fn create_log_file(mut self, file_name: &str) -> Result<Self, IoError> {
        let file_name = format!("{}.log", file_name);

        print_with_tag(Tag::Info, format!(r#"Create "{}" file in "{}""#, file_name, self.path_to_box().display()));

        self.log_file = Some(file_name);

        try!(self.write_log(self.start_mark_for_log()));

        Ok(self)
    }

    pub fn create_box_with_log(self, file_name: &str) -> Result<Self, IoError> {
        try!(self.create_box());
        self.create_log_file(file_name)
    }


    pub fn squeeze_dusts<P: AsRef<Path>>(&self, paths_to_dust: Vec<P>) -> Result<(), IoError> {
        let path_to_dust_box = self.path_to_dust_box();

        let message = format!("Move dusts to \"{}\"", path_to_dust_box.display());
        try!(self.print_and_log(message));

        for path_to_dust in &paths_to_dust {
            let path_to_dust = path_to_dust.as_ref();
            let target_file  = match path_to_dust.file_name() {
                Some(f) => f,
                None    => unreachable!("Cannot get file name from path!!"),
            };
            let target_base = match path_to_dust.parent() {
                Some(b) => b,
                None    => unreachable!("Cannot get base name from path!!"),
            };

            let to = path_buf![&path_to_dust_box, target_base];

            let message = format!("  => \"{}\"", path_to_dust.display());
            try!(self.print_and_log(message));

            try!(fs::create_dir_all(&to));

            // forcedly overwrite if the file exists with same name.
            match fs::rename(path_to_dust, path_buf![to, target_file]) {
                Ok(_)  => (),
                Err(e) => match e.kind() {
                    IoErrorKind::PermissionDenied => try!(self.print_and_log("     Interrupted for permission")),
                    _                             => return Err(e),
                },
            };
        }

        Ok(())
    }

    pub fn squeeze_empty_dirs_only<P: AsRef<Path>>(&self, paths_to_dir: Vec<P>) -> Result<(), IoError> {
        let path_to_dust_box = self.path_to_dust_box();

        let message = format!("Move empty dirs to \"{}\"", path_to_dust_box.display());
        try!(self.print_and_log(message));

        for path_to_dir in &paths_to_dir {
            if is_empty_dir(path_to_dir) {
                let path_to_dir = path_to_dir.as_ref();

                let message = format!("  => \"{}\"", path_to_dir.display());
                try!(self.print_and_log(message));

                match fs::remove_dir(path_to_dir) {
                    Ok(_)  => try!(fs::create_dir_all(path_buf![&path_to_dust_box, path_to_dir])),
                    Err(e) => match e.kind() {
                        IoErrorKind::PermissionDenied => try!(self.print_and_log("     Interrupted for permission")),
                        _                             => return Err(e),
                    },
                };
            }
        }

        Ok(())
    }


    pub fn delete_expired_boxes(&self, moratorium: Duration) -> Result<(), IoError> {
        let path_to_storage = Self::path();

        try!(self.print_and_log("Delete expired dusts"));

        let target_boxes = try!(ls(&path_to_storage))
            .into_iter()
            .filter(|date| match Local.datetime_from_str(format!("{} 00:00:00", date).as_ref(), "%Y-%m-%d %H:%M:%S") {
                Ok(created_date) => created_date + moratorium < self.now,
                Err(_)           => false,
            })
            .map(|dir| path_to_storage.join(dir))
            .collect::<Vec<PathBuf>>();

        for target_box in &target_boxes {
            let message = format!("  => \"{}\"", target_box.display());
            try!(self.print_and_log(message));
            try!(fs::remove_dir_all(target_box));
        };

        Ok(())
    }


    fn print_and_log<S: AsRef<str>>(&self, message: S) -> Result<(), IoError> {
        print_with_tag(Tag::Info, message.as_ref());

        if self.log_file.is_some() {
            try!(self.write_log(message));
        }

        Ok(())
    }
}
