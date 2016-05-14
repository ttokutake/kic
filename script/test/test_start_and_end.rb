require_relative 'helper'

class TestStartAndEnd < TestWithBasicSetup
  def test_basic_cron_setup
    original = get_cron_contents

    register_with_cron!
    contents = get_cron_contents
    assert_true contents.include?("#{BIN} patrol")
    ['sweep indeed', 'burn indeed'].each do |command|
      r = /cd #{PWD}.*#{BIN} #{command}\s*\n/
      assert r.match(contents)
    end

    unregister_from_cron!
    final = get_cron_contents
    assert_equal original, final
  end
end
