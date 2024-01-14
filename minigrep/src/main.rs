use minigrep::Config;
use std::env;
use std::process;
// use std::ffi;

fn main() {
    // using env::args instead of just args is less ambiguous
    // To deal with invalid unicode, use std::env::args_os, which
    // will produce OsString values.
    // let args: Vec<ffi::OsString> = env::args_os().collect();
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for: `{}`", config.query);
    println!("in file      :  {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    };
}
