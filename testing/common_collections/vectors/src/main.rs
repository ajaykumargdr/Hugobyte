/*
fn main() {
    
// vector
// way 1
    let mut vec: Vec<i32> = Vec::new();
    
    vec.push(5);
    vec.push(4);
    vec.push(3);

    // return Option<T>
    // vec.pop();   

// way 2

    let mut vec = vec!['a', 'z', 'f', 'b' ];
    println!("{:?}", vec);

    // get only the reference not the copy
    let c:&char = &vec[2];  
    println!("{c}");

    // get return Option<&T> (ref only)
    println!("{:?}", vec.get(100));

    let val:&char = vec.get(30).unwrap_or(&'-');
    println!("{val}");


// Iterating on Vectors
    println!("\nIterating through vectors:-");

    for i in &mut vec{  // &vec is enough
        println!("{i}");

        // changing values
        *i = 'X' ;  
    }

    println!("{:?}", vec);

// List of diff type values with enum

#[derive(Debug)]
enum SpreadSheetCall{
    IntVal(i32),
    FloatVal(f64),
    Text(String)
}

    let mut row:Vec<SpreadSheetCall> = Vec::new(); 

    row.push(SpreadSheetCall::IntVal(10));
    row.push(SpreadSheetCall::FloatVal(10.00));
    row.push(SpreadSheetCall::Text("10".to_string()));

    println!("{:?}", row);

    println!("#############################################\n\n");

    vec_and_iters();

}


fn vec_and_iters(){

    let v = [2, 4, 6, 8, 10];

// iter().mapping

   let v2:Vec<i32> = v.iter().map( |element|   
        element*2
    ).collect();

    println!("{:?}", v2);   // [4, 8, 12, 16, 20]
    println!("{:?}", v);    // [2, 4, 6, 8, 10]

// creating vector ///////////////////////////////////

    // iterating a range

    let v: Vec<i32> = 
        (1..)
        .filter(|x| x % 3 == 0)
        .take(5)
        .collect();
    
    println!("{:?}", v);
 
}

*/

// move_semantics2.rs
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.

// Expected output:
// vec0 has length 3 content `[22, 44, 66]`
// vec1 has length 4 content `[22, 44, 66, 88]`

fn main() {
    let mut vec0 = Vec::new();

    // Do not move the following line!
    let mut vec1 = fill_vec(&mut vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: &mut Vec<i32>) -> Vec<i32> {
    // let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec.clone()

}
