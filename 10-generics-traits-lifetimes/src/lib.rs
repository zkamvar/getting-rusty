// Traits ---------------------------------------------------------------------
// These are a way to group method signatures together to define a set of
// behaviors necessary to accomplish the same purpose.

// Example two different structs that hold text:
//  - NewsArticle: long-form text with metadata about location
//  - Tweet: maximum 280 chars, metadata new tweet, retweet, reply
//
// A Summary trait
//
// NOTE: if this ends with a semicolon instead of a definition, then there is
// no default, and each type needs to define its own method (a lot like S3).
pub trait Summary {
    fn bummerize(&self) -> bool;
    fn summarize(&self) -> String {
        // Default implementation
        String::from("(Read on...)")
    }
    // Other method signatures go here
}

// Implementing a trait on a type
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// impl Summary for NewsArticle {}
impl Summary for NewsArticle {
    fn bummerize(&self) -> bool {
        false
    }
    fn summarize(&self) -> String {
        format!("{}, by {}, ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn bummerize(&self) -> bool {
        true
    }
    fn summarize(&self) -> String {
        format!("from   : {}\nmessage: \"{}\"", self.username, self.content)
    }
}

// Traits as parameters (in functions) ----------------------------------------
//
// This allows us to create more generic functions that can call methods on
// any type that implements the Summary trait. This means that we can write
// functions specific for our types, but these functions will cease to work
// if String or i32 types are passed.

// impl Trait syntax ~~~
// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

// Trait bound syntax ~~~
//
// The `impl Trait` syntax is sugar for the trait bound syntax. So the above
// code is equivalent to:
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Benefits of Trait bound syntax ~~~
//
// The trait bound syntax is useful for more complex situations where you would
// have more than one argument in your function:
//
//      pub fun notify(item1: &impl Summary, item2: &impl Summary)
// vs
//      pub fun notify<T: Summary>(item1: &T, item2: &T)

// Multiple Traits ~~~
//
// We can use the `+` syntax to allow for multiple traits in a single parameter
//
//      pub fn notify(item: &(impl Summary + Display))
// vs
//      pub fn notify<T: Summary + Display>(item: &T)
//

// Clearer Trait Bounds with `where` Clauses ~~~
//
// The Trait bound syntax really shines when you need to have multiple possible
// traits for individual parameters to avoid your functions from becoming
// overburdened in the definition:
//
//      fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u &U) -> i32 {
//          // Function Body
//      }
// is equivalent to
//      fn some_function<T, U>(t: &T, u: &U) -> i32
//          where T: Display + Clone,
//                U: Clone + Debug
//      {
//          // Function Body
//      }

// Returning Types that Implement Traits --------------------------------------
//
// If we want to return a type of a trait, we can do that by declaring `impl Trait`
// in the return type:
//
// NOTE: you can only return ONE TYPE per Trait. This means, that we need to
// define different functions for different Types.
fn returns_summarizable_tweet() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
