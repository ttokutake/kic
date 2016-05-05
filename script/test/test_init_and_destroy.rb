require_relative 'helper'

class TestInitAndDestroy < TestWithBuild
  def teardown
    if base_dir_exists?
      destroy_kic!
    end
  end

  def base_dir_exists?
    Dir.exists?(BASE_DIR)
  end

  def test_basic_setup
    assert_false base_dir_exists?

    initialize_kic!
    assert_true base_dir_exists?
    assert_true Dir .exists?(STORAGE_DIR)
    assert_true File.exists?(CONFIG_FILE)
    assert_true File.exists?(IGNORE_FILE)

    destroy_kic!
    assert_true !Dir.exists?(BASE_DIR)
  end

  def test_init_should_not_delete_existing_config_and_ignore_file
    initialize_kic!

    File.open(CONFIG_FILE, 'w').close
    File.open(IGNORE_FILE, 'w').close
    initialize_kic!
    assert_true File.zero?(CONFIG_FILE)
    assert_true File.zero?(IGNORE_FILE)
  end

  def test_destroy_should_be_interrupted
    initialize_kic!

    destroy_kic! 'no'
    assert_true base_dir_exists?
  end
end
