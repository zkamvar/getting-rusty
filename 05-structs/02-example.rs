#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
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
}
fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("DEBUG:\trect is {:#?}", rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
