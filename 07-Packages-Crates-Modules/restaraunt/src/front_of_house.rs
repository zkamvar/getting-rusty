// Nested modules create a tree from the root of the crate
//
// crate/front_of_house/hosting/add_to_waitlist
// Public elements must be declared all down the tree
pub mod hosting; 

mod serving {
    fn take_order() {}

    fn serve_order() {}

    fn take_payment() {}
}
