require 'test/unit'
require_relative 'test_helper'

class TestConfig < Test::Unit::TestCase
  class << self
    def startup
      build!
    end
  end

  def setup
    initialize_kic!
  end

  def teardown
    destroy_kic!
  end

  def test_config_set_should_preserve_new_value
  end
end
