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

    // Iteration --------------------------------------------------------------
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
}
