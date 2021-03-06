use std::fmt::Display;
use std::io::{self, Error as IoError, Write};


pub fn echo<D: Display>(message: D) {
    print!("{}", message);
    if let Err(why) = io::stdout().flush() {
        panic!("{}", why);
    }
}

pub fn read_line_from_stdin() -> Result<String, IoError>  {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .map(|_| input.trim().to_string())
}


pub enum Tag {
    Info,
    Notice,
    Warning,
    Error,
    Caution,
}
impl Tag {
    fn to_str(&self) -> &str {
        match *self {
            Tag::Info    => "INFO",
            Tag::Notice  => "NOTICE",
            Tag::Warning => "WARNING",
            Tag::Error   => "ERROR",
            Tag::Caution => "CAUTION",
        }
    }
}

pub fn format_with_tag<D: Display>(tag: Tag, message: D) -> String {
    format!("{}: {}", tag.to_str(), message)
}

pub fn print_with_tag<D: Display>(tag: Tag, message: D) {
    println!("{}", format_with_tag(tag, message));
}
