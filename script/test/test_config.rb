require_relative 'helper'
require          'toml'

class TestConfig < TestWithBasicSetup
  @@initial_toml = {
    'burn'  => {'moratorium' => '2 weeks'},
    'sweep' => {'moratorium' => '10 minutes', 'period' => 'daily', 'time' => '00:00'}
  }

  def test_initial_config
    toml = TOML.load_file(CONFIG_FILE)
    assert_equal @@initial_toml, toml
  end

  def test_config_set_should_preserve_new_value
  end

  def test_config_set_should_display_usage
  end

  def test_config_set_should_display_error
  end

  def test_config_init_should_remake_config_file
  end
end
