fn main() {
    
    let mut s = String::from("hello");
    
    let ref_s= &mut s;
       
    // can be used (async)
    ref_s.push_str("changed");
    s = String::from("another");
    
    // why cannot use both ?
    // cannot use both mut and immut in same time
    // also 2 mut's canot used at same time 
//    println!("Og String {} {}", ref_s, s);


///////////////////////

    let mut s2 = String::from("Hello");

    let r1 = &s2;
    let r2 = &s2;

    println!("{} & {}", r1, r2);

    let r3 = r1;
    
    println!("{} & {} & {}", r1, r2, r3);



}



// variables r1 and r2 will not be used after this point