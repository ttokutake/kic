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
  - Instantly move of "dust" file
  - Directory which is forbid to run

## Installation

TBD

## Basic Usage

### Preliminary

- We use `tree` command for easily explanation.

### Hello, "kic"!

1. Change directory which you want to register.
2. `$ kic init`
3. Confirm `.kic` directory and essential files have been created in current directory.
4. Confirm initially existing files have written in "ignore" file.

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

$ cat .kic/ignore
./dir1/file2
./file1
```

### Try to move "dust" files manually

1. Make "dust" files.
2. `$ kic sweep all` (dry-run)
3. Check "dust" files are not moved to "dust box".
4. `$ kic sweep all indeed`
5. Check "dust" files have been moved to "dust box".

```bash
$ touch file3 dir1/file4
.
├── .kic
│   ├── config.toml
│   ├── ignore
│   └── warehouse
├── dir1
│   ├── file2
│   └── file4
├── file1
└── file3

3 directories, 6 files

$ kic sweep all
INFO: Create "2016-05-16" directory in ".kic/warehouse"
INFO: Create "sweep.log" file in ".kic/warehouse/2016-05-16"
INFO: Read "config.toml" file
INFO: Get the parameter for "sweep.moratorium"
INFO: Read "ignore" file
INFO: Move dusts to ".kic/warehouse/2016-05-16/dusts" (dry-run mode)
INFO:   => "./dir1/file4"
INFO:   => "./file3"
INFO: Move empty dirs to ".kic/warehouse/2016-05-16/dusts" (dry-run mode)

$ tree -a
.
├── .kic
│   ├── config.toml
│   ├── ignore
│   └── warehouse
│       └── 2016-05-16
│           ├── dusts
│           └── sweep.log
├── dir1
│   ├── file2
│   └── file4
├── file1
└── file3

5 directories, 7 files

$ kic sweep all indeed
INFO: Create "2016-05-16" directory in ".kic/warehouse"
INFO: Create "sweep.log" file in ".kic/warehouse/2016-05-16"
INFO: Read "config.toml" file
INFO: Get the parameter for "sweep.moratorium"
INFO: Read "ignore" file
INFO: Move dusts to ".kic/warehouse/2016-05-16/dusts"
INFO:   => "./dir1/file4"
INFO:   => "./file3"
INFO: Move empty dirs to ".kic/warehouse/2016-05-16/dusts"

$ tree -a
.
├── .kic
│   ├── config.toml
│   ├── ignore
│   └── warehouse
│       └── 2016-05-16
│           ├── dusts
│           │   ├── dir1
│           │   │   └── file4
│           │   └── file3
│           └── sweep.log
├── dir1
│   └── file2
└── file1

6 directories, 7 files
```

### Why we used `all` option above example?

1. `$ kic sweep` (dry-run)
2. Confirm "dust" files do not appear in the list.
3. Confirm there is the moratorium Until "dust" files are moved to "dust box" (Default: 10 minutes).

```bash
$ touch file3 dir1/file4

$ kic sweep
INFO: Create "2016-05-16" directory in ".kic/warehouse"
INFO: Create "sweep.log" file in ".kic/warehouse/2016-05-16"
INFO: Read "config.toml" file
INFO: Get the parameter for "sweep.moratorium"
INFO: Read "ignore" file
INFO: Move dusts to ".kic/warehouse/2016-05-16/dusts" (dry-run mode)
INFO: Move empty dirs to ".kic/warehouse/2016-05-16/dusts" (dry-run mode)

$ cat .kic/config.toml

[burn]
moratorium = "2 weeks"

[sweep]
moratorium = "10 minutes"
period = "daily"
time = "00:00"

### ... (10 minutes later)
$ kic sweep
INFO: Create "2016-05-16" directory in ".kic/warehouse"
INFO: Create "sweep.log" file in ".kic/warehouse/2016-05-16"
INFO: Read "config.toml" file
INFO: Get the parameter for "sweep.moratorium"
INFO: Read "ignore" file
INFO: Move dusts to ".kic/warehouse/2016-05-16/dusts" (dry-run mode)
INFO:   => "./dir1/file4"
INFO:   => "./file3"
INFO: Move empty dirs to ".kic/warehouse/2016-05-16/dusts" (dry-run mode)

$ kic sweep indeed
INFO: Create "2016-05-16" directory in ".kic/warehouse"
INFO: Create "sweep.log" file in ".kic/warehouse/2016-05-16"
INFO: Read "config.toml" file
INFO: Get the parameter for "sweep.moratorium"
INFO: Read "ignore" file
INFO: Move dusts to ".kic/warehouse/2016-05-16/dusts"
INFO:   => "./dir1/file4"
INFO:   => "./file3"
INFO: Move empty dirs to ".kic/warehouse/2016-05-16/dusts"

### Enable to change "moratorium"
$ kic config set sweep.moratorium 0minute
INFO: Read "config.toml" file
INFO: Set the parameter for "sweep.moratorium"
INFO: Create "config.toml" file

$ touch file3 dir1/file4

$ kic sweep
INFO: Create "2016-05-16" directory in ".kic/warehouse"
INFO: Create "sweep.log" file in ".kic/warehouse/2016-05-16"
INFO: Read "config.toml" file
INFO: Get the parameter for "sweep.moratorium"
INFO: Read "ignore" file
INFO: Move dusts to ".kic/warehouse/2016-05-16/dusts" (dry-run mode)
INFO:   => "./dir1/file4"
INFO:   => "./file3"
INFO: Move empty dirs to ".kic/warehouse/2016-05-16/dusts" (dry-run mode)
```

### Try to delete "dust box" manually

1. Confirm there is the moratorium Until "dust box" are deleted. (Default: 2 weeks)
2. Change the moratorium for confirmation.
3. Rename "box" name for confirmation.
4. `$ kic burn` (dry-run)
5. Check "box" is not deleted from "warehouse".
6. `$ kic burn indeed`
7. Check "box" has been deleted from "warehouse".

```bash
$ cat .kic/config.toml 

[burn]
moratorium = "2 weeks"

[sweep]
moratorium = "0 minute"
period = "daily"
time = "00:00"

$ kic config set burn.moratorium 1day
INFO: Read "config.toml" file
INFO: Set the parameter for "burn.moratorium"
INFO: Create "config.toml" file

### Rename the directory which has been named as today to yesterday
$ mv .kic/warehouse/2016-05-16/ .kic/warehouse/2016-05-15/

$ kic burn
INFO: Read "config.toml" file
INFO: Get the parameter for "burn.moratorium"
INFO: Create "2016-05-16" directory in ".kic/warehouse"
INFO: Create "burn.log" file in ".kic/warehouse/2016-05-16"
INFO: Delete expired dusts (dry-run mode)
INFO:   => ".kic/warehouse/2016-05-15"

$ tree -a
.
├── .kic
│   ├── config.toml
│   ├── ignore
│   └── warehouse
│       ├── 2016-05-15
│       │   ├── dusts
│       │   │   ├── dir1
│       │   │   │   └── file4
│       │   │   └── file3
│       │   └── sweep.log
│       └── 2016-05-16
│           ├── burn.log
│           └── dusts
├── dir1
│   └── file2
└── file1

8 directories, 8 files

$ kic burn indeed
INFO: Read "config.toml" file
INFO: Get the parameter for "burn.moratorium"
INFO: Create "2016-05-16" directory in ".kic/warehouse"
INFO: Create "burn.log" file in ".kic/warehouse/2016-05-16"
INFO: Delete expired dusts
INFO:   => ".kic/warehouse/2016-05-15"

$ tree -a
.
├── .kic
│   ├── config.toml
│   ├── ignore
│   └── warehouse
│       └── 2016-05-16
│           ├── burn.log
│           └── dusts
├── dir1
│   └── file2
└── file1

5 directories, 5 files

### Action like below is also OK!
$ rm -rf .kic/warehouse/2016-05-16/
```

### Register/Unregister with/from `cron`

TBD

### Add/Delete files or directories to/from "ignore"

TBD

### Useful sub-command of "ignore"

TBD

### Help me!

TBD

### Bye Bye, "kic"!

TBD
