#!/bin/bash

my_dir=$(dirname ${BASH_SOURCE:-$0})

bundle exec ruby $my_dir/test/test_init_and_destroy.rb
bundle exec ruby $my_dir/test/test_config.rb
bundle exec ruby $my_dir/test/test_ignore.rb
bundle exec ruby $my_dir/test/test_sweep.rb
bundle exec ruby $my_dir/test/test_burn.rb
bundle exec ruby $my_dir/test/test_start_and_end.rb
