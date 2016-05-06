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
  end

  def test_config_add_should_not_append_non_existing_file_to_ignore_file
  end

  def test_config_remove_should_display_usage
    exit_status, is_usage = output_usage?(@@command_remove)
    assert_not_equal 0, exit_status
    assert_true      is_usage
  end

  def test_config_remove_should_delete_existing_file_from_ignore_file
  end

  def test_config_remove_should_delete_non_existing_file_from_ignore_file
  end

  def test_config_current_should_replace_ignore_file_with_new_one_mirroring_current_directory_tree
  end

  def test_config_clear_should_delete_all_contents_from_ignore_file
  end
end
