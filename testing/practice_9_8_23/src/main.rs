
fn main() {
    
    let mut a = 5;

    let r1 = &mut a;
    *r1 = 10;
    
    let r2 = &mut a; 
    *r2 = 20;

    // uncommenting this will make the creation of second mut ref (r2) error
    // because these 2 mutable references are overlapping
    // *r1 = 10; 

    println!("{a}");

////////////////// Hashmap Get

use std::collections::HashMap;

    let mut mp = HashMap::new();

    mp.insert("key1", 1);
    mp.insert("key2", 2);

    println!("{:?}", mp.get("key2"));


}
