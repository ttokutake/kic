use super::Command;

pub struct Set;

impl Command for Set {
    fn validation(&self) -> bool { true }

    fn help_message(&self) -> &'static str {
        return "set!";
    }

    fn main(&self) {
        unimplemented!()
    }
}
