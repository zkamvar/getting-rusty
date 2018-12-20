fn main() {
    // Example 1: this code is valid, but leaves us with the index for
    // the first word in 's' (5), without the original word.
    let s = String::from("hello world!");
    let word = first_word_bytes(&s);
    s.clear();
    
    // Slices can take care of this
    let s = String::from("hello world!");

    let hello = &s[0..5];
    let world = &s[6..11];
}

/* 
 * Return the index of the end of the first word.
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


