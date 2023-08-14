use super::*;

#[test]
fn add(){
    assert_eq!( calculator::add(2,2), 4 );
    assert_eq!( calculator::add(2.0,4.0), 6.0 );
    assert_ne!( calculator::add(5, 5), 0); 
    assert_ne!( calculator::add(5.0, 5.0), 0.0); 
}

#[test]
fn sub(){
    assert_eq!( calculator::sub(5, 1), 4);
    assert_eq!( calculator::sub(5.0, 1.0), 4.0);
    assert_ne!( calculator::sub(10, 10), 20);
    assert_ne!( calculator::sub(10.0, 10.0), 20.0);
}

#[test]
fn mul(){
    assert_eq!(calculator::mul(5, 5), 25);
    assert_eq!(calculator::mul(5.0, 5.0), 25.0);

    assert_ne!(calculator::mul(5, 10), 15);
    assert_ne!(calculator::mul(5.0, 10.0), 15.0);
}

#[test]
#[should_panic= "by zero"]
fn div(){

    assert_eq!(calculator::div(10, 2), 5);
    assert_eq!(calculator::div(10.0, 2.0), 5.0);

    assert_ne!(calculator::div(10, 2), 8);
    assert_ne!(calculator::div(10.0, 2.0), 12.0);
    
    // zero division error test case
    calculator::div(10, 0);
    calculator::div(10.0, 0.0);

}
