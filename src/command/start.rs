use super::Command;

pub struct Start;

impl Command for Start {
    fn validation(&self) -> bool { true }

    fn help_message(&self) -> &'static str {
        return "start!";
    }

    fn main(&self) {
        unimplemented!()
    }
}
