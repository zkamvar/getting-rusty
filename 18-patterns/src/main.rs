// Patterns consist of some combination of
//
// - Literals
// - Destructured arrays, enums, structs, or tuples
// - Variables
// - Wildcards
// - Placeholders

// Matching using Option<T> ----------------------------------------------------
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, // This part is necessary because matching is exhaustive
        Some(i) => Some(i + 1),
    }
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
    ColorChange(Color),
}

#[derive(Debug)]
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

fn main() {
    println!("## All the Places Patterns Can Be Used\n");

    // ------------------------------------------------
    println!("---| `match` Arms");
    let five = Some(5);
    println!("Five plus one is {:?}", plus_one(five));
    println!("None plus one is {:?}", plus_one(None));
    println!();

    // ------------------------------------------------
    println!("---| Conditional `if let` Expressions");
    let favourite_colour: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(colour) = favourite_colour {
        println!("[if let] Using your favourite colour, {colour}, as the background");
    } else if is_tuesday {
        println!("[else if] Tuesday is green day!");
    } else if let Ok(age) = age {
        // The `age` variable is only in scope starting here.
        // This is because `Ok(age)` is the _shadowed_ `age` variable.
        // Not possible to do `let Ok(age) = age && age > 30`
        if age > 30 {
            println!("[else if let Ok()] Using purple as the background colour");
        } else {
            println!("[else if let Ok()] Using orange as the background colour");
        }
    } else {
        // this shows that it is not exhaustive.
        println!("[else] Using blue as the background colour");
    }
    println!();

    // ------------------------------------------------
    println!("---| `while let` Condidtional Loops");
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    println!("pop the top offfa {:?}", stack);
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
    println!();

    // ------------------------------------------------
    println!("---| `for` Loops");

    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
    println!();

    // ------------------------------------------------
    println!("---| `let` statements");
    println!("`let x = 5;`, the variable name is a simple PATTERN.");

    println!("in tuples, the number of elements in PATTERN are checked with the EXPRESSION.");
    let (_x, _y, _z) = (3, 4, 5);
    println!();

    // ------------------------------------------------
    println!("---| function parameters");

    let point = (3, 5);
    print_coordinates(&point);
    println!();

    // ================================================
    println!("## Refutability: Weather a Pattern Might Fail to Match\n");

    println!("`let x` statements are irrefutable because they can match anything");
    println!(
        "`let (x, y)` statements are refutable because these will not match anything but a
    two-element tuple"
    );
    println!();

    // ================================================
    println!("## Pattern Syntax\n");

    println!("|--- Matching Literals");
    let x = 1;
    match x {
        1 => println!("one!"),
        2 => println!("two!"),
        3 => println!("three!"),
        _ => println!("anything!"),
    }
    println!();

    // ------------------------------------------------
    println!("---| Matching Named Variables");

    let x = Some(5);
    let y = "TEN";

    match x {
        Some(50) => println!("Got 50"),
        // This is NOT the same y as above!
        // This is matching ANY value inside a Some()
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);
    println!();

    // ------------------------------------------------
    println!("---| Multiple Patterns");
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    println!();

    // ------------------------------------------------
    println!("---| Matching Ranges of values with `..=`");
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='z' => println!("lowercase ASCII"),
        'A'..='Z' => println!("uppercase ASCII"),
        _ => println!("you got punct"),
    }
    println!();

    // ------------------------------------------------
    println!("---| Destructuring to Break Apart Values");

    println!("   |___| ... Structs");
    let p = Point { x: 0, y: 7 };

    // One way, with confusing names
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    // More succinct
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x: 0, y: 0 } => println!("At the origin"),
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }
    println!("   |___| ... Enums");
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Changing color to {r}, {g}, {b}");
        }
        Message::ColorChange(placeholder) => {
            println!("Meh: {:?}", placeholder);
        }
    }
    println!("   |___| ... Nested Structs and Enums");
    let msg = Message::ColorChange(Color::Hsv(0, 160, 255));

    match msg {
        Message::ColorChange(Color::Rgb(r, g, b)) => {
            println!("Change color to RGB {r}, {g}, {b}");
        }
        Message::ColorChange(Color::Hsv(h, s, v)) => {
            println!("Change color to HSV {h}, {s}, {v}");
        }
        _ => (),
    }
    println!("   |___| ... Structs and Tuples");

    let ((_feet, _inches), Point { x: _x, y: _y }) = ((3, 10), Point { x: 3, y: -10 });
    println!();

    // ------------------------------------------------
    println!("---| Ignoring Values in a Pattern");
    println!("   |___| ... Entire values with _");
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
        println!(
            "Useful when implementing a trait with a certain type signature, but you do not
            need a particular parameter."
        )
    }
    foo(3, 4);
    println!("   |___| ... Parts of a Value with a Nested _");

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}");
        }
    }
    println!("   |___| ... Unused Variable by Starting Its Name with _");
    // this variable IS still bound and behaves according to the borrowing rules
    let _x = 5;

    println!("   |___| ... Remaining Parts of a Value with ..");

    struct Point3D {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point3D { x: 0, y: 0, z: 0 };
    match origin {
        Point3D { x, .. } => println!("x is {}", x),
    }

    match numbers {
        (first, .., last) => {
            println!("A range: ({first}, {last})");
        }
    }
    println!();

    // ------------------------------------------------
    println!("---| Extra Condidtionals with Match Guards");

    println!(
        "match thing {{
    Some(x) [condition on x] => {{do stuff}},
    Some(x) => {{do other stuff}},
    None => (),
}}"
    );

    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);

    let x = 4;
    let y = false;

    match x {
        // equivalent to (4 | 5 | 6) if y
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
    println!();

    // ------------------------------------------------
    println!("---| @ Bindings");
    enum Msg {
        Hello { id: i32 },
    }

    let msg = Msg::Hello { id: 5 };

    match msg {
        Msg::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Msg::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Msg::Hello { id } => println!("Found some other id: {}", id),
    }
}
