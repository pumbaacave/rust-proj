use state_machine::blog::Post;
fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.get_content());

    post.request_review();
    assert_eq!("", post.get_content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.get_content());
    
    post.take_back();
    assert_eq!("", post.get_content());
}
