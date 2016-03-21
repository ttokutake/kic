use std::fmt::Display;
use std::io::{self, Error as IoError, Write};


pub fn echo<S: Display>(message: S) {
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

pub fn format_with_tag<S: Display>(tag: Tag, message: S) -> String {
    let tag = match tag {
        Tag::Info      => "INFO",
        Tag::Notice    => "NOTICE",
        Tag::Warning   => "WARNING",
        Tag::Error     => "ERROR",
        Tag::Caution   => "CAUTION",
    };

    format!("{}: {}", tag, message)
}

pub fn print_with_tag<S: Display>(tag: Tag, message: S) {
    println!("{}", format_with_tag(tag, message));
}
