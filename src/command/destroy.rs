use super::Command;

pub struct Destroy;

impl Command for Destroy {
    fn validation(&self) -> bool { true }

    fn help_message(&self) -> &'static str {
        return "destroy!";
    }

    fn main(&self) {
        unimplemented!()
    }
}
