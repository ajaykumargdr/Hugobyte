use std::fmt;

// stucts that implementing this should have to
// implemented fmt::Display trait
trait OutlinePrint: fmt::Display {

    fn outline_print(&self) {

        let output = self.to_string();
        let len = output.len();
        
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}


fn main() {
    let p = Point{ x: 500, y: 1000};
    p.outline_print();

////////// 
    
    let w = Wrapper(vec![
            String::from("element1"),
            String::from("element2")
        ]);
    
// w implements fmt:Display so we can directly print
    println!("{w}");    

}

// fmt::Display implementation for a struct that contains 
// a Vec<String> 

struct  Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
