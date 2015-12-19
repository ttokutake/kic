use std::env;

fn main() {
    let args = env::args().skip(1).collect::<Vec<String>>();

    if args.is_empty() {
        print_usage();
        return
    }

    match args[0].as_ref() {
        "help"    => print_usage(),
        "init"    => initialize(),
        "set"     => set_params(),
        "clean"   => clean(),
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
    start   # Start \"{}\"
    end     # End \"{}\"
    set     # Set parameters.
    clean   # Clean current directory.
    destroy # Destroy \"{}\"",
        bin_name,
        bin_name,
        bin_name,
        bin_name,
    );
}

fn initialize() {
    println!("initialize");
}

fn set_params() {
    println!("set parameters");
}

fn clean() {
    println!("clean");
}

fn register_with_cron() {
    println!("register");
}

fn unregister_cron() {
    println!("unregister");
}

fn destroy() {
    println!("destroy");
}
