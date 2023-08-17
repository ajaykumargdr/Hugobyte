// BlogPost state trait
trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;

    // ? why not -> String
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

// Draft State
struct Draft {}

// Implementing State trait methods
impl State for Draft {

    // ? why not ref (&self)
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

// Review Pending State
struct PendingReview {}

// Implementing State trait methods
impl State for PendingReview {

    // Does nothing
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

// Blog Post Published State
struct Published {}

impl State for Published {
    
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

// #######################################################
// Post Structure
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

// Methods of Post Structure
impl Post {
    pub fn new() -> Self {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str { // self is Post ref
        self.state.as_ref().unwrap().content(self) 
    }

    pub fn request_review(&mut self) {

        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }

    }

    pub fn approve(&mut self){
        if let Some(s) = self.state.take(){
            self.state = Some(s.approve())
        }
    }


}

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    assert_eq!("", post.content());

    post.request_review();

    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
