require_relative 'helper'

class TestBurn < TestWithBasicSetup
  @@command_burn        = 'burn'
  @@command_burn_indeed = 'burn indeed'

  def setup
    super

    FileUtils.touch('file1')
    exec('sweep all indeed')
    assert_true File.exist?(BOX)
  end

  def test_burn_should_not_delete_non_expired_box
    exec(@@command_burn)
    assert_true File.exist?(BOX)

    exec(@@command_burn)
    assert_true File.exist?(BOX)

    non_expired_time = TODAY - 13
    non_expired_box = path_to_box(non_expired_time)
    FileUtils.mv(BOX, non_expired_box)

    result = exec(@@command_burn)
    assert_false result.include?(non_expired_box)
    assert_true  File.exists?(non_expired_box)

    result = exec(@@command_burn_indeed)
    assert_false result.include?(non_expired_box)
    assert_true  File.exists?(non_expired_box)
  end

  def test_burn_should_delete_expired_box
    expired_time = TODAY - 14
    expired_box  = path_to_box(expired_time)
    FileUtils.mv(BOX, expired_box)

    result = exec(@@command_burn)
    assert_true result.include?(expired_box)
    assert_true File.exists?(expired_box)

    exec(@@command_burn_indeed)
    assert_true  result.include?(expired_box)
    assert_false File.exists?(expired_box)
  end
end
