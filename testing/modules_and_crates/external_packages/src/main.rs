
use rand::*;

use std::collections::HashMap;

fn main() {

    // random number 

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("{}", secret_number);

    let mut map = HashMap::new();
    map.insert(4, 1);
    println!("{:?}", map);

}
