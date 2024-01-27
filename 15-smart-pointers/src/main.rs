#[derive(Debug)]
enum List {
    Cons(i32, Box<List>), // Box<List> is a pointer to the next List enum
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;
impl<T> Deref for MyBox<T> {
    // associated type: slightly different way of declaring generic parameter
    // (see chapter 19)
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // index into a Tuple. In this case, MyBox is a length 1 tuple.
        &self.0
    }
}

use crate::List::{Cons, Nil};

fn main() {
    println!("## Box<T> will store data on the heap ------ \n");
    let b = Box::new(5);
    println!("b = {}", b);

    println!("---| Enabling recursive types with Boxes");
    // at compile time, it's not clear how much space a recursive type will need,
    // so a Box helps Rust understand that it can allocate memory from the heap
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    let list3 = Cons(3, Box::new(Nil));
    let list2 = Cons(2, Box::new(list3));
    let list1 = Cons(1, Box::new(list2));
    println!("We've got a list {:?}\nand another list {:?}", list, list1);

    println!("## Treating Smart Pointers with the `Deref` Trait ------\n");

    let x = 5;
    let y = &x; // this is equivalent: let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    println!("---| Defining our own smart pointer\n");

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *(y.deref()));
}
