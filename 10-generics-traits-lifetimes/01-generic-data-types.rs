// Generics, are Rust's way of handling duplicate concepts.
//
// This lesson will cover
//
// 1. How to extract a function to reduce duplication
// 2. How construct a generic function that differ only in the parameter types
// 3. How to use generic types in struct and enum definitions

fn main() {
    let number_list = vec![38, 203, 38, 2837, 39, 7];
    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("the largest number is {}", largest);

    let number_list = vec![02, 1, 24, 3, 5, 1, 100, 8, 3, 33, 5];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("the largest number is {}", largest);
}
