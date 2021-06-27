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
    fn summarize(&self) -> String {
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
    fn summarize(&self) -> String {
        format!("from   : {}\nmessage: \"{}\"", self.username, self.content)
    }
}
