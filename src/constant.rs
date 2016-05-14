pub const ME: &'static str      = "kic";
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub const MAIN_DIR: &'static str = ".";

#[cfg(target_os="linux")]
pub const BANNED_DIRS: [&'static str; 20] = [
    "/",
    "/bin",
    "/boot",
    "/dev",
    "/etc",
    "/home",
    "/lib",
    "/lib64",
    "/lost+found",
    "/media",
    "/mnt",
    "/opt",
    "/proc",
    "/root",
    "/run",
    "/sbin",
    "/srv",
    "/sys",
    "/usr",
    "/var",
];
#[cfg(target_os="macos")]
pub const BANNED_DIRS: [&'static str; 20] = [
    "/",
    "/Applications",
    "/System",
    "/bin",
    "/etc",
    "/net",
    "/sbin",
    "/var",
    "/Library",
    "/Users",
    "/cores",
    "/home",
    "/opt",
    "/tmp",
    "/Network",
    "/Volumes",
    "/dev",
    "/installer.failurerequests",
    "/private",
    "/usr",
];

pub const WORKING_DIR_NAME: &'static str = ".kic";
pub const STORAGE_DIR_NAME: &'static str = "warehouse";
pub const CONFIG_FILE_NAME: &'static str = "config.toml";
pub const IGNORE_FILE_NAME: &'static str = "ignore";
