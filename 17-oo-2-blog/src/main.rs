use blog::Post;

// A blog post starts as an empty draft.
// When the draft is done, a review of the post is requested.
// When the post is approved, it gets published.
// Only published blog posts return content to print, so unapproved posts can’t
//   accidentally be published.
fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
