# KIC (Keep It Clean)

[![Build Status](https://travis-ci.org/ttokutake/kic.svg?branch=master)](https://travis-ci.org/ttokutake/kic)

## Links

- https://crates.io/crates/kic

## Abstract

- "kic" prevents someone from making a mess of directories.
- For registered directories, "kic" automatically moves "dust" files into "dust box".
- "dust" is file which is not specified as a requirement in "kic".
 
## Notice

- "kic" uses your `cron` in UNIX platform.
- "kic" is incomplete for Windows platform.
  - Autonomous deletion
  - Judgement of a hidden file
  - Instantly move of "dust" file
  - Directory which is forbid to run

## Installation

1. Please install **stable** version of [rust](https://www.rust-lang.org/index.html). See [here](https://www.rust-lang.org/downloads.html).
2. `$ cargo install kic`
3. Add "~/.cargo/bin" directory to `$PATH`.
 
### Example for MacOS

```bash
$ curl -sSf https://static.rust-lang.org/rustup.sh | sh

$ cargo install kic

$ echo 'PATH=$PATH:$HOME/.cargo/bin' >> $HOME/.bash_profile && source $HOME/.bash_profile
```

## Update the package

1. `$ cargo uninstall kic`
2. `$ cargo install kic`

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

1. Make "dust" files.
2. `$ kic sweep` (dry-run)
3. Confirm "dust" files do not appear in the list.
4. Confirm there is the moratorium Until "dust" files are moved to "dust box" (Default: 10 minutes).

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

1. `$ kic start`
2. Check several "kic" commands have added to your cron.
3. Check running time of command in cron change by "config.toml".
4. `$ kic end`
5. Check "kic" commands related to current directory have deleted from your cron.

```bash
$ kic start
INFO: Read cron
INFO: Read "config.toml" file
INFO: Get the parameter for "sweep.period"
INFO: Get the parameter for "sweep.time"
INFO: Set new cron

$ crontab -l
###################################
# "kic" uses the lines from this.
# Please don't touch them and me!
###################################
0 12 * * *	/Users/tokutake/codes/kic/target/debug/kic patrol
0 0 * * *	cd /Users/tokutake/tmp && /Users/tokutake/codes/kic/target/debug/kic burn indeed
0 0 * * *	cd /Users/tokutake/tmp && /Users/tokutake/codes/kic/target/debug/kic sweep indeed
###################################
# "kic" uses the lines up to here.
# Please don't touch them and me!
###################################

$ kic config set sweep.period weekly
INFO: Read "config.toml" file
INFO: Set the parameter for "sweep.period"
INFO: Create "config.toml" file

$ kic config set sweep.time 14:00
INFO: Read "config.toml" file
INFO: Set the parameter for "sweep.time"
INFO: Create "config.toml" file

$ kic start
INFO: Read cron
INFO: Read "config.toml" file
INFO: Get the parameter for "sweep.period"
INFO: Get the parameter for "sweep.time"
INFO: Set new cron

$ crontab -l
###################################
# "kic" uses the lines from this.
# Please don't touch them and me!
###################################
0 12 * * *	/Users/tokutake/codes/kic/target/debug/kic patrol
0 0 * * *	cd /Users/tokutake/tmp && /Users/tokutake/codes/kic/target/debug/kic burn indeed
0 14 * * 0	cd /Users/tokutake/tmp && /Users/tokutake/codes/kic/target/debug/kic sweep indeed
###################################
# "kic" uses the lines up to here.
# Please don't touch them and me!
###################################

$ kic end
INFO: Read cron
INFO: Set new cron

$ crontab -l
### There is no contents
```

### Add/Delete files or directories to/from "ignore"

1. Make non-"dust" files.
2. Add the file to "ignore".
3. Check the file is not listed.
4. Make a directory and non-"dust" files in it.
5. Add the directory to "ignore".
6. Check the files are not listed.
7. Delete files in the directory.
8. Check the empty directory is listed regardless of "ignore".
9. Add an empty hidden file if you want to leave empty directories.

```bash
$ touch file3 file4

$ kic ignore add file3 file4
INFO: Read "ignore" file
INFO: Create "ignore" file

$ kic sweep all
INFO: Create "2016-05-17" directory in ".kic/warehouse"
INFO: Create "sweep.log" file in ".kic/warehouse/2016-05-17"
INFO: Read "config.toml" file
INFO: Get the parameter for "sweep.moratorium"
INFO: Read "ignore" file
INFO: Move dusts to ".kic/warehouse/2016-05-17/dusts" (dry-run mode)
INFO: Move empty dirs to ".kic/warehouse/2016-05-17/dusts" (dry-run mode)

$ mkdir dir2 && touch dir2/file{5,6}

### If you are leaving "dir1", specifying "dir1" is also OK!
$ kic ignore add dir2
INFO: Read "ignore" file
INFO: Create "ignore" file

$ kic sweep all
INFO: Create "2016-05-17" directory in ".kic/warehouse"
INFO: Create "sweep.log" file in ".kic/warehouse/2016-05-17"
INFO: Read "config.toml" file
INFO: Get the parameter for "sweep.moratorium"
INFO: Read "ignore" file
INFO: Move dusts to ".kic/warehouse/2016-05-17/dusts" (dry-run mode)
INFO: Move empty dirs to ".kic/warehouse/2016-05-17/dusts" (dry-run mode)

$ rm dir2/*

$ kic sweep all
INFO: Create "2016-05-17" directory in ".kic/warehouse"
INFO: Create "sweep.log" file in ".kic/warehouse/2016-05-17"
INFO: Read "config.toml" file
INFO: Get the parameter for "sweep.moratorium"
INFO: Read "ignore" file
INFO: Move dusts to ".kic/warehouse/2016-05-17/dusts" (dry-run mode)
INFO: Move empty dirs to ".kic/warehouse/2016-05-17/dusts" (dry-run mode)
INFO:   => "./dir2"

$ touch dir2/.kickeep

$ kic sweep all
INFO: Create "2016-05-17" directory in ".kic/warehouse"
INFO: Create "sweep.log" file in ".kic/warehouse/2016-05-17"
INFO: Read "config.toml" file
INFO: Get the parameter for "sweep.moratorium"
INFO: Read "ignore" file
INFO: Move dusts to ".kic/warehouse/2016-05-17/dusts" (dry-run mode)
INFO: Move empty dirs to ".kic/warehouse/2016-05-17/dusts" (dry-run mode)
```

### Useful sub-command of "ignore"

1. Make non-"dust" directories and files.
2. `$ kic ignore current`
3. Check the files are not listed.
4. Remove non-"dust" files.
5. `$ kic ignore refresh`
6. Check the non-existing files (and directories) have already not existed in "ignore".

```bash
$ touch file{5,6,7}

$ mkdir dir3 && dir3/file{8,9,10}

### NOTICE: If you have registered some directories in "ignore", They will be deleted from "ignore" and Files in them will be added to it.
$ kic ignore current
CAUTION: Do you want to preserve current state? [yes/no]: y
INFO: Create "ignore" file

$ kic sweep all
INFO: Create "2016-05-18" directory in ".kic/warehouse"
INFO: Create "sweep.log" file in ".kic/warehouse/2016-05-18"
INFO: Read "config.toml" file
INFO: Get the parameter for "sweep.moratorium"
INFO: Read "ignore" file
INFO: Move dusts to ".kic/warehouse/2016-05-18/dusts" (dry-run mode)
INFO: Move empty dirs to ".kic/warehouse/2016-05-18/dusts" (dry-run mode)

$ cat .kic/ignore 
./dir1/file2
./dir3/file10
./dir3/file8
./dir3/file9
./file1
./file3
./file4
./file5
./file6
./file7

$ rm file{5,6,7}

$ kic ignore refresh

$ cat .kic/ignore
./dir1/file2
./dir3/file10
./dir3/file8
./dir3/file9
./file1
./file3
./file4
```

### Help me!

1. Check general help message.
2. Check help message for each command.

```bash
$ kic
Usage:
    kic <Command>

Description:
    Keep your directories clean

Command:
    help    # Display usage for each command
    version # Display the version of this software
    init    # Register current directory, i.e. create ".kic" directory
    config  # Change "config.toml" file's contents
    ignore  # Change "ignore" file's contents
    sweep   # Move dust files and empty directories into "warehouse" directory
    burn    # Delete expired directories in "warehouse" directory
    start   # Start automatic "sweep" and "burn" (UNIX-like: cron, Windows: ?)
    end     # End automatic "sweep" and "burn" (UNIX-like: cron, Windows: ?)
    destroy # Unregister current directory, i.e. delete ".kic" directory
    patrol  # Keep your "cron" file clean (UNIX-like only)

$ kic help config
Usage:
    kic config set <Key> <Value>
    kic config init

Description:
    Change "config.toml" file's contents

Command:
    set  # Set parameters related to "sweep" and "burn" commands
    init # Initialize "config.toml" file

Keys:
    burn.moratorium  # Moratorium to delete directories in "warehouse"
    sweep.moratorium # Moratorium to Move "dust"s into "warehouse"
    sweep.period     # Period to Move "dust"s by automatic "sweep"
    sweep.time       # Time to Move "dust"s by automatic "sweep"
```

### Bye Bye, "kic"!

1. `$ kic destroy`
2. Check contents of cron is deleted if you have left it.

```bash
$ kic start
INFO: Read cron
INFO: Read "config.toml" file
INFO: Get the parameter for "sweep.period"
INFO: Get the parameter for "sweep.time"
INFO: Set new cron

$ kic destroy
CAUTION: Do you want to clear all files related to "kic"? [yes/no]: y
INFO: Delete ".kic" directory
INFO: Read cron
INFO: Set new cron

$ crontab -l
### There is no contents
```
