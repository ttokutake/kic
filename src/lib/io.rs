use std::fmt::Display;
use std::io::{self, Write};

pub fn echo<S: Display>(message: S) {
    print!("{}", message);
    if let Err(why) = io::stdout().flush() {
        panic!("{}", why);
    }
}

pub fn read_line_from_stdin() -> Result<String, io::Error>  {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .map(|_| input.trim().to_string())
}


pub enum Tag {
    Execution,
    Okay,
    Notice,
    Warning,
    Error,
    Caution,
}

pub fn format_with_tag<S: Display>(indent_level: usize, tag: Tag, message: S) -> String {
    let indent = (0 .. indent_level).map(|_| "  ").collect::<String>();

    let extra = match tag {
        Tag::Okay | Tag::Notice => "\n",
        _                       => "",
    };

    let tag = match tag {
        Tag::Execution => "EXECUTION",
        Tag::Okay      => "OK",
        Tag::Notice    => "NOTICE",
        Tag::Warning   => "WARNING",
        Tag::Error     => "ERROR",
        Tag::Caution   => "CAUTION",
    };

    format!("{}{}: {}{}", indent, tag, message, extra)
}

pub fn print_with_tag<S: Display>(indent_level: usize, tag: Tag, message: S) {
    println!("{}", format_with_tag(indent_level, tag, message));
}
