use rand::*;

pub fn print_hello(){
    println!("Hello from lib_pack");

    println!("Generated rand number:-");
    let x = thread_rng().gen_range(0..=2);
    print!("{x}");
}