#[derive(Debug)]
struct MyBox<T>(T,);         // struct MyBox(i32,);

impl<T> MyBox<T>{
    fn new(x: T) -> MyBox<T>{
        MyBox(x)
    }
}


use std::ops::Deref;    // Deref is a trait
impl<T> Deref for MyBox<T>{

    type Target = T;    // 

    fn deref(&self) -> &Self::Target {
        &self.0
    }

}


fn main() {

    let x_box = MyBox::new(5);  
    println!("{:#?}", x_box);

// #########################################

// Own Smartpointer

    let x = 5;
    let box_x = MyBox::new(x);

    // this cannot be dereferenced
    // if this is Box type it can deref
    // cz it implements Deref

    // assert_eq!(5, *box_x);

// after implementing Deref

    let x = 5;
    let box_x = MyBox::new(x);

    // now it can deref the value from the smart pointer
    assert_eq!(5, *box_x);

    // Box will deref it but user defined box will not
    // print!("{box_x}");

// #### NOTE ####
// this Deref returns the value with the struct (box)

//  i) if it is a ref (MyBox(&x)) we get &x
// ii) if it is a val (MyBox(x)) we get x  


// #########################################
// Implicit Deref Coercions

    let name_box = MyBox::new(String::from("HugoByte"));
    hello(&name_box);

    // by &name we get &MyBox<String> only
    // but Deref will deref it to the String

// Example 2

    let name_box = 
    MyBox::new(String::from("Hugobyte"));


    // hello( (&name_box)[..] );
    println!("{}", &(*name_box)[..] );

    // let s = String::from("text is this");
    // println!("{}", &s[0..5] );

}


// normal function
fn hello(name : &str){
    println!("Hello, {name}!");
}