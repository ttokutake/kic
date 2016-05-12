require_relative 'helper'

class TestBurn < TestWithBasicSetup
  @@command_burn        = 'burn'
  @@command_burn_indeed = 'burn indeed'

  def test_burn_should_not_delete_non_expired_dust_box
  end

  def test_burn_should_delete_expired_dust_box
  end
end
