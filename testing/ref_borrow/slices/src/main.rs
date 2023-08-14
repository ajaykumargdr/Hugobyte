fn main() {

    let s = String::from("Hello world");
    
    // let hello = &s[0..5];
    // let world = &s[7..6];
    
    // println!("{hello} {world}");


    let mut s = String::from("hello");
    let word = &s[1..4];

    // s.clear();

    println!("{word}");



    let mut s = String::from("Hello world");
    
    let m = &mut s;
    let im = &s;

    println!("{m} {im}");

}
