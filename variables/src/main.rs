fn main() {
    another_function(5, 6);
    let x = take_five();
    let p = plus_one(x);
    println!("The value of {} + 1 is {}", x, p);
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn take_five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

