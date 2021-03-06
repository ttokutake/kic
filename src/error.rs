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
    NotFoundBurnMoratorium,
    NotFoundSweepMoratorium,
    NotFoundSweepPeriod,
    NotFoundSweepTime,
    BurnMoratorium,
    SweepMoratorium,
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
            ConfigErrorKind::Something               => r#"Something went to wrong"#,
            ConfigErrorKind::InvalidKey              => r#"Please set key in ["burn.moratorium", "sweep.moratorium", "sweep.period", "sweep.time"]"#,
            ConfigErrorKind::NonStringValue          => r#"Please set values as "String""#,
            ConfigErrorKind::NotFoundBurnMoratorium  => r#"Please set "burn.moratorium""#,
            ConfigErrorKind::NotFoundSweepMoratorium => r#"Please set "sweep.moratorium""#,
            ConfigErrorKind::NotFoundSweepPeriod     => r#"Please set "sweep.period""#,
            ConfigErrorKind::NotFoundSweepTime       => r#"Please set "sweep.time""#,
            ConfigErrorKind::BurnMoratorium          => r#"Please set value like "3days" or "1week" as "burn.moratorium""#,
            ConfigErrorKind::SweepMoratorium         => r#"Please set value like "10hours" or "1week" as "sweep.moratorium""#,
            ConfigErrorKind::SweepPeriod             => r#"Please set "daily" or "weekly" as "sweep.period""#,
            ConfigErrorKind::SweepTime               => r#"Please set value from "00:00" to "23:59" as "sweep.time""#,
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
                format!("{} refresh"          , self.common_usage()),
                format!("{} current"          , self.common_usage()),
                format!("{} clear"            , self.common_usage()),
            ],
            UsageKind::Sweep => vec![format!("{} [all] [indeed]", self.common_usage())],
            UsageKind::Burn  => vec![format!("{} [indeed]", self.common_usage())],
            _ => vec![self.common_usage()],
        }
    }

    fn description(&self) -> &str {
        match *self {
            UsageKind::Nothing => "Keep your directories clean",
            UsageKind::Help    => "Display usage for each command",
            UsageKind::Version => "Display the version of this software",
            UsageKind::Init    => "Register current directory, i.e. create \".kic\" directory",
            UsageKind::Config  => "Change \"config.toml\" file's contents",
            UsageKind::Ignore  => "Change \"ignore\" file's contents",
            UsageKind::Sweep   => "Move dust files and empty directories into \"warehouse\" directory",
            UsageKind::Burn    => "Delete expired directories in \"warehouse\" directory",
            UsageKind::Start   => "Start automatic \"sweep\" and \"burn\" (UNIX-like: cron, Windows: ?)",
            UsageKind::End     => "End automatic \"sweep\" and \"burn\" (UNIX-like: cron, Windows: ?)",
            UsageKind::Destroy => "Unregister current directory, i.e. delete \".kic\" directory",
            UsageKind::Patrol  => "Keep your \"cron\" file clean (UNIX-like only)",
        }
    }

    fn sub_commands(&self) -> Vec<String> {
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
                "set  # Set parameters related to \"sweep\" and \"burn\" commands".to_string(),
                "init # Initialize \"config.toml\" file"                          .to_string(),
            ],
            UsageKind::Ignore => vec![
                "add     # Add directories and files which will be ignored to \"ignore\" file"        .to_string(),
                "remove  # Remove directories and files which have been ignored from \"ignore\" file" .to_string(),
                "refresh # Remove non-existing directories and files from \"ignore\" file"            .to_string(),
                "current # Replace \"ignore\" file with one which register current all files"         .to_string(),
                "clear   # Clear \"ignore\" file, i.e. all files will be aimed from \"sweep\" command".to_string(),
            ],
            UsageKind::Sweep => vec![
                "(none)     # Move fakely dust files into \"warehouse\""                                  .to_string(),
                "indeed     # Move indeed dust files into \"warehouse\""                                  .to_string(),
                "all        # Move fakely dust files into \"warehouse\" including recently accessed files".to_string(),
                "all indeed # Move indeed dust files into \"warehouse\" including recently accessed files".to_string(),
            ],
            UsageKind::Burn => vec![
                "(none) # Delete expired directories in \"warehouse\" fakely".to_string(),
                "indeed # Delete expired directories in \"warehouse\" indeed".to_string(),
            ],
            _ => Vec::new(),
        }
    }

    fn optional_items(&self) -> (&str, Vec<&str>) {
        match *self {
            UsageKind::Config => ("Keys", vec![
                r#"burn.moratorium  # Moratorium to delete directories in "warehouse""#,
                r#"sweep.moratorium # Moratorium to Move "dust"s into "warehouse""#,
                r#"sweep.period     # Period to Move "dust"s by automatic "sweep""#,
                r#"sweep.time       # Time to Move "dust"s by automatic "sweep""#,
            ]),
            _ => ("", Vec::new()),
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

        let sub_commands = Self::create_area("Command", self.kind.sub_commands());

        let (header, items) = self.kind.optional_items();
        let optional_area   = Self::create_area(header, items);

        main + &sub_commands + &optional_area
    }

    fn create_area<S: AsRef<str>>(header: &str, items: Vec<S>) -> String {
        if items.is_empty() {
            "".to_string()
        } else {
            format!("\n{}:\n{}", header, items
                .iter()
                .fold(String::new(), |body, item| body + "    " + item.as_ref() + "\n")
            )
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
