// structs are basically just a form of object orientation. They have the 
// same basic definitions as they do in C. They also will contain methods.
// 
// The parts of the structs must not be pointers because the data in the struct
// must be valid as long as the struct is valid. If the struct doesn't own its
// data, then this is a false assumption.
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
// Structs don't always have to have named fields. This allows us to have 
// specific classes that are similar, but very different
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main () {

    let black  = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    // Mutable instances means that you can change specific fields
    let mut user1 = build_user(String::from("ship@man.com"), String::from("boatboy"));
    
    // Here, we can change the sign in count
    user1.sign_in_count = 420;

    // Details from other structs can be used to fill in the gaps
    //
    let mut user2 = User {
        username: String::from("mintraven"),
        email: String::from("corvus@carvo.ne"),
        // user: user1.sign_in_count,
        // active: user1.active,
        // ^
        // | these are equivalent
        // v
        ..user1
    };
}

// constructor
fn build_user (email: String, username: String) -> User {
    User {
        email,   // Note: here we can use shorthand if the args are matching
        username,
        active: true,
        sign_in_count: 1,
    }
}
