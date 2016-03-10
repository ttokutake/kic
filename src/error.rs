extern crate regex;
extern crate toml;

use constant::*;
use std::num::ParseIntError;
use std::error;
use std::fmt::{self, Display};
use std::io::Error as IoError;


#[derive(Debug)]
pub enum CliError {
    CannotHappen(CannotHappenError),
    Config(ConfigError),
    Io(IoError),
    ParseInt(ParseIntError),
    ParseToml(toml::ParserError),
    Regex(regex::Error),
    RunningPlace(RunningPlaceError),
}
impl Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CliError::CannotHappen(ref e) => e.fmt(f),
            CliError::Config(ref e)       => e.fmt(f),
            CliError::Io(ref e)           => e.fmt(f),
            CliError::ParseInt(ref e)     => e.fmt(f),
            CliError::ParseToml(ref e)    => e.fmt(f),
            CliError::Regex(ref e)        => e.fmt(f),
            CliError::RunningPlace(ref e) => e.fmt(f),
        }
    }
}
impl error::Error for CliError {
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            CliError::CannotHappen(ref e) => Some(e),
            CliError::Config(ref e)       => Some(e),
            CliError::Io(ref e)           => Some(e),
            CliError::ParseInt(ref e)     => Some(e),
            CliError::ParseToml(ref e)    => Some(e),
            CliError::Regex(ref e)        => Some(e),
            CliError::RunningPlace(ref e) => Some(e),
        }
    }

    fn description(&self) -> &str {
        match *self {
            CliError::CannotHappen(ref e) => e.description(),
            CliError::Config(ref e)       => e.description(),
            CliError::Io(ref e)           => e.description(),
            CliError::ParseInt(ref e)     => e.description(),
            CliError::ParseToml(ref e)    => e.description(),
            CliError::Regex(ref e)        => e.description(),
            CliError::RunningPlace(ref e) => e.description(),
        }
    }
}
impl From<CannotHappenError> for CliError {
    fn from(e: CannotHappenError) -> CliError {
        CliError::CannotHappen(e)
    }
}
impl From<ConfigError> for CliError {
    fn from(e: ConfigError) -> CliError {
        CliError::Config(e)
    }
}
impl From<IoError> for CliError {
    fn from(e: IoError) -> CliError {
        CliError::Io(e)
    }
}
impl From<ParseIntError> for CliError {
    fn from(e: ParseIntError) -> CliError {
        CliError::ParseInt(e)
    }
}
impl From<toml::ParserError> for CliError {
    fn from(e: toml::ParserError) -> CliError {
        CliError::ParseToml(e)
    }
}
impl From<regex::Error> for CliError {
    fn from(e: regex::Error) -> CliError {
        CliError::Regex(e)
    }
}
impl From<RunningPlaceError> for CliError {
    fn from(e: RunningPlaceError) -> CliError {
        CliError::RunningPlace(e)
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


#[derive(Debug)]
pub enum EssentialKind {
    WorkingDir,
    StorageDir,
    ConfigFile,
    IgnoreFile,
}
impl Display for EssentialKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            EssentialKind::WorkingDir => format!("{} directory", WORKING_DIR_NAME),
            EssentialKind::StorageDir => format!("{} directory", STORAGE_DIR_NAME),
            EssentialKind::ConfigFile => format!("{} file"     , CONFIG_FILE_NAME),
            EssentialKind::IgnoreFile => format!("{} file"     , IGNORE_FILE_NAME),
        })
    }
}

#[derive(Debug)]
pub struct EssentialLack {
    pub what: EssentialKind,
}
impl Display for EssentialLack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} does not exist. Please use \"init\" command", self.what)
    }
}
impl error::Error for EssentialLack {
    fn cause(&self) -> Option<&error::Error> { None }

    fn description(&self) -> &str { "essential file does not exist" }
}


#[derive(Debug)]
pub enum UsageKind {
    Nothing,
    Init,
    Config,
    Ignore,
    Sweep,
    Burn,
    Start,
    End,
    Destroy,
}
impl Display for UsageKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            UsageKind::Nothing => format!(
r#"Usage:
    {} <command> [--help|-h]

Command:
    init    # Register current directory.
    config  # Set config's parameters.
    ignore  # Set ignored files.
    sweep   # Sweep files in current directory.
    burn    # Burn sweeped files.
    start   # Start "{}".
    end     # End "{}".
    destroy # Destroy "{}"."#,
                ME,
                ME,
                ME,
                ME,
            ),
            UsageKind::Init    => "init!"   .to_string(),
            UsageKind::Config  => "config!" .to_string(),
            UsageKind::Ignore  => "ignore!" .to_string(),
            UsageKind::Sweep   => "sweep!"  .to_string(),
            UsageKind::Burn    => "burn!"   .to_string(),
            UsageKind::Start   => "start!"  .to_string(),
            UsageKind::End     => "end!"    .to_string(),
            UsageKind::Destroy => "destroy!".to_string(),
        })
    }
}

#[derive(Debug)]
pub struct Usage {
    pub kind: UsageKind,
}
impl Display for Usage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}
impl error::Error for Usage {
    fn cause(&self) -> Option<&error::Error> { None }

    fn description(&self) -> &str { "show usage" }
}


#[derive(Debug)]
pub struct CannotHappenError;
impl Display for CannotHappenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cannot happen")
    }
}
impl error::Error for CannotHappenError {
    fn cause(&self) -> Option<&error::Error> { None }

    fn description(&self) -> &str { "cannot happen" }
}


#[derive(Debug)]
pub enum ConfigErrorKind {
    NotFoundBurnAfter,
    BurnAfter,
    NumOfBurnAfter,
    UnitOfBurnAfter,
}
impl Display for ConfigErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            ConfigErrorKind::NotFoundBurnAfter => r#"Please set [burn]after param"#,
            ConfigErrorKind::BurnAfter         => r#"Invalid "[burn]after" param"#,
            ConfigErrorKind::NumOfBurnAfter    => r#"Please set positive number as "[burn]after""#,
            ConfigErrorKind::UnitOfBurnAfter   => r#"Please set "day" or "week" as "[burn]after""#,
        })
    }
}

#[derive(Debug)]
pub struct ConfigError {
    pub kind: ConfigErrorKind,
}
impl Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}
impl error::Error for ConfigError {
    fn cause(&self) -> Option<&error::Error> { None }

    fn description(&self) -> &str { "invalid params" }
}
