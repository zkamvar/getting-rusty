// A blog post starts as an empty draft.
// When the draft is done, a review of the post is requested.
// When the post is approved, it gets published.
// Only published blog posts return content to print, so unapproved posts can’t
//   accidentally be published.

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
    approved: bool,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
            approved: false,
        }
    }
    pub fn add_text(&mut self, text: &str) {
        // Does not depend on state of the post; not part of the state pattern
        self.content.push_str(text);
    }
    pub fn content(&self) -> &str {
        self.state //    Option<Box<dyn State>>
            .as_ref() // Option<&Box<dyn State>>
            .unwrap() // &Box<dyn State>
            .content(self) // State (because of deref coercion)
    }
    pub fn request_review(&mut self) {
        // NOTE: if let Some() always ensures that we have a Some value
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            // require two approvals
            if self.approved {
                self.state = Some(s.approve())
            } else {
                self.approved = true;
                self.state = Some(s.request_review())
            }
        }
    }
    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }
}

// Defines behavior shared by different post states.
trait State {
    // NOTE: the signature is self: Box<Self> so that the state value of the
    //       Post can be transformed (because the ownership of box is passed on)
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        // default is empty string slice because we do not need it for
        // Draft or PendingReview.
        // The lifetimes here ensure that the content we create here has
        // the same lifetime as the Post.
        ""
    }
}

// Objects implementing the State trait ========================================
struct Draft {}
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

struct Published {}
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
