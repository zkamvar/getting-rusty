// Back of house demonstrates super module
//
// Note that these should move together to make sure that super:: works. 
fn serve_order() {}

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
