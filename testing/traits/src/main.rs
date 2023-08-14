
use aggregator::Summary;

#[derive(Debug)]
pub struct NewsArticle{
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

#[derive(Debug)]
pub struct Tweet{
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// Summary trait Implementation for NewsArticle

impl Summary for NewsArticle{
    fn summarize(&self) -> String{
        format!("{}, by {} in {}", self.headline, self.author, self.location)
    }

    fn to_string(&self) -> String{
        format!("headline: {}, location: {}, author: {}, content: {}", self.headline, self.location, self.author, self.content)
    }
}

// Summary trait Implementation for NewsArticle

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// any type that implements the 'Summary' trait 
// can call this function

// cz if it implements trait it must implement the 
// methods also

pub fn notify(item: &impl Summary){
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_optimized_syntax<T: Summary>( item: &T){
    println!("Breaking news! {}", item.summarize());
}


// item - any type that implements these 2 traits

// pub fn notify2(item: &(impl Summary + trait2)){}
// pub fn notify3<T: Summary + trait2>(item: &(T)){}


// Returning a Trait implemented object

pub fn get_summarizable() -> impl Summary{

    Tweet{
        username: String::from("tweets username"),
        content:String::from("tweets content"),
        reply: false,
        retweet: true
    }

} 


fn main() {

    let news1 = NewsArticle{
        headline: String::from("HeadLine1"),
        location: String::from("Location1"),
        author: String::from("author1"),
        content: String::from("content1")
    }; 

    println!("{}", news1.summarize());

    let tweet1 = Tweet{
        username: String::from("username1"),
        content: String::from("Content1"),
        reply: false,
        retweet: false
    };

    println!("{}", tweet1.summarize());    

    println!("{}", news1.to_string());
    println!("{}",tweet1.to_string());


// trait as parameter

    notify(&news1);

    notify_optimized_syntax(&news1);
    
// trait as a return

    println!("Returned values (impl Summary trait) {}", get_summarizable().summarize());

}   
