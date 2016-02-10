use super::Command;

pub struct End;

impl Command for End {
    fn validation(&self) -> bool { true }

    fn help_message(&self) -> &'static str {
        return "end!";
    }

    fn main(&self) {
        unimplemented!()
    }
}
