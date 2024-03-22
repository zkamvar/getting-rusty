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
    pub fn add_text(&mut self, text: &str) {
        // Does not depend on state of the post; not part of the state pattern
        self.content.push_str(text);
    }
    pub fn content(&self) -> &str {
        ""
    }
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
}

// Defines behavior shared by different post states.
trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
}

// Objects implementing the State trait
struct Draft {}
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
}

struct PendingReview {}
impl State for PendingReview {}

struct Published {}
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
