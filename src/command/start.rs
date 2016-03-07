use error::*;
use super::Command;

pub struct Start;

impl Command for Start {
    fn validation(&self) -> bool { true }

    fn help_message(&self) -> Usage {
        return Usage { kind: UsageKind::Start };
    }

    fn main(&self) {
        unimplemented!()
    }
}
