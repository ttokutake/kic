require 'test/unit'

BASE_DIR = '.kic'

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

def destroy_kic!
  if Dir.exists?('.kic') then
    `
      expect -c '
        set timeout 5
        spawn cargo run destroy
        expect {
          default { exit 1 }
          -regexp "\\\\\\[yes/no\\\\\\]:\\\\s*$"
        }
        send "yes\\n"
        expect eof
      '
    `
    raise 'Failed to destroy.' if $? != 0
  end
end

class TestInitAndDestroy < Test::Unit::TestCase
  class << self
    def startup
      build!
    end
  end

  def teardown
    destroy_kic!
  end

  @@storage  = "#{BASE_DIR}/warehouse"
  @@config   = "#{BASE_DIR}/config.toml"
  @@ignore   = "#{BASE_DIR}/ignore"


  def test_basic_init_and_destroy
    assert !Dir.exists?(BASE_DIR)

    initialize_kic!
    assert Dir .exists?(BASE_DIR)
    assert Dir .exists?(@@storage)
    assert File.exists?(@@config)
    assert File.exists?(@@ignore)

    destroy_kic!
    assert !Dir.exists?(BASE_DIR)
  end

  def test_init_should_not_delete_existing_config_and_ignore
    initialize_kic!

    File.open(@@config, "w").close
    File.open(@@ignore, "w").close
    initialize_kic!
    assert File.zero?(@@config)
    assert File.zero?(@@ignore)
  end
end
