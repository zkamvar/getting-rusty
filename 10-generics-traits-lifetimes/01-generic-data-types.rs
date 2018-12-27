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

fn main() {
    let number_list = vec![38, 203, 38, 2837, 39, 7];
    println!("the largest number is {}", largest(&number_list));

    let number_list = vec![02, 1, 24, 3, 5, 1, 100, 8, 3, 33, 5];

    println!("the largest number is {}", largest(&number_list));
    
    let char_list = vec!['a', 'f', 'w', 't', 'h'];

    println!("the largest character is {}", largest_char(&char_list));
    println!("the largest number is {}", largest_i32(&number_list));

    // all types must be the same if the struct only has one type
    let pint = Pint { x: 5, y: 10};
    let pint = Pint { x: 5.0, y: 10.0};
    // let wont_work = Pint { x: 5.0, y: 10 }`

    let point = Point { x: 5, y: 10};
    let point = Point { x: 5.0, y: 10.0};
    let will_work = Point { x: 5.0, y: 10 };
}


