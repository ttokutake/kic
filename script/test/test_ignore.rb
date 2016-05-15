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

  def test_config_add_should_append_existing_dir_to_ignore_file
    dir_name = 'test'
    exec("#{@@command_add} #{dir_name}/")
    contents = File.open(IGNORE_FILE, &:read)
    expected_line = File.join('.', dir_name)
    assert_true  contents.include?(expected_line)
    assert_false contents.include?(expected_line + '/')
  end

  def test_config_add_should_not_append_non_existing_file_to_ignore_file
    file = 'non_existing_file'

    path_with_current_dir = File.join('.', file)
    contents = File.open(IGNORE_FILE, &:read)
    assert_false contents.include?(path_with_current_dir)

    exec("#{@@command_add} #{file}")
    contents = File.open(IGNORE_FILE, &:read)
    assert_false contents.include?(path_with_current_dir)
  end

  def test_config_remove_should_display_usage
    exit_status, is_usage = output_usage?(@@command_remove)
    assert_not_equal 0, exit_status
    assert_true      is_usage
  end

  def test_config_remove_should_delete_file_from_ignore_file
    files = [
      ['non_existing_file', false],
      ['blackbox_test.sh' , true ],
    ]

    files.each do |file, exists_in|
      path = File.join('.', file)
      contents = File.open(IGNORE_FILE, &:read)
      assert_equal exists_in, contents.include?(path)

      exec("#{@@command_remove} #{file}")
      contents = File.open(IGNORE_FILE, &:read)
      assert_false contents.include?(path)
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
