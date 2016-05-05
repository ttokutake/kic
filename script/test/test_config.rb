require_relative 'helper'
require          'toml'

class TestConfig < TestWithBasicSetup
  @@set_command = "#{BIN} config set"

  @@initial_toml = {
    'burn'  => {'moratorium' => '2 weeks'},
    'sweep' => {'moratorium' => '10 minutes', 'period' => 'daily', 'time' => '00:00'}
  }

  def test_initial_config
    toml = TOML.load_file(CONFIG_FILE)
    assert_equal @@initial_toml, toml
  end

  def test_config_set_should_display_usage
    args = [
      '',
      'key_only',
    ]
    args.each do |arg|
      output = `#{@@set_command} #{arg}`
      assert_not_equal $?, 0
      assert_true      output.include?('Usage:')
    end
  end

  def test_config_set_should_preserve_new_value
    kvs = {
      'burn.moratorium'  => ['1day', '7days', '1week', '4weeks'],
      'sweep.moratorium' => ['0minute', '60minutes', '0hour', '24hours', '0day', '7days', '0week', '4weeks'],
      "sweep.period"     => ['daily', 'weekly'],
      "sweep.time"       => ['00:00', '23:59'],
    }
    kvs.each do |key, values|
      values.each do |value|
        `#{@@set_command} #{key} #{value}`
        assert_equal $?, 0
      end
    end
  end

  def test_config_set_should_display_error
    kvs = {
      'burn.moratorium'  => ['day', 'week'],
      'sweep.moratorium' => ['minute', 'hour', 'day', 'week'],
      'sweep.period'     => ['hourly', 'monthly'],
      'sweep.time'       => ['24:00', '00:00:00'],
    }
    kvs.each do |key, values|
      values.each do |value|
        output = `#{@@set_command} #{key} #{value}`
        assert_not_equal $?, 0
        assert_true      output.include?('ERROR:')
      end
    end
  end

  def test_config_init_should_remake_config_file
  end
end
