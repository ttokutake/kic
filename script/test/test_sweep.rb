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
    not_dusts      = [@f1, @f2, @f3]
    non_empty_dirs = [@d1, @d2]

    result = exec(@@command_sweep)
    (not_dusts + non_empty_dirs).each do |not_dust|
      assert_false result.include?(enclose(not_dust))
    end
    assert_true result.include?(enclose(@d3))

    exec(@@command_sweep_indeed)
    assert_true File.exist?(DUST_BOX)
    assert_true  File.exist?(File.join(DUST_BOX, @d3))
    assert_false File.exist?(@d3)
    non_empty_dirs.each do |non_empty_dir|
      assert_true File.exist?(File.join(DUST_BOX, non_empty_dir))
      assert_true File.exist?(non_empty_dir)
    end
    not_dusts.each do |not_dust|
      assert_false File.exist?(File.join(DUST_BOX, not_dust))
      assert_true  File.exist?(not_dust)
    end
  end

  def test_sweep_should_delete_dust_files_without_moratorium
    exec('config set sweep.moratorium 0minute')

    dusts = [@f1, @f2, @f3, @d1, @d2, @d3]

    result = exec(@@command_sweep)
    dusts.each do |dust|
      assert_true result.include?(enclose(dust))
    end

    exec(@@command_sweep_indeed)
    assert_true File.exist?(DUST_BOX)
    dusts.each do |dust|
      assert_true  File.exist?(File.join(DUST_BOX, dust))
      assert_false File.exist?(dust)
    end
  end

  def test_sweep_all_should_delete_dust_files_with_recently_accessed_files
    dusts = [@f1, @f2, @f3, @d1, @d2, @d3]

    result = exec(@@command_sweep_all)
    dusts.each do |dust|
      assert_true result.include?(enclose(dust))
    end

    result = exec(@@command_sweep_all_indeed)
    assert_true File.exist?(DUST_BOX)
    dusts.each do |dust|
      assert_true  File.exist?(File.join(DUST_BOX, dust))
      assert_false File.exist?(dust)
    end
  end
end
