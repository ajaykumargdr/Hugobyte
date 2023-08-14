use std::env;
use std::fs;

struct Point<T> {
    x: T,
    y: T,
}

impl<T: std::ops::Add<Output = T> + Clone > Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    // fn distance_from_origin(&self) -> T {
    //     self.x.clone() + self.y.clone()
    // }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        let s = format!("{}, by {} ({})", self.headline, self.author, self.location);
        println!("{}", self.headline);
        s
    }
}

fn main() {

/*//////////////////////////////////////////////////////////////
    let args:Vec<String> = env::args().collect();

    let file_name = &args[1];
    let word_to_find = &args[2];

    let file_content = 
    fs::read_to_string(file_name).expect("File not found");

    for line in file_content.lines(){
        if line.contains(word_to_find){
            println!("{line}");
        }
    }

 *//////////////////////////////////////////////////////////////  
    let n1 = NewsArticle{
        headline: String::from("Headline"),
        location: String::from("Location"),
        author: String::from("Author"),
        content: String::from("Content")
    };

    println!("{}", n1.summarize());
    println!("{}", n1.summarize());
    

}
