// if let is a construct that allows you to match only a single value you care
// about. 
enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}

enum UsState {
    Alaska,
    Arkansas,
}
fn main() {
    let some_u8_value = Some(0u8);
    // using match
    match some_u8_value {
        Some(3) => println!("three"),
        _       => (),
    }
    // using if let
    if let Some(3) = some_u8_value {
        println!("three");
    }

    // Using if let in verbose situations
    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alaska));
    // using match ---------------------------
    match coin {
        Coin::Quarter(state) => {
            println!("I got a quarter from {:?}", state);
        },
        _ => count += 1,
    }
    // using if let allows for else ----------
    if let Coin::Quarter(state) = coin {
        println!("I got a quarter from {:?}", state);
    } else {
        count += 1;
    }
}
