fn main() {
    // let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3]; // the vec! macro creates a vector

    // Values can be added to mutable vectors with push
    let mut v = Vec::new();
    v.push(4);
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("the Vector 'v' is {:?}", v);

    // Accessors --------------------------------------------------------------
    // This will panic if the index is too high
    let third: &i32 = &v[2];
    println!("the third element of 'v' is {}", third);

    // This allows indices to fall outside of the range
    let v_index = 69;
    match v.get(v_index) { // v.get() gives us Option<&v>
        Some(_) => println!("Reachable element at index: {}", v_index),
        None    => println!("Unreachable element at index: {}", v_index)
    }

    /* NOTE: It is illegal to make an immutable borrow and then push to
     *       a mutable vector.
     *
     * -----------------------
     *
     * let mut v = vec![1, 2, 3];
     *
     * let first = &v[0];
     *
     * v.push(4); 
     *
     * println!("first element is {}", first); // PANIC!
     */

    // Iteration ---------------------------------------------------------------
    let v = vec![100, 32, 69];
    for i in &v {
        println!("{}", i);
    }
    // for mutable vectors
    let mut v = vec![18, -40, 369];
    println!("before: {:?}", v);
    for i in &mut v {
        *i += 51;
    }
    println!("after:  {:?}", v);

    // Using enums to store multiple values ------------------------------------
    // 
    // Since enums are all of the same type, we can use them in a vector to 
    // store multiple values.
    #[derive(Debug)]
    enum SC { // Spreadsheet Cell
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SC::Int(6911),
        SC::Text(String::from("the inside")),
        SC::Float(2.10),
    ];
    println!("the value of row is {:?}", row);
    
    // Attempt at a sort of data frame representation.
    let ID = vec![SC::Int(69), SC::Int(11), SC::Int(29),];
    let desc = vec![SC::Text(String::from("a")), SC::Text(String::from("b")), SC::Text(String::from("c")),];
    let val = vec![SC::Float(1.1), SC::Float(2.2), SC::Float(3.3),];
    
    let dat = vec![ID, desc, val];

    println!("The data is {:?}", dat);
}
