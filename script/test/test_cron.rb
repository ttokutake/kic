require_relative 'helper'

class TestCron < TestWithBasicSetup
  @@command_start  = 'start'
  @@command_end    = 'end'
  @@command_patrol = 'patrol'

  @@command_sweep_indeed = 'sweep indeed'
  @@command_burn_indeed  = 'burn indeed'

  def register_with_cron!
    exec(@@command_start)
    raise 'Failed to register with "cron"' if $? != 0
  end

  def unregister_from_cron!
    exec(@@command_end)
    raise 'Failed to unregister from "cron"' if $? != 0
  end

  def get_cron_contents
    `crontab -l`
  end

  def confirm_registered_line(contents, pwd)
    [@@command_sweep_indeed, @@command_burn_indeed].each do |command|
      r = /cd #{pwd} && #{BIN} #{command}\n/
      assert r.match(contents)
    end
  end

  def test_basic_cron_setup
    original = get_cron_contents

    register_with_cron!
    contents = get_cron_contents
    assert_true contents.include?("#{BIN} #{@@command_patrol}")
    confirm_registered_line(contents, PWD)

    unregister_from_cron!
    assert_equal original, get_cron_contents
  end

  def test_patrol_should_delete_invalid_lines
    original = get_cron_contents

    dir_name = 'new_dir'
    Dir.mkdir(dir_name)
    Dir.chdir(dir_name)
    initialize_kic!
    register_with_cron!
    confirm_registered_line(get_cron_contents, Dir.pwd)

    Dir.chdir(PWD)
    FileUtils.remove_entry(dir_name)

    exec(@@command_patrol)
    assert_equal original, get_cron_contents
  end
end
