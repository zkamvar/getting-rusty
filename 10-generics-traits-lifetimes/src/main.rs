#![allow(unused_variables)]
// Generics, are Rust's way of handling duplicate concepts.
//
// This lesson will cover
//
// 1. How to extract a function to reduce duplication
// 2. How construct a generic function that differ only in the parameter types
// 3. How to use generic types in struct and enum definitions

// Type naming conventions ----------------------------------------------------
//
// Rust's Type naming conventions are CamelCase, but often the default will be
// just a single letter, "T" for "Type".
//
// The type here takes two _traits_ for items in the list:
//
//  1. PartialOrd: any types that can be ordered (strings, integers, floats)
//  2. Copy: types that are of a known size at compile time (strings, integers, floats)
//
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// If we don’t want to restrict the largest function to the types that implement the Copy trait,
// we could specify that T has the trait bound Clone instead of Copy. Then we could clone each
// value in the slice when we want the largest function to have ownership. Using the clone
// function means we’re potentially making more heap allocations in the case of types that own
// heap data like String, and heap allocations can be slow if we’re working with large amounts of
// data.
fn largest_clone<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();
    for i in 0..list.len() {
        let mut item = list[i].clone();
        if item > largest {
            largest = item;
        }
    }
    largest
}
// ZHIAN DOES NOT KNOW HOW TO IMPLEMENT THESE
//
// Another way we could implement largest is for the function to return a reference to a T value in
// the slice. If we change the return type to &T instead of T, thereby changing the body of the
// function to return a reference, we wouldn’t need the Clone or Copy trait bounds and we could
// avoid heap allocations. Try implementing these alternate solutions on your own!

// Structs and enums ----------------------------------------------------------

// A struct can be generic, but if it has one generic type, all types must be
// the same
struct Pint<T> {
    x: T,
    y: T,
}

// multiple fields in the struct can have different generic types
struct Point<T, U> {
    x: T,
    y: U,
}

// Method definitions
// generic type methods
impl<T> Pint<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
// concrete methods act on specific type
impl Pint<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Traits ---------------------------------------------------------------------
// These are a way to group method signatures together to define a set of
// behaviors necessary to accomplish the same purpose.
//
// Here, Summary is a trait on the Tweet Type. We can implement this because we
// have defined Tweet in our crate. But, we cannot implement this on a type that
// is outside of our crate.
use chapter10::{self, Summary, Tweet, NewsArticle};

fn main() {
    // Functions ---
    let number_list = vec![38, 203, 38, 2837, 39, 7];
    println!("the largest number is {}", largest(&number_list));

    let number_list = vec![02, 1, 24, 3, 5, 1, 100, 8, 3, 33, 5];
    println!("the largest number is {}", largest(&number_list));

    let char_list = vec!['a', 'f', 'w', 't', 'h'];
    println!("the largest character is {}", largest(&char_list));
    println!("the largest number is {}", largest_clone(&number_list));

    // Structs ---
    // all types must be the same if the struct only has one type
    let pint = Pint { x: 5, y: 10};
    // Generic method works, but concrete method not defined for i32
    println!("pint.x = {}", pint.x());
    // println!("distance from origin: {}", pint.distance_from_origin());
    let pint = Pint { x: 3.0, y: 4.0};
    println!("pint.x = {}", pint.x());
    println!("distance from origin: {}", pint.distance_from_origin());
    // let wont_work = Pint { x: 5.0, y: 10 }`

    let point = Point { x: 5, y: 10};
    let point = Point { x: 5.0, y: 10.0};
    let will_work = Point { x: 5.0, y: 10 };

    // Traits ---
    let tweet = Tweet {
        username: String::from("dril"),
        content: String::from(
            "No More Fooling Around: if your post sucks my balls , it will be regarded as SHIT !!!!",
        ),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet\n{}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("NEW SHOES"),
        location: String::from("Ragland"),
        author: String::from("Betch"),
        content: String::from(
            "These shoes are three hundred fucking dollars...\
            LET'S GET EM",
        ),
    };
    println!("SHOE NEWS: {}", article.summarize());
}


