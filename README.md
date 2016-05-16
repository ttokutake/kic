# KIC (Keep It Clean)

[![Build Status](https://travis-ci.org/ttokutake/kic.svg?branch=master)](https://travis-ci.org/ttokutake/kic)

## Abstract

- "kic" prevents someone from making a mess of directories.
- For registered directories, "kic" autonomously moves "dust" files into "dust box".
- "dust" is file which is not specified as a requirement in "kic".
 
## Notice

- "kic" uses your `cron` in UNIX platform.
- "kic" is incomplete for Windows platform.
  - Autonomous deletion
  - Judgement of a hidden file
  - Directory which is forbid to run

## Installation

TBD

## Basic Usage

### Initialize

1. Change directory which you want to register.
2. `$ kic init`
3. Confirm `.kic` directory and essential files have created in current directory.
```bash
$ pwd
/Users/tokutake/tmp

$ tree -a
.
├── dir1
│   └── file2
└── file1

1 directory, 2 files

$ kic init
INFO: Create ".kic" directory
INFO: Create "warehouse" directory
INFO: Create "config.toml" file
INFO: Create "ignore" file

$ tree -a
.
├── .kic
│   ├── config.toml
│   ├── ignore
│   └── warehouse
├── dir1
│   └── file2
└── file1

3 directories, 4 files
```

