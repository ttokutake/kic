require 'test/unit'
require_relative 'test_helper'

class TestInitAndDestroy < Test::Unit::TestCase
  class << self
    def startup
      build!
    end
  end

  def teardown
    destroy_kic!
  end

  def test_basic_init_and_destroy
    assert !Dir.exists?(BASE_DIR)

    initialize_kic!
    assert Dir .exists?(BASE_DIR)
    assert Dir .exists?(STORAGE_DIR)
    assert File.exists?(CONFIG_FILE)
    assert File.exists?(IGNORE_FILE)

    destroy_kic!
    assert !Dir.exists?(BASE_DIR)
  end

  def test_init_should_not_delete_existing_config_and_ignore
    initialize_kic!

    File.open(CONFIG_FILE, 'w').close
    File.open(IGNORE_FILE, 'w').close
    initialize_kic!
    assert File.zero?(CONFIG_FILE)
    assert File.zero?(IGNORE_FILE)
  end
end
