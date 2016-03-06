use std::error;
use std::fmt::{self, Display};
use std::io;


#[derive(Debug)]
pub enum CliError {
    Io(io::Error),
    RunningPlace(RunningPlaceError),
}
impl Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CliError::Io(ref e)           => e.fmt(f),
            CliError::RunningPlace(ref e) => e.fmt(f),
        }
    }
}
impl error::Error for CliError {
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            CliError::Io(ref e)           => Some(e),
            CliError::RunningPlace(ref e) => Some(e),
        }
    }

    fn description(&self) -> &str {
        match *self {
            CliError::Io(ref e)           => e.description(),
            CliError::RunningPlace(ref e) => e.description(),
        }
    }
}
impl From<io::Error> for CliError {
    fn from(e: io::Error) -> CliError {
        CliError::Io(e)
    }
}


#[derive(Debug)]
pub struct RunningPlaceError {
    pub dir: String,
}
impl Display for RunningPlaceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cannot run in \"{}\" directory", self.dir)
    }
}
impl error::Error for RunningPlaceError {
    fn cause(&self) -> Option<&error::Error> { None }

    fn description(&self) -> &str { "cannot run in banned directories" }
}
