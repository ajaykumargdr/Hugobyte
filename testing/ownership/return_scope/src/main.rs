fn main() {
    let s1 = gives_ownership();

    println!("I got {}", s1);

    let s2 = String::from("hello");

    let s3 = takes_and_returns(s2);

    println!("I got my string \"{}\" back", s3);
}

fn gives_ownership() -> String{
    String::from("new string")
}

fn takes_and_returns(str_val: String) -> String{
    println!("given string \"{}\"", str_val);
    
    // return ownership
    str_val
}
