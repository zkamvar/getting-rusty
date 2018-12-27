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

fn main() {
    let number_list = vec![38, 203, 38, 2837, 39, 7];
    println!("the largest number is {}", largest(&number_list));

    let number_list = vec![02, 1, 24, 3, 5, 1, 100, 8, 3, 33, 5];

    println!("the largest number is {}", largest(&number_list));
    
    let char_list = vec!['a', 'f', 'w', 't', 'h'];

    println!("the largest character is {}", largest_char(&char_list));
    println!("the largest number is {}", largest_i32(&number_list));
}


