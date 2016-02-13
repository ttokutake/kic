use std::fmt::Display;
use std::io::{self, Write};

pub fn echo<S: Display>(message: S) {
    print!("{}", message);
    if let Err(why) = io::stdout().flush() {
        panic!("{}", why);
    }
}

pub fn read_line_from_stdin() -> String {
    let mut input = String::new();
    if let Err(why) = io::stdin().read_line(&mut input) {
        panic!("{}", why);
    }
    input.trim().to_string()
}
