// Unlike structs, enums define the *possible* values something could take. This
// allows us to define one function that can take either type, but still have
// the same signature.
enum IpAddr {
    V4(u8, u8, u8, u8), // V4 variant: u8 forces the integer to be between 0 and 255
    V6(String), // V6 variant
}
// let home = IpAddr::V4(127, 0, 0, 0);
//
// let loopback = IpAddr::V6(String::from("::1"));

enum Message {
    Quit,
    Move { x: i32, y: i32 }, // this includes an anonymous struct
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Enums can also have methods associated
impl Message {
    fn call(&self) {
        // something goes here, but it's a bit of a mystery to me
    }
}

// The Option enum ------------------------------------------------------------
// 
// This is an enum that allows for some or none of something to be defined:

fn main() {
    let m = Message::Write(String::from("hello"));
    // m.call();
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None; // This is none for now, but could be an i32 later
}
