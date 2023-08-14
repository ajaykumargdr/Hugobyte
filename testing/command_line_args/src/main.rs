use std::env;
use std::fs;

use std::process;

fn main() {
    
    let args: Vec<String> =  env::args().collect();
    
    dbg!(env::args().next() );
    dbg!(&args);

    let poem = fs::read_to_string(&args[1]).expect("### Enter Valid file name ###");

    process::exit(1);

    println!("{poem}");


}
