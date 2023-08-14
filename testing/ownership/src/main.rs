fn main() {
    
    let mut x = 5;
    let mut y = x;

    println!("{x} {y}"); // 5 5 
    x = 20;
    println!("{x} {y}"); // 20 5



    let mut s1 = String::from("hello");

    // without clone method the s2 goes out of scope
    let s2 = s1.clone();

    s1 = "string is changed".to_string();

    println!("{} {}", s1, s2);

}
