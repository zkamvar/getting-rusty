mod back_of_house;
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
