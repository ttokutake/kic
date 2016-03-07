use error::*;
use super::Command;

pub struct End;

impl Command for End {
    fn validation(&self) -> bool { true }

    fn usage(&self) -> Usage {
        return Usage { kind: UsageKind::End };
    }

    fn main(&self) {
        unimplemented!()
    }
}
