require 'test/unit'

class TestSample < Test::Unit::TestCase
  def setup
    @sample = 'sample'
  end

  def teardown
    @sample = nil
  end

  def test_sample
    assert_equal 'sample', @sample
  end
end
