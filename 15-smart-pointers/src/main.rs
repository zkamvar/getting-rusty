// The Box<T> immutable pointer: one owner -------------------------------------
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>), // Box<List> is a pointer to the next List enum
    Nil,
}

// The Rc<T> immutable pointer: many owners ------------------------------------
use std::rc::Rc;
#[derive(Debug)]
enum Node {
    Cons(i32, Rc<Node>),
    Nil,
}

// Demonstration of Deref ------------------------------------------------------
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    // associated type: slightly different way of declaring generic parameter
    // (see chapter 19)
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // index into a Tuple. In this case, MyBox is a length 1 tuple.
        &self.0
    }
}

// Demonstration of Drop trait -------------------------------------------------
#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!(
            "\nㅠ_ㅠ Dropping CustomSmartPointer with data `{}`",
            self.data
        );
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

    println!("## The `Drop` Trait: Running Code on Cleanup ------\n");

    let c = CustomSmartPointer {
        data: String::from("my string"),
    };
    let d = CustomSmartPointer {
        data: String::from("your string"),
    };

    println!("CustomSmartPointers {:?} and {:?} created", c, d);
    drop(c);
    println!("pointer for c dropped before the end of main\n");

    println!("## `Rc<T>, Reference Counted Pointer`");
    // The reference counted pointer accounts for _multiple_ owners of a given
    // value. This is common for graphs.
    let terminus = Rc::new(Node::Cons(10, Rc::new(Node::Nil)));
    let a = Rc::new(Node::Cons(5, terminus));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Node::Cons(3, Rc::clone(&a));
    println!("count after creating a = {}", Rc::strong_count(&a));
    {
        let c = Node::Cons(4, Rc::clone(&a));
        println!("count after creating a = {}", Rc::strong_count(&a));
        println!("Node {:?} is referenced in {:?} and {:?}", a, b, c);
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
