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

pub fn eat_at_restaraunt() {
    // Order a breakfast in the summer with Rye toast
    let mut meal  = back_of_house::Breakfast::summer("Rye");
    let mut drink = back_of_house::Beverage::Coffee;
    // Change our mind about what break we'd like at the last minute
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast, plz", meal.toast);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
