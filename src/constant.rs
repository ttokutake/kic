pub const ME: &'static str = "kic";

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

pub const WORKING_DIR_NAME: &'static str = ".kic";
pub const STORAGE_DIR_NAME: &'static str = "warehouse";
pub const CONFIG_FILE_NAME: &'static str = "config.toml";
pub const IGNORE_FILE_NAME: &'static str = "ignore";

pub const KEEPED_FILE_NAME: &'static str = ".kickeep";
