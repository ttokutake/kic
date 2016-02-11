use super::Command;

pub struct Burn;

impl Command for Burn {
    fn validation(&self) -> bool { true }

    fn help_message(&self) -> &'static str {
        return "burn!";
    }

    fn main(&self) {
        unimplemented!()
    }
}
