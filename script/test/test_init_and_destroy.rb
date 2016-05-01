require 'test/unit'

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

class TestInitAndDestroy < Test::Unit::TestCase
  class << self
    def startup
      build!
    end
  end

  def setup
    @base_dir = '.kic'
  end

  def test_basic_init_and_destroy
    assert !Dir.exists?(@base_dir)

    initialize_kic!
    assert Dir .exists?(@base_dir)
    assert Dir .exists?("#{@base_dir}/warehouse")
    assert File.exists?("#{@base_dir}/config.toml")
    assert File.exists?("#{@base_dir}/ignore")

    destroy_kic!
    assert !Dir.exists?(@base_dir)
  end
end
