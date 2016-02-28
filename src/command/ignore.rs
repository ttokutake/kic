use super::Command;

pub struct Ignore {
    pub command: Option<String>,
    pub paths  : Vec<String>,
}

impl Command for Ignore {
    fn validation(&self) -> bool { true }

    fn help_message(&self) -> &'static str {
        return "ignore!";
    }

    fn main(&self) {
        match self.command {
            Some(ref c) => match c.as_ref() {
                "add"     => add(self),
                "remove"  => remove(self),
                "current" => ignore_current_files(),
                "clear"   => clear_ignore_file(),
                _         => self.help(),
            },
            None => self.help(),
        }
    }
}


fn add(ignore: &Ignore) {
    let paths = &ignore.paths;
    if paths.len() == 0 {
        return ignore.help();
    }
    println!("do add command with {:?}", paths);
}

fn remove(ignore: &Ignore) {
    let paths = &ignore.paths;
    if paths.len() == 0 {
        return ignore.help();
    }
    println!("do remove command with {:?}", paths);
}

fn ignore_current_files() {
    println!("do current command");
}

fn clear_ignore_file() {
    println!("do clear command");
}
