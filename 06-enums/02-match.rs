// Simple matching with enums --------------------------------------------------
// Match pretty much does what it says on the tin. It matches things
#[derive(Debug)]
enum US_State {
    Alabama,
    Alaska,
    // --snip--
}
enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(US_State),
}

fn value_in_cents (coin: Coin) -> u32 {
    match coin {
        Coin::Penny  =>  1,
        Coin::Nickle =>  5,
        Coin::Dime   => 10,
        Coin::Quarter(state) =>  { // patterns can bind to values
            println!("State quarter from {:?}", state);    
            25
        },
    }
}
// value_in_cents(Coin::Quarter(US_State::Alaska))
//
// Matching using Option<T> ----------------------------------------------------
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None    => None, // This part is necessary because matching is exhaustive
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);
    let six  = plus_one(five);
    let none = plus_one(None);
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), // _ matches the rest and () are the goggles
    }
}
