fn main() {
    // Decalring a mutable string 
    let mut s = String::from("hello");
    // Appending a string to the mutable string
    s.push_str(", world!");
    
    // Defining a variable based on another variable
    let x: u32 = 5; // bind the value 5 to `x`
    let y: u32 = x; // make a copy of the value in `x` and bind it to `y`
    println!("x = {} y = {}", x, y);

    // Strings are a bit different...
    let s1 = String::from("olleh"); // store the string on the heap and bind the
                                    //   "olleh" string to a pointer s1
    let s2 = s1;                    // *move* the pointer in s1 to s2
    println!("{}", s);
    // println!("{}", s1); // This will not work because s1 was invalidated 
    println!("{}", s2);
    let s1 = s2.clone();   // cloning creates a copy of the data on the heap
    println!("s1 = {} s2 = {}", s1, s2);
}
