// option_enum 
    // Option enum (don't need give ecplicit type)
    // but we can pass only one paras

    // generic type parameter

#[derive(Debug)]
enum Option<T>{
    None,
    Some(T),
    another(T)  // not needed | nothing in it
    // bcz if we want some more other data to be 
    // stored within a generic type var we can declare
    // another Option enum
}

fn main() {

// Standard
    let op1 = Option::<i32>::None;  
    println!("{:?} ", op1);

    let op2 = Option::<i32>::Some(54);
    println!("{:?}", op2);

// Option enum (don't need-give ecplicit type)

    // let opt1 = None; // not works with
    let op2 = Some(5); // works

    // println!("{:?}", op1);
    println!("{:?}", op2);

    // let op3 = another(bool); // not works
    
}
