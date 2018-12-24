use std::fs::File;
use std::io::ErrorKind; // For treating different kinds of errors

fn main() {
    // panic_at_the_disco();
    wait_panic();
}

fn panic_at_the_disco() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(errrr) => {
            panic!("the disco: {:?}", errrr);
        },
    };
}

fn wait_panic() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(errrr) => match errrr.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(newf) => newf,
                Err(bork) => {
                    panic!("There wasn't a file, so I tried to create one, but got another error: {:?}", bork);
                }
            },
            other_error => panic!("the disco: {:?}", other_error),
        },
    };

}
