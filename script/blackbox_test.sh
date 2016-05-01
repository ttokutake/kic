#!/bin/bash

my_dir=$(dirname ${BASH_SOURCE:-$0})

ruby $my_dir/test/test_init_and_destroy.rb
