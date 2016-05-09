require_relative 'helper'

class TestIgnore < TestWithBasicSetup
  @@command_add     = 'ignore add'
  @@command_remove  = 'ignore remove'
  @@command_current = 'ignore current'
  @@command_clear   = 'ignore clear'

  def test_config_add_should_display_usage
    exit_status, is_usage = output_usage?(@@command_add)
    assert_not_equal 0, exit_status
    assert_true      is_usage
  end

  def test_config_add_should_append_existing_file_to_ignore_file
    file_names = [
      [[              ], 'file1'],
      [['dir1'        ], 'file2'],
      [['dir1', 'dir2'], 'file3'],
    ]
    file_names.each do |dir_names, file_name|
      file = if dir_names.empty?
        file_name
      else
        path = File.join(dir_names)
        FileUtils.mkdir_p(path)
        File.join(path, file_name)
      end
      FileUtils.touch(file)

      exec("#{@@command_add} #{file}")
      contents = File.open(IGNORE_FILE, &:read)
      assert_true contents.include?(File.join('.', file))

      FileUtils.rm(file)
      FileUtils.remove_entry(dir_names.first) unless dir_names.empty?
    end
  end

  def test_config_add_should_not_append_non_existing_file_to_ignore_file
    file = 'non_existing_file'
    dir  = 'empty_dir'
    FileUtils.mkdir(dir)

    [file, dir].each do |path|
      exec("#{@@command_add} #{path}")
      contents = File.open(IGNORE_FILE, &:read)
      assert_false contents.include?(File.join('.', path))
    end

    FileUtils.rmdir(dir)
  end

  def test_config_remove_should_display_usage
    exit_status, is_usage = output_usage?(@@command_remove)
    assert_not_equal 0, exit_status
    assert_true      is_usage
  end

  def test_config_remove_should_delete_file_from_ignore_file
    files = [
      'non_existing_file',
      'blackbox_test.sh',
    ]

    files.each do |file|
      exec("#{@@command_remove} #{file}")
      contents = File.open(IGNORE_FILE, &:read)
      assert_false contents.include?(File.join('.', file))
    end
  end

  def test_config_current_should_replace_ignore_file_with_new_one_mirroring_current_directory_tree
    initial_ignore = File.open(IGNORE_FILE, &:read)

    File.open(IGNORE_FILE, 'w').close
    exec_with_stdin(@@command_current)
    assert_equal initial_ignore, File.open(IGNORE_FILE, &:read)
  end

  def test_config_clear_should_delete_all_contents_from_ignore_file
    exec_with_stdin(@@command_clear)
    assert_true File.zero?(IGNORE_FILE)
  end
end
