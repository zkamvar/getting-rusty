// Back of house demonstrates super module
//
// Note that these should move together to make sure that super:: works. 
fn serve_order() {}

mod back_of_house {
    // All elements of this enum are public
    pub enum Beverage {
        Coffee,
        Tea,
        Juice,
    }
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
        pub fn winter(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("persimmon"),
            }
        }
    }
}


mod front_of_house;

// Re-export hosting path.
pub use crate::front_of_house::hosting;

// Using the as syntax to provide a new name
// use self::front_of_house::hosting::add_to_waitlist as host_waitlist;

pub fn eat_at_restaraunt() {
    // use of hosting:: here is idiomatic to show that we are using an external
    // module to this function. 
    hosting::add_to_waitlist();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}