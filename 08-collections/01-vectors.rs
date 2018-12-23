fn main() {
    // let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3]; // the vec! macro creates a vector

    // Values can be added to mutable vectors with push
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{:?}", v);
}
