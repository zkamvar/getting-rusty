#[derive(Debug)]
// Struct -------------------------
struct Rectangle {
    width: u32,
    height: u32,
}
// Methods ------------------------
impl Rectangle {
    // Note: methods can take ownership, borrow immutably (here), and
    //       borrow mutably. Since we don't want to harm the Rectangle itself,
    //       we are borrowing immutably.
    //
    //  - Borrowing mutably allows us to change a part of the struct.
    //  - Taking ownership is rare, but may be used in conversion of types
    fn area (&self) -> u32 {
        self.width * self.height
    }
    // A test to see if the current rectangle can hold another rectangle
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
// Associated functions -----------
impl Rectangle {
    fn square (size: u32) -> Rectangle {
        Rectangle {width: size, height: size}
    }
}
fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };
    
    // Using an associated function
    let square1 = Rectangle::square(20);
    
    println!("DEBUG:\trect is {:#?}", rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can rect1 hold a {} px square? {}", square1.area(), rect1.can_hold(&square1));
}
