use std::io;
use std::io::ErrorKind; // For treating different kinds of errors
use std::io::Read;
use std::fs::File;

fn main() {
    // panic_at_the_disco();
    // wait_panic();
    // use_expect();
    let f = read_username_from_file().expect("no file!?");
    println!("woo! {}", f);
}

// Panic if something... anything is wrong -------------------------------------
fn panic_at_the_disco() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(errrr) => {
            panic!("the disco: {:?}", errrr);
        },
    };
}

// Wait and check the situation and then panic ---------------------------------
fn wait_panic() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(errrr) => match errrr.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(newf) => {
                    println!("I just created hello.txt");
                    newf
                },
                Err(bork) => {
                    panic!("There wasn't a file, so I tried to create one, but got another error: {:?}", bork);
                },
            },
            other_error => panic!("the disco: {:?}", other_error),
        },
    };
}

// Using expect method ---------------------------------------------------------
fn use_expect() {
    let f = File::open("hello.txt").expect("the disco");
}

// Error propogation -----------------------------------------------------------
// This passes the Result type to the parent function so that it can figure out
// how best to deal with the error
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(yeah) => yeah,
        Err(wut) => return Err(wut),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(dang) => return Err(dang),
    }
}
