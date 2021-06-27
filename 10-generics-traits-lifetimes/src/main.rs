#![allow(unused_variables)]
// Generics, are Rust's way of handling duplicate concepts.
//
// This lesson will cover
//
// 1. How to extract a function to reduce duplication
// 2. How construct a generic function that differ only in the parameter types
// 3. How to use generic types in struct and enum definitions

fn largest(list: &[i32])-> i32 {
// fn largest<T>(list: &[T])-> T {
    let mut largest = list[0];
    for &number in list.iter() {
        if number > largest {
            largest = number;
        }
    }
    largest
}
// Generic Data Types in function definitions ----------------------------------

fn largest_i32(list: &[i32])-> i32 {
    let mut largest = list[0];
    for &number in list.iter() {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn largest_char(list: &[char])-> char {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Type naming conventions ----------------------------------------------------
//
// Rust's Type naming conventions are CamelCase, but often the default will be
// just a single letter, "T" for "Type".
//
// NOTE: This will _not_ compile because of the fact that we use the greater 
// than operator, which needs a type that can be ordered. At the moment, our 
// definition allows _ANYTHING_ including unordered types.

// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];
//     for &number in list.iter() {
//         if number > largest {
//             largest = number;
//         }
//     }
//     largest
// }

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
    println!("the largest character is {}", largest_char(&char_list));
    println!("the largest number is {}", largest_i32(&number_list));

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


