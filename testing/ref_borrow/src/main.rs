fn main() {
    
    let s1 = String::from("Hello");

    let len = calculate_length(&s1);

    println!("The length of the {s1} is {len}");
}

fn calculate_length(s : &String) -> usize{
    
    // not work (ref)
    // s.push_str(", world");

    s.len()
}
