require_relative 'helper'

class TestSweep < TestWithBasicSetup
  @@command_sweep            = "sweep"
  @@command_sweep_indeed     = "sweep indeed"
  @@command_sweep_all        = "sweep all"
  @@command_sweep_all_indeed = "sweep all indeed"

  def setup
    super

    @dir1, @dir2, @dir3 = ['dir1', 'dir2', 'dir3']
    @file1, @file2, @file3 = ['file1', 'file2', 'file3']
    @d1 = File.join('.', @dir1)
    @d2 = File.join(@d1, @dir2)
    @d3 = File.join(@d2, @dir3)
    @f1 = File.join('.', @file1)
    @f2 = File.join(@d1, @file2)
    @f3 = File.join(@d2, @file3)
    FileUtils.mkdir_p(@d3)
    FileUtils.touch(@f1)
    FileUtils.touch(@f2)
    FileUtils.touch(@f3)
  end

  def teardown
    super

    FileUtils.rm(@f1)           if File.exist?(@f1)
    FileUtils.remove_entry(@d1) if File.exist?(@d1)
  end

  def enclose(str)
    '"' + str + '"'
  end

  def test_sweep_should_delete_dust_files
    result = exec(@@command_sweep)
    assert_false result.include?(enclose(@f1))
    assert_false result.include?(enclose(@f2))
    assert_false result.include?(enclose(@f3))
    assert_false result.include?(enclose(@d1))
    assert_false result.include?(enclose(@d2))
    assert_true  result.include?(enclose(@d3))

    exec(@@command_sweep_indeed)
    assert_true File.exist?(DUST_BOX)
    assert_true File.exist?(File.join(DUST_BOX, @d3))

    exec('config set sweep.moratorium 0minute')
    result = exec(@@command_sweep)
    assert_true result.include?(enclose(@f1))
    assert_true result.include?(enclose(@f2))
    assert_true result.include?(enclose(@f3))
    assert_true result.include?(enclose(@d1))
    assert_true result.include?(enclose(@d2))

    exec(@@command_sweep_indeed)
    assert_true File.exist?(DUST_BOX)
    assert_true File.exist?(File.join(DUST_BOX, @f1))
    assert_true File.exist?(File.join(DUST_BOX, @f2))
    assert_true File.exist?(File.join(DUST_BOX, @f3))
    assert_true File.exist?(File.join(DUST_BOX, @d1))
    assert_true File.exist?(File.join(DUST_BOX, @d2))
  end

  def test_sweep_all_should_delete_dust_files_with_recently_accessed_files
    result = exec(@@command_sweep_all)
    assert_true result.include?(enclose(@f1))
    assert_true result.include?(enclose(@f2))
    assert_true result.include?(enclose(@f3))
    assert_true result.include?(enclose(@d1))
    assert_true result.include?(enclose(@d2))
    assert_true result.include?(enclose(@d3))

    result = exec(@@command_sweep_all_indeed)
    assert_true File.exist?(DUST_BOX)
    assert_true File.exist?(File.join(DUST_BOX, @f1))
    assert_true File.exist?(File.join(DUST_BOX, @f2))
    assert_true File.exist?(File.join(DUST_BOX, @f3))
    assert_true File.exist?(File.join(DUST_BOX, @d1))
    assert_true File.exist?(File.join(DUST_BOX, @d2))
    assert_true File.exist?(File.join(DUST_BOX, @d3))
  end
end
