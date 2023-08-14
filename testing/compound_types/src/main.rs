fn main() {

// Tuples 

    let tup:(i32, f64, u8, i32) = (22, 4.3, 68, 22);

    // Pattern Matching Destructure
    let (x, y, z, a) = tup;
    println!("tuple values: {x} {y} {z} {a}");

    // Use directly (!should use format string)
    println!("tuple values: {} {}", tup.0, tup.1);

    // 'unit' values (empty)
    let tup2 = ();

// Arrays (types must be same)

   let arr:[u32;5] = [1, 3, 4, 8, 6];

   let mut tup:(f32, i32, u64) = (34.54, -65, 872);

   tup.1 = 10;

   println!(" {}", tup.1);


    



}


