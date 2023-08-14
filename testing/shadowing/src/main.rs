fn main() {

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x is: {x}");
    }

    println!("The value of x is: {x}");


    // mut and shadowing
    
    let mut spaces = "   ";

    // will throw error (mismatched types)
    spaces = spaces.len();  

    let spaces = "    ";
    println!("The spaces is: '{spaces}'");
    let spaces = spaces.len();
    println!("The number of spaces is: {spaces}");




}
