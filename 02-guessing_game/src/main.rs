extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // IDK if it helps to use explicit typing here, but It's good to remind
    // myself what's going on.
    let secret_number: u32 = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please enter your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!!!!!11one!!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("this porridge is too cold!"),
            Ordering::Greater => println!("this porridge is too hot!"),
            Ordering::Equal   => {
                println!("this porridge is ~~just right~~");
                break;
            },
        }
    }
}
