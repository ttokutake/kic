use std::env;

mod constants;
mod command;

use constants::*;
use command::*;

fn main() {
    let current_dir = env::current_dir().unwrap();
    match BANNED_DIRS.iter().find(|d| current_dir.ends_with(d)) {
        Some(dir) => {
            println!("Cannot run in \"{}\"", dir);
            return
        }
        None => {},
    }

    let args = env::args().skip(1).collect::<Vec<String>>();

    if args.is_empty() {
        print_usage();
        return
    }

    match args[0].as_ref() {
        "help"    => print_usage(),
        "init"    => initialize(),
        "set"     => set_params(),
        "sweep"   => sweep(),
        "start"   => register_with_cron(),
        "end"     => unregister_cron(),
        "destroy" => destroy(),
        _         => print_usage(),
    }
}

fn print_usage() {
    let full_path_to_bin = env::current_exe().unwrap();
    let bin_name         = full_path_to_bin.file_name()
        .unwrap()
        .to_str()
        .unwrap();
    println!(
"Usage:
    {} <command>

Commands:
    help    # Print this message.
    init    # Register current directory.
    set     # Set parameters.
    sweep   # Sweep files in current directory.
    start   # Start \"{}\"
    end     # End \"{}\"
    destroy # Destroy \"{}\"",
        bin_name,
        bin_name,
        bin_name,
        bin_name,
    );
}
