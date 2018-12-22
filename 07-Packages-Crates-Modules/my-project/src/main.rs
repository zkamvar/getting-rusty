mod sound {
    pub mod instrument {
        pub mod woodwind {
            pub fn clarinet() {
                println!("toot");
            }
        }
        pub mod string {
            pub fn guitar() {
                println!("meedlymeeldymeedlyWAAAAAAAA");
            }
        }
    }
    pub mod voice {
        pub fn soprano () {
          println!("EEEEE");  
        }
    }
}
fn main() {
    crate::sound::instrument::woodwind::clarinet();
    sound::instrument::string::guitar();
    sound::voice::soprano();
}
