fn main() {
    
    let s = String::from("hello");
    
    takes_ownership(s);

    // println!("here s is not valid {}", s);

    let x =5;

    makes_copy(50);
    println!("Here x is valid {x}");

}

fn takes_ownership(str: String){
    println!("{}", str);    
}

fn makes_copy( int_val : i32){
    println!("{}", int_val);
}
