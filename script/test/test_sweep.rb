require_relative 'helper'

class TestSweep < TestWithBasicSetup
  @@command_sweep            = "sweep"
  @@command_sweep_indeed     = "sweep indeed"
  @@command_sweep_all        = "sweep all"
  @@command_sweep_all_indeed = "sweep all indeed"

  def test_sweep_should_delete_dust_files
  end

  def test_sweep_all_should_delete_dust_files_with_latest_files
  end
end
