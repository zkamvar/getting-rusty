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
