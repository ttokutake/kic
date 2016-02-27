use super::Command;

pub struct Ignore;

impl Command for Ignore {
    fn validation(&self) -> bool { true }

    fn help_message(&self) -> &'static str {
        return "ignore!";
    }

    fn main(&self) {
        unimplemented!()
    }
}
