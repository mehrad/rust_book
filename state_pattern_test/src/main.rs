use state_pattern_test::Post;
use std::panic::{catch_unwind, AssertUnwindSafe};

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    //expect rejecting approved post to panic
    let result = catch_unwind(AssertUnwindSafe(|| {
        post.reject();
    }));
    assert!(result.is_err());

    //test reject method and panic 
    let mut post_2 = Post::new();
    post_2.add_text("I ate a salad for lunch today");
    post_2.request_review();
    post_2.reject();
    assert_eq!("", post_2.content());
    println!("All tests passed!");

}


//Require two calls to approve before the state can be changed to Published.
//https://users.rust-lang.org/t/solving-an-an-exercise-from-the-chapter-17-of-trpl/69679
//Allow users to add text content only when a post is in the Draft state. Hint: have the state object responsible for what might change about the content but not responsible for modifying the Post.
//