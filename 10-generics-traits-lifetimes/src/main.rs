#![allow(unused_variables)]
// Generics, are Rust's way of handling duplicate concepts.
//
// This lesson will cover
//
// 1. How to extract a function to reduce duplication
// 2. How construct a generic function that differ only in the parameter types
// 3. How to use generic types in struct and enum definitions

// Listing 10-4 ---------------------------------------------------------------
// both of these functions are pinned for a specific type
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}
fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}

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

// If we don’t want to restrict the largest function to the types that implement
// the Copy trait, we could specify that T has the trait bound Clone instead of
// Copy. Then we could clone each value in the slice when we want the largest
// function to have ownership. Using the clone function means we’re potentially
// making more heap allocations in the case of types that own heap data like
// String, and heap allocations can be slow if we’re working with large amounts
// of data.
fn largest_clone<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();
    for i in 0..list.len() {
        let item = list[i].clone();
        if item > largest {
            largest = item;
        }
    }
    largest
}
// ZHIAN DOES NOT KNOW HOW TO IMPLEMENT THESE
//
// Another way we could implement largest is for the function to return a
// reference to a T value in the slice. If we change the return type to &T
// instead of T, thereby changing the body of the function to return a
// reference, we wouldn’t need the Clone or Copy trait bounds and we could avoid
// heap allocations. Try implementing these alternate solutions on your own!

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
use chapter10::{self, NewsArticle, Summary, Tweet};

// Lifetimes ------------------------------------------------------------------
use chapter10::FirstSentence;
// These are ways of declaring how long a variable should live beyond the rust
// scope.
/*
 * This does not compile: The return value needs a lifetime parameter
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

 * The reason for this is because it is impossible to know how the lifetime of
 * x and y relate to the lifetime of the return value. It makes sense if you
 * remember about how functions will always take ownership of a reference.
 *
 * If we have this pattern, it's not clear what the lifetime of result will be.
 * let x = "bennigans";
 * {
 *    let y = "shenanigans";
 *    let result = longest(x.as_str(), y.as_str());
 *    println!("both x and y would be valid here");
 * }
 * println!("only x would be vaild here");
 */
// Function signature with lifetime annotation
//
// Constraint: returned reference will be valid as long as both the parameters
// are valid.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// Note that lifetime is in relation to the _inputs_ A function cannot return
// a reference created _within_ the function
/*
fn longest<'a>(x: &str, y: &str) -> &'a str {
   let result = String::from("really long string");
   result.as_str()
}
 */

// In Structs, lifetime references need to be added for every reference inside
// the definition:
struct ImportantExcerpt<'a> {
    part: &'a str,
}
// Lifetime names for strct fields always need to be declared after the impl
// keyword and after the struct's name because those are part of
// the struct's type
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

fn main() {
    // Listing 10-1 ------------------------------
    //
    // This is an example of code that should be encapsulated in a function and
    // deduplicated. This is fixed in the `largest()` function
    let number_list = vec![34, 50, 25, 100, 65];

    let mut the_largest = &number_list[0];

    for number in &number_list {
        if number > the_largest {
            the_largest = number;
        }
    }
    println!("The largest number is {}", the_largest);

    // Functions ----------------------------------
    // use a non-generic function: one that requires a _specific_ input type
    let number_list = vec![38, 203, 38, 2837, 39, 7];
    println!("the largest number is {}", largest_i32(&number_list));

    // -- Generic Data Types --
    // Listing 10-4 -------------------------------
    // We can define a function that takes integers
    let number_list = vec![02, 1, 24, 3, 5, 1, 100, 8, 3, 33, 5];
    println!("the largest number is {}", largest_i32(&number_list));
    // We can also define a function that takes characters
    let char_list = vec!['a', 'f', 'w', 't', 'h'];
    println!("the largest character is {}", largest_char(&char_list));

    // We can also define a GENERIC function that will take both
    println!("the largest number is {}", largest(&number_list));
    println!("the largest character is {}", largest(&char_list));

    println!("the largest number is {}", largest_clone(&number_list));

    // Structs ---
    // all types must be the same if the struct only has one type
    let pint = Pint { x: 5, y: 10 };
    // Generic method works, but concrete method not defined for i32
    println!("pint.x = {}", pint.x());
    // println!("distance from origin: {}", pint.distance_from_origin());
    let pint = Pint { x: 3.0, y: 4.0 };
    println!("pint.x = {}", pint.x());
    println!("distance from origin: {}", pint.distance_from_origin());
    // let wont_work = Pint { x: 5.0, y: 10 }`

    let point = Point { x: 5, y: 10 };
    let point = Point { x: 5.0, y: 10.0 };
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

    println!("\n\n--- *** ---\n LIFETIMES\n-----------\n");
    // Main aim of lifetimes is to prevent _dangling references_:
    /* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
                          // ---------+
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */

    println!("\n---> Functions\n------------");
    // Generic lifetimes in functions
    let string1 = String::from("bennigans");
    let string2 = "shennanigans";
    let result1 = longest(string1.as_str(), string2);
    println!("The longest string is {}", result1);
    {
        // the 'a lifetime is the part of the scope that _overlaps_,
        // so the result here is in scope for longer;
        let string2 = String::from("I am the viper");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
    // This is no longer in scope, so it will fail.
    // println!("The longest string is {}", result);
    // This is still in scope, we are still returning shennanigans
    println!("The longest string is {}", result1);
    println!("\n---> Structs\n------------");
    // An instance of a struct cannot outlive the reference it holds in its
    // fields.
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    // novel does not go out of scope here, so the reference inside of
    // ImportantExcerpt is valid.
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    let j = FirstSentence {
        part: first_sentence,
    };

    println!("Important Excerpt (level {}): {}", i.level(), i.part);
    println!("{}", j.summarize());
}
