require 'date'
require 'test/unit'


def path_to_box(time)
  File.join(STORAGE_DIR, time.strftime('%Y-%m-%d'))
end


PWD = Dir.pwd

BIN_RELATIVE = File.join(__dir__, '..', '..', 'target', 'debug', 'kic')
BIN          = File.expand_path(BIN_RELATIVE)

BASE_DIR    = '.kic'
STORAGE_DIR = File.join(BASE_DIR, 'warehouse')
CONFIG_FILE = File.join(BASE_DIR, 'config.toml')
IGNORE_FILE = File.join(BASE_DIR, 'ignore')

TODAY = Date.today

BOX      = path_to_box(TODAY)
DUST_BOX = File.join(BOX, 'dusts')


def build!
  `cargo build`
  raise 'Build failed.' if $? != 0
end

def initialize_kic!
  exec('init')
  raise 'Failed to initialize.' if $? != 0
end

def destroy_kic!(input = 'yes')
  exec_with_stdin('destroy', input)
  raise 'Failed to destroy.' if $? != 0
end

def exec(command)
  `#{BIN} #{command}`
end

def exec_with_stdin(command, input = 'yes')
  `echo '#{input}' | #{BIN} #{command}`
end

def output_usage?(command)
  output = exec(command)
  [$?, output.start_with?('Usage:')]
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
