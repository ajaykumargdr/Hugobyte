fn main() {

    // throws error (stm doesn't return)
    // let x = (let y = 10);

    // let mut x = 10;
    // let mut y = 20;
    // x = y = 30; // not possible

///////////////////

// multiline epression

    let y = {
        let x = 3;
        // println!("{y}"); // y is not created 
        x + 1   // token (in last line) 

        // else it returns ()-unit
    }; 

    println!("{y}");
    
    let mut x = 10;
     
}
