fn main() {
    
// Integer (default i32)

    let x:i32 = 43_000;
    let x = 0xff; // HexaDecimal
    println!("The value of x is: {x}");

    // ascii value
    let y:u8 = b'A'; 
    println!("The value of y is: {y}");

    // Type Suffix 
    let _z:u32 = 58u32;
    
    // let a:u8 = 257;
    // println!("The value of a is: {a}"); 

// Floating-Point (default f64)

    let x:f32 = -2.0; 

    let x = x/0.0;
    println!("x is: {x}");  // -inf

// Boolean (1byte)
    
    let bvar:bool = true;   // can't store 1  
    println!("boolean variable: {bvar}");
    // boolean addition

// Character (4bytes)

    let c:char = 'z';
    println!("character variable: {c}");

    let c:char = '😻';
    println!("Heart eyed cat: {c}"); // [?]

}
