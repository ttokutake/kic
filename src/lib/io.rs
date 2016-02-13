use std::fmt::Display;
use std::io::{self, Write};

pub fn echo<S: Display>(message: S) {
    print!("{}", message);
    if let Err(why) = io::stdout().flush() {
        panic!("{}", why);
    }
}
