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

pub const DEFAULT_CONFIG: &'static str =
"[sweep]
  period = daily
  time   = 00:00

[burn]
  after = 2 weeks

[hidden-file]
  delete = no
";
