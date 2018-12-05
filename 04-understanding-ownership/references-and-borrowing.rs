fn main() {
    let s1 = String::from("hello");

    // Without references, the original value must be passed back if we wanted
    // to use it ever again.
    let (s2, len1) = calculate_length_nr(s1);
    // With references, we need only retun value we care about
    let len2       = calculate_length(&s2);
    println!("The length of '{}' is {} (yes, {}).", s2, len1, len2);

    // Mutable references
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);
    {
        // Placing this mutable reference in brackets gives it a new scop
        let r1 = &mut s;
        println!("r1 = {}", r1);
    } // r1 is invalid after these brackets

    let r2 = &mut s;
    println!("r2 = {}", r2);
}

fn calculate_length_nr(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn calculate_length(s: &String) -> usize {
    s.len() // remember: no semicolon here because it is returned
}

// A function to mutate a string
fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}
