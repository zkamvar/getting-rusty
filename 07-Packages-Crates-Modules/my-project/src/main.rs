mod sound {
    pub mod instrument {
        pub mod woodwind {
            pub fn clarinet() {
                super::breathe_in(); // super is equivalent to '..' in UNIX
                println!("toot");
            }
        }
        pub mod string {
            pub fn guitar() {
                println!("meedlymeeldymeedlyWAAAAAAAA");
            }
        }
        fn breathe_in() {
            println!("shpp");
        }
    }
    pub mod voice {
        pub fn soprano () {
          println!("EEEEE");  
        }
    }
}

mod plant {
    #[derive(Debug)]
    pub struct Vegetable {
        pub name: String, // only this part of the struct is visible
        id: i32,
    }
    impl Vegetable {
        pub fn new(name: &str) -> Vegetable {
            Vegetable {
                name: String::from(name),
                id: 1,
            }
        }
    }
}

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
