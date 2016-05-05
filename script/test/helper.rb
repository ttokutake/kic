require 'test/unit'


BIN = File.join(__dir__, '..', '..', 'target', 'debug', 'kic')

BASE_DIR    = '.kic'
STORAGE_DIR = File.join(BASE_DIR, 'warehouse')
CONFIG_FILE = File.join(BASE_DIR, 'config.toml')
IGNORE_FILE = File.join(BASE_DIR, 'ignore')


def build!
  `cargo build`
  raise 'Build failed.' if $? != 0
  #`cargo test`
  #raise 'Whitebox tests failed.' if $? != 0
end

def initialize_kic!
  `#{BIN} init`
  raise 'Failed to initialize.' if $? != 0
end

def destroy_kic!(input = 'yes')
  `
    expect -c '
      set timeout 5
      spawn #{BIN} destroy
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


class TestWithBuild < Test::Unit::TestCase
  class << self
    def startup
      build!
    end
  end
end

class TestWithBasicSetup < TestWithBuild
  def setup
    initialize_kic!
  end

  def teardown
    destroy_kic!
  end
end
