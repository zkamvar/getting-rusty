// Note: these are both needed in order for rust to recognise these as modules
mod sound; 
mod plant; 

// Bringin modules into scope
use crate::sound::instrument; // absolute path
use self::sound::voice;       // relative path
use self::plant::Vegetable;

fn main() {
    // Part 1
    instrument::woodwind::clarinet(); 
    instrument::string::guitar();
    voice::soprano();
    // Part 2
    let mut v = Vegetable::new("squash");
    v.name = String::from("Butternut squash");
    println!("{} are delicious!", v.name);
    println!("The vegetable struct is\n{:#?}\nThe name is public, but the id is private", v);
    // println!("ID: {}", v.id); // id is private
}
