use super::Command;

pub struct Init;

impl Command for Init {
    fn validation(&self) -> bool { false }

    fn help_message(&self) -> &'static str {
        return "init!";
    }

    fn main(&self) {
        unimplemented!()
    }
}
