// A blog post starts as an empty draft.
// When the draft is done, a review of the post is requested.
// When the post is approved, it gets published.
// Only published blog posts return content to print, so unapproved posts can’t
//   accidentally be published.

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
}

// Defines behavior shared by different post states.
trait State {}

// The Draft Object has the State trait.
struct Draft {}

impl State for Draft {}
