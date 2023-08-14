fn main() {

// Scalar data types

    let x = 5;
    let y = &x;

    assert_eq!(5, *y);

    // will not work (cz rustc will not deref the y)
    // assert_eq!(5, y);

// using Box

    let x = 5;
    let y = Box::new(x);

    println!("{x}");

    assert_eq!(5, *y);  // works(cz box impl Deref)

    // won't works (still its Box<i32> only)
    // assert_eq!(5, y);    

// some deref's

    let x = String::from("this");

    let y = Box::new(&x);

    println!("{x}");
    
    // It will deref both
    println!("{y}");
    println!("{:#?}", y);  


}
