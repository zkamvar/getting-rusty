use std::env;
use std::fs;
use std::process;
// use std::ffi;

fn main() {
    // using env::args instead of just args is less ambiguous
    // To deal with invalid unicode, use std::env::args_os, which
    // will produce OsString values.
    // let args: Vec<ffi::OsString> = env::args_os().collect();
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for: `{}`", config.query);
    println!("in file      :  {}", config.file_path);

    let contents = read_file(config);

    println!("With text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Expected two arguments: <string> and <file>.");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

fn read_file(cfg: Config) -> String {
    let path = cfg.file_path.as_str();
    fs::read_to_string(path).expect("Should have been able to read the file")
}
