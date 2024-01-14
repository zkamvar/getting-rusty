use std::env;
use std::fs;
// use std::ffi;

fn main() {
    // using env::args instead of just args is less ambiguous
    // To deal with invalid unicode, use std::env::args_os, which
    // will produce OsString values.
    // let args: Vec<ffi::OsString> = env::args_os().collect();
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for: `{}`", query);
    println!("in file      :  {}", file_path);

    let contents = read_file(file_path.as_str());

    println!("With text:\n{contents}");
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("Should have been able to read the file")
}
