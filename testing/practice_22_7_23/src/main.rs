
fn main() {
    
    let mut s = String::from("this is t he string");

    let x:Vec<&str> = s.split("th").collect();
    println!("{:?}", x);

    if x[0]!= s{
        println!("The string contains the string");
    }

}

