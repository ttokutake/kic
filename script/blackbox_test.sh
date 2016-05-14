#!/bin/bash

my_dir=$(dirname ${BASH_SOURCE:-$0})

test_file_names="
  test_init_and_destroy
  test_config
  test_ignore
  test_sweep
  test_burn
  test_cron
"

for test_file_name in $test_file_names; do
  bundle exec ruby $my_dir/test/${test_file_name}.rb
done
