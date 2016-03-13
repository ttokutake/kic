extern crate regex;
extern crate toml;

use constant::*;
use self::regex::Error as RegexError;
use self::toml::ParserError as ParseTomlError;
use std::num::ParseIntError;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::Error as IoError;


#[derive(Debug)]
pub enum CliError {
    CannotHappen(CannotHappenError),
    Config(ConfigError),
    Io(IoError),
    ParseInt(ParseIntError),
    ParseToml(ParseTomlError),
    Regex(RegexError),
    RunningPlace(RunningPlaceError),
}
impl Display for CliError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
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
impl Error for CliError {
    fn cause(&self) -> Option<&Error> {
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
impl From<ParseTomlError> for CliError {
    fn from(e: ParseTomlError) -> CliError {
        CliError::ParseToml(e)
    }
}
impl From<RegexError> for CliError {
    fn from(e: RegexError) -> CliError {
        CliError::Regex(e)
    }
}
impl From<RunningPlaceError> for CliError {
    fn from(e: RunningPlaceError) -> CliError {
        CliError::RunningPlace(e)
    }
}


#[derive(Debug)]
pub struct CannotHappenError;
impl Display for CannotHappenError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "Cannot happen")
    }
}
impl Error for CannotHappenError {
    fn description(&self) -> &str { "cannot happen" }
}


#[derive(Debug)]
pub enum ConfigErrorKind {
    NotFoundBurnAfter,
    BurnAfter,
    NumOfBurnAfter,
    UnitOfBurnAfter,
}
#[derive(Debug)]
pub struct ConfigError {
    kind: ConfigErrorKind,
}
impl ConfigError {
    pub fn new(kind: ConfigErrorKind) -> ConfigError {
        ConfigError { kind: kind }
    }
}
impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", match self.kind {
            ConfigErrorKind::NotFoundBurnAfter => r#"Please set [burn]after param"#,
            ConfigErrorKind::BurnAfter         => r#"Invalid "[burn]after" param"#,
            ConfigErrorKind::NumOfBurnAfter    => r#"Please set positive number as "[burn]after""#,
            ConfigErrorKind::UnitOfBurnAfter   => r#"Please set "day" or "week" as "[burn]after""#,
        })
    }
}
impl Error for ConfigError {
    fn description(&self) -> &str { "invalid params" }
}


#[derive(Debug)]
pub enum EssentialKind {
    WorkingDir,
    StorageDir,
    ConfigFile,
    IgnoreFile,
}
#[derive(Debug)]
pub struct EssentialLack {
    what: EssentialKind,
}
impl EssentialLack {
    pub fn new(what: EssentialKind) -> EssentialLack {
        EssentialLack { what: what }
    }
}
impl Display for EssentialLack {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{} does not exist. Please use \"init\" command", match self.what {
            EssentialKind::WorkingDir => format!("{} directory", WORKING_DIR_NAME),
            EssentialKind::StorageDir => format!("{} directory", STORAGE_DIR_NAME),
            EssentialKind::ConfigFile => format!("{} file"     , CONFIG_FILE_NAME),
            EssentialKind::IgnoreFile => format!("{} file"     , IGNORE_FILE_NAME),
        })
    }
}
impl Error for EssentialLack {
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
#[derive(Debug)]
pub struct Usage {
    kind: UsageKind,
}
impl Usage {
    pub fn new(kind: UsageKind) -> Usage {
        Usage { kind: kind }
    }
}
impl Display for Usage {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", match self.kind {
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
impl Error for Usage {
    fn description(&self) -> &str { "show usage" }
}


#[derive(Debug)]
pub struct RunningPlaceError {
    dir: String,
}
impl RunningPlaceError {
    pub fn new(dir: String) -> RunningPlaceError {
        RunningPlaceError { dir: dir }
    }
}
impl Display for RunningPlaceError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "Cannot run in \"{}\" directory", self.dir)
    }
}
impl Error for RunningPlaceError {
    fn description(&self) -> &str { "cannot run in banned directories" }
}
