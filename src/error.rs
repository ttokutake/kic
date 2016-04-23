extern crate regex;
extern crate toml;

use self::regex::Error as RegexError;
use self::toml::ParserError as ParseTomlError;

use constant::{self, ME};
use std::num::ParseIntError;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::Error as IoError;


#[derive(Debug)]
pub enum CliError {
    Config(ConfigError),
    Cron(CronError),
    Essential(EssentialLack),
    Io(IoError),
    ParseInt(ParseIntError),
    ParseToml(ParseTomlError),
    Regex(RegexError),
    RunningPlace(RunningPlaceError),
    Usage(Usage),
}
impl Display for CliError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            CliError::Config(ref e)       => e.fmt(f),
            CliError::Cron(ref e)         => e.fmt(f),
            CliError::Essential(ref e)    => e.fmt(f),
            CliError::Io(ref e)           => e.fmt(f),
            CliError::ParseInt(ref e)     => e.fmt(f),
            CliError::ParseToml(ref e)    => e.fmt(f),
            CliError::Regex(ref e)        => e.fmt(f),
            CliError::RunningPlace(ref e) => e.fmt(f),
            CliError::Usage(ref u)        => u.fmt(f),
        }
    }
}
impl Error for CliError {
    fn cause(&self) -> Option<&Error> {
        match *self {
            CliError::Config(ref e)       => Some(e),
            CliError::Cron(ref e)         => Some(e),
            CliError::Essential(ref e)    => Some(e),
            CliError::Io(ref e)           => Some(e),
            CliError::ParseInt(ref e)     => Some(e),
            CliError::ParseToml(ref e)    => Some(e),
            CliError::Regex(ref e)        => Some(e),
            CliError::RunningPlace(ref e) => Some(e),
            CliError::Usage(ref u)        => Some(u),
        }
    }

    fn description(&self) -> &str {
        match *self {
            CliError::Config(ref e)       => e.description(),
            CliError::Cron(ref e)         => e.description(),
            CliError::Essential(ref e)    => e.description(),
            CliError::Io(ref e)           => e.description(),
            CliError::ParseInt(ref e)     => e.description(),
            CliError::ParseToml(ref e)    => e.description(),
            CliError::Regex(ref e)        => e.description(),
            CliError::RunningPlace(ref e) => e.description(),
            CliError::Usage(ref u)        => u.description(),
        }
    }
}
impl From<ConfigError> for CliError {
    fn from(e: ConfigError) -> CliError {
        CliError::Config(e)
    }
}
impl From<CronError> for CliError {
    fn from(e: CronError) -> CliError {
        CliError::Cron(e)
    }
}
impl From<EssentialLack> for CliError {
    fn from(e: EssentialLack) -> CliError {
        CliError::Essential(e)
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
impl From<Usage> for CliError {
    fn from(u: Usage) -> CliError {
        CliError::Usage(u)
    }
}


#[derive(Debug, PartialEq)]
pub enum ConfigErrorKind {
    Something,
    InvalidKey,
    NonStringValue,
    NotFoundBurnAfter,
    NotFoundSweepPeriod,
    NotFoundSweepTime,
    BurnAfter,
    SweepPeriod,
    SweepTime,
}
#[derive(Debug, PartialEq)]
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
            ConfigErrorKind::Something           => r#"Something went to wrong"#,
            ConfigErrorKind::InvalidKey          => r#"Cannot set this key"#,
            ConfigErrorKind::NonStringValue      => r#"Please set values as "String""#,
            ConfigErrorKind::NotFoundBurnAfter   => r#"Please set [burn]after param"#,
            ConfigErrorKind::NotFoundSweepPeriod => r#"Please set [sweep]period param"#,
            ConfigErrorKind::NotFoundSweepTime   => r#"Please set [sweep]time param"#,
            ConfigErrorKind::BurnAfter           => r#"Please set value like "3days" or "1week" as "[burn]after""#,
            ConfigErrorKind::SweepPeriod         => r#"Please set "daily" or "weekly" as "[sweep]period""#,
            ConfigErrorKind::SweepTime           => r#"Please set value from "00:00" to "23:59" as "[sweep]time""#,
        })
    }
}
impl Error for ConfigError {
    fn description(&self) -> &str { "invalid parameters" }
}


#[derive(Debug, PartialEq)]
pub enum CronErrorKind {
    BinExistsInInvalidDir,
    InvalidCharacterCode,
    InvalidPath,
    FailedToWrite,
}
#[derive(Debug, PartialEq)]
pub struct CronError {
    kind: CronErrorKind,
}
impl CronError {
    pub fn new(kind: CronErrorKind) -> Self {
        CronError { kind: kind }
    }
}
impl Display for CronError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", match self.kind {
            CronErrorKind::BinExistsInInvalidDir => "The \"bin\" exists in directory whose path include non-UTF-8 charecter code",
            CronErrorKind::InvalidCharacterCode  => "The \"cron\" file consists of non-UTF-8 character code",
            CronErrorKind::InvalidPath           => "The path to current directory include non-UTF-8 character code",
            CronErrorKind::FailedToWrite         => "Failed to write new \"cron\" file"
        })
    }
}
impl Error for CronError {
    fn description(&self) -> &str { "problems around cron" }
}


#[derive(Debug, PartialEq)]
pub enum EssentialKind {
    WorkingDir,
    StorageDir,
    ConfigFile,
    IgnoreFile,
}
#[derive(Debug, PartialEq)]
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
        write!(f, "{} does not exist", match self.what {
            EssentialKind::WorkingDir => format!("\"{}\" directory", constant::WORKING_DIR_NAME),
            EssentialKind::StorageDir => format!("\"{}\" directory", constant::STORAGE_DIR_NAME),
            EssentialKind::ConfigFile => format!("\"{}\" file"     , constant::CONFIG_FILE_NAME),
            EssentialKind::IgnoreFile => format!("\"{}\" file"     , constant::IGNORE_FILE_NAME),
        })
    }
}
impl Error for EssentialLack {
    fn description(&self) -> &str { "essential file does not exist" }
}


#[derive(Debug, PartialEq)]
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


#[derive(Debug, PartialEq)]
pub enum UsageKind {
    Nothing,
    Help,
    Version,
    Init,
    Config,
    Ignore,
    Sweep,
    Burn,
    Start,
    End,
    Destroy,
    Patrol,
}
impl UsageKind {
    fn command(&self) -> &str {
        match *self {
            UsageKind::Nothing => "<Command>",
            UsageKind::Help    => "help",
            UsageKind::Version => "version",
            UsageKind::Init    => "init",
            UsageKind::Config  => "config",
            UsageKind::Ignore  => "ignore",
            UsageKind::Sweep   => "sweep",
            UsageKind::Burn    => "burn",
            UsageKind::Start   => "start",
            UsageKind::End     => "end",
            UsageKind::Destroy => "destroy",
            UsageKind::Patrol  => "patrol",
        }
    }

    fn common_usage(&self) -> String {
        format!("{} {}", ME, self.command())
    }

    fn usages(&self) -> Vec<String> {
        match *self {
            UsageKind::Help   => vec![format!("{} <SubCommand>", self.common_usage())],
            UsageKind::Config => vec![
                format!("{} set <Key> <Value>", self.common_usage()),
                format!("{} init"             , self.common_usage()),
            ],
            UsageKind::Ignore => vec![
                format!("{} add <File> ..."   , self.common_usage()),
                format!("{} remove <File> ...", self.common_usage()),
                format!("{} current"          , self.common_usage()),
                format!("{} clear"            , self.common_usage()),
            ],
            _ => vec![self.common_usage()],
        }
    }

    fn description(&self) -> &str {
        match *self {
            UsageKind::Nothing => "Keep your directories clean!",
            UsageKind::Help    => "Display usage for each command.",
            UsageKind::Version => "Display the version of this software.",
            UsageKind::Init    => "Register current directory.",
            UsageKind::Config  => "Change parameter.",
            UsageKind::Ignore  => "Change \"non-dust\" file's list.",
            UsageKind::Sweep   => "Sweep \"dust\" files and empty directories in current directory.",
            UsageKind::Burn    => "Burn \"sweeped\" files.",
            UsageKind::Start   => "Register with \"cron\" for autonomous operation.",
            UsageKind::End     => "Unregister from \"cron\".",
            UsageKind::Destroy => "Unregister current directory.",
            UsageKind::Patrol  => "Keep \"cron\" file clean.",
        }
    }

    fn sub_descriptions(&self) -> Vec<String> {
        match *self {
            UsageKind::Nothing => vec![
                format!("{}{}", "help    # ", UsageKind::Help   .description()),
                format!("{}{}", "version # ", UsageKind::Version.description()),
                format!("{}{}", "init    # ", UsageKind::Init   .description()),
                format!("{}{}", "config  # ", UsageKind::Config .description()),
                format!("{}{}", "ignore  # ", UsageKind::Ignore .description()),
                format!("{}{}", "sweep   # ", UsageKind::Sweep  .description()),
                format!("{}{}", "burn    # ", UsageKind::Burn   .description()),
                format!("{}{}", "start   # ", UsageKind::Start  .description()),
                format!("{}{}", "end     # ", UsageKind::End    .description()),
                format!("{}{}", "destroy # ", UsageKind::Destroy.description()),
                format!("{}{}", "patrol  # ", UsageKind::Patrol .description()),
            ],
            UsageKind::Config => vec![
                "set  # Set parameter"       .to_string(),
                "init # Initialize configure".to_string(),
            ],
            UsageKind::Ignore => {
                let file_expression = "\"non-dust\" file's list";
                vec![
                    format!("add     # Add specified files to {}"        , file_expression),
                    format!("remove  # Remove specified files from {}"   , file_expression),
                    format!("current # Replace {} with current all files", file_expression),
                    format!("clear   # Clear {}"                         , file_expression),
                ]
            },
            _ => Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Usage {
    kind: UsageKind,
}
impl Usage {
    pub fn new(kind: UsageKind) -> Usage {
        Usage { kind: kind }
    }

    fn message(&self) -> String {
        let usage = self.kind
            .usages()
            .iter()
            .fold(String::new(), |message, line| {
                message + "    " + line + "\n"
            });

        let main = format!("Usage:\n{}\nDescription:\n    {}\n", usage, self.kind.description());

        let sub_description = self.kind
            .sub_descriptions()
            .iter()
            .fold(String::new(), |message, line| {
                message + "    " + line + "\n"
            });

        if sub_description.is_empty() {
            main
        } else {
            format!("{}{}{}", main, "\nCommand:\n", sub_description)
        }
    }
}
impl Display for Usage {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}
impl Error for Usage {
    fn description(&self) -> &str { "show usage" }
}
