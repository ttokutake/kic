pub struct Config;

impl Config {
    pub fn default() -> String {
        r#"
[burn]
  after = "2 weeks"

[sweep]
  period = "daily"
  time   = "00:00"
"#
            .to_string()
    }
}
