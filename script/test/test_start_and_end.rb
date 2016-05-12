require_relative 'helper'

class TestStartAndEnd < TestWithBasicSetup
  def test_basic_cron_setup
    original = get_cron_contents

    register_with_cron!
    after = get_cron_contents
    assert_true after.include?("#{BIN} patrol")
    assert_true after.include?("#{BIN} sweep")
    assert_true after.include?("#{BIN} burn")

    unregister_from_cron!
    final = get_cron_contents
    assert_equal original, final
  end
end
