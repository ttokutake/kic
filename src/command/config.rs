use super::Command;

pub struct Config;

impl Command for Config {
    fn validation(&self) -> bool { true }

    fn help_message(&self) -> &'static str {
        return "config!";
    }

    fn main(&self) {
        unimplemented!()
    }
}
