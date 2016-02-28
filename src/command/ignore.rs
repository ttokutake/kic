use super::Command;

pub struct Ignore {
    pub command: Option<String>,
    pub path   : Option<String>,
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


fn extract_path(ignore: &Ignore) -> Result<&String, ()> {
    match ignore.path {
        Some(ref p) => Ok(p),
        None        => {
            ignore.help();
            Err(())
        },
    }
}

fn add(ignore: &Ignore) {
    let path = match extract_path(ignore) {
        Ok(p)  => p,
        Err(_) => return,
    };
    println!("do add command with {:?}", path);
}

fn remove(ignore: &Ignore) {
    let path = match extract_path(ignore) {
        Ok(p)  => p,
        Err(_) => return,
    };
    println!("do remove command with {:?}", path);
}

fn ignore_current_files() {
    println!("do current command");
}

fn clear_ignore_file() {
    println!("do clear command");
}
