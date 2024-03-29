use blog::Post;

fn main() {
    lib();
    // state_lib();
}

fn lib() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();
    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}

// fn state_lib() {
//     // State 0, Create new post which starts on draft
//     let mut post = Post::new();

//     // State 1, Draft contains text, which is not yet public
//     post.add_text("I ate a salad for lunch today");
//     assert_eq!("", post.content());
    
//     // State 2, Review which concludes if the post is approved
//     post.request_review();
//     assert_eq!("", post.content());
    
//     // State 3, final state which publishes the post
//     post.approve();
//     assert_eq!("I ate a salad for lunch today", post.content());
// }