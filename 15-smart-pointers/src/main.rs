// The Box<T> immutable pointer: one owner -------------------------------------
#[derive(Debug)]
enum BList {
    BCons(i32, Box<BList>), // Box<BList> is a pointer to the next BList enum
    BNil,
}

// The Rc<T> immutable pointer: many owners ------------------------------------
use std::rc::Rc;
#[derive(Debug)]
enum RList {
    RCons(i32, Rc<RList>),
    RNil,
}

use std::cell::RefCell;
#[derive(Debug)]
enum CList {
    CCons(Rc<RefCell<i32>>, Rc<CList>),
    CNil,
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

use crate::BList::{BCons, BNil};
use crate::CList::{CCons, CNil};
use crate::RList::{RCons, RNil};

fn main() {
    println!("## Box<T> will store data on the heap ------ \n");
    let b = Box::new(5);
    println!("b = {}", b);

    println!("---| Enabling recursive types with Boxes");
    // at compile time, it's not clear how much space a recursive type will need,
    // so a Box helps Rust understand that it can allocate memory from the heap
    let list = BCons(1, Box::new(BCons(2, Box::new(BCons(3, Box::new(BNil))))));
    let list3 = BCons(3, Box::new(BNil));
    let list2 = BCons(2, Box::new(list3));
    let list1 = BCons(1, Box::new(list2));
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
    let terminus = Rc::new(RCons(10, Rc::new(RNil)));
    let a = Rc::new(RCons(5, terminus));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = RCons(3, Rc::clone(&a));
    println!("count after creating a = {}", Rc::strong_count(&a));
    {
        let c = RCons(4, Rc::clone(&a));
        println!("count after creating a = {}", Rc::strong_count(&a));
        println!("RList {:?} is referenced in {:?} and {:?}", a, b, c);
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    println!("`RefCell<T>` and the Interior Mutability Pattern");
    // RefCell<T> is a lot like Box<T> _except_ that if an ownership rule is
    // violated, it is violated at runtime and not compile time.
    //
    // The following is impossible, but there are situations where you will want
    // a value to be mutables _some of the time_:
    //
    // let x = 5;
    // let y = &mut x;

    // Using Rc<RefCell<i32>> allows the value to have multiple owners. The fact
    // that it is a ref cell allows usto mutate it.
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(CCons(Rc::clone(&value), Rc::new(CNil)));
    let b = Rc::new(CCons(Rc::new(RefCell::new(3)), Rc::clone(&a)));
    let c = Rc::new(CCons(Rc::new(RefCell::new(4)), Rc::clone(&a)));

    println!("a before = {:?}", a);
    println!("b before = {:?}", b);
    println!("c before = {:?}", c);

    let old = value
        .borrow() // need to borrow to clone the value inside.
        .clone();

    *value.borrow_mut() += 10;

    println!("old = {:?}, current = {:?}", old, value);

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
