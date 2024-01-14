use std::env;
// use std::ffi;

fn main() {
    // using env::args instead of just args is less ambiguous
    // To deal with invalid unicode, use std::env::args_os, which
    // will produce OsString values.
    // let args: Vec<ffi::OsString> = env::args_os().collect();
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}
