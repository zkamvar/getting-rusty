fn main() {
    // Example 1: this code is valid, but leaves us with the index for
    // the first word in 's' (5), without the original word.
    let mut s = String::from("hello world!");
    let word = first_word_bytes(&s);
    s.clear();
    
    // Slices can take care of this
    let mut s = String::from("hello world!");
    let len = s.len();

    let hello = &s[0..word]; // 0..5
    let world = &s[6..len];  // 6..11
    println!("{} {}", hello, world);

    // Equivalent: 
    let hello = &s[..=4];
    let world = &s[6..];
    println!("{} {}", hello, world);

    let my_string = String::from("안녕 지구!");
    // Strings can be converted to string literals by using slices
    let word = first_word(&my_string[..]);
    println!("{}{}", word, world);

    let my_sting_literal = "안녕 지구!";
    
    // Works
    let my_string = first_word(&my_string[..]);
    println!("{}{}", word, world);

    // Also works because string literals ARE slices stored in the binary
    let my_string = first_word(my_string);
    println!("{}{}", word, world);
}

/* 
 * Return the index of the end of the first word.
 *
 * Problems:
 * 
 * 1. returns an unsigned integer instead of a &str or &String
 * 2. signature is a &String, but it would be better if it were &str,
 *    since &str is compatible with both.
 * */
fn first_word_bytes(s: &String) -> usize {
    // Convert string to array using the as_bytes() method
    let bytes = s.as_bytes();

    //                iterates ... creates tuple (n, &)
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len() // remember: no semicolon here because it is
            // being returned
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]
        }
    }
    &s[..]
}
