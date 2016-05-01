BASE_DIR    = '.kic'
STORAGE_DIR = "#{BASE_DIR}/warehouse"
CONFIG_FILE = "#{BASE_DIR}/config.toml"
IGNORE_FILE = "#{BASE_DIR}/ignore"

def build!
  `cargo build`
  raise 'Build failed.' if $? != 0
  `cargo test`
  raise 'Whitebox tests failed.' if $? != 0
end

def initialize_kic!
  `cargo run init`
  raise 'Failed to initialize.' if $? != 0
end

def destroy_kic!(input = 'yes')
  `
    expect -c '
      set timeout 5
      spawn cargo run destroy
      expect {
        default { exit 1 }
        -regexp "\\\\\\[yes/no\\\\\\]:\\\\s*$"
      }
      send "#{input}\\n"
      expect eof
    '
  `
  raise 'Failed to destroy.' if $? != 0
end
