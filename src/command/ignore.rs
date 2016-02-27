use super::Command;

pub struct Ignore {
    pub command: Option<String>,
    pub value  : Option<String>,
}

impl Command for Ignore {
    fn validation(&self) -> bool { true }

    fn help_message(&self) -> &'static str {
        return "ignore!";
    }

    fn main(&self) {
        println!("{:?}", self.command);
        println!("{:?}", self.value);
    }
}
