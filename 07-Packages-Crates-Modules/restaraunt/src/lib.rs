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
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

// using the pub keyword to expose a function to the API
pub fn eat_at_restaraunt() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
