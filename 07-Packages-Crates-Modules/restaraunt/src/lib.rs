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

// Nested modules create a tree from the root of the crate
//
// crate/front_of_house/hosting/add_to_waitlist
mod front_of_house {
    // Public elements must be declared all down the tree
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// Bring path into scope
// use crate::front_of_house::hosting;

// Re-export hosting path.
pub use self::front_of_house::hosting;

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
