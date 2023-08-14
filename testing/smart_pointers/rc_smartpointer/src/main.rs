#![allow(unused)]

use std::rc::Rc;

#[derive(Debug)]
enum Node{
    Cons(i32, Box<Node>),
    Nil,
}

#[derive(Debug)]
enum Node2{
    Cons(i32, Rc<Node2>),
    Nil,
}

fn main() {
/*     
    let a = Node::Cons(
        5,
        Box::new(
            Node::Cons(
                10, 
                Box::new(Node::Nil))
        ) 
    );

    // Using Box you cannot use 2 reference for a 
    // single object

    let b = Node::Cons(3, Box::new(a));
    // let c = Node::Cons(4, Box::new(a));      // a is moved

    // also we cannot pass reference

    println!("{:#?}", b);

*////////////////////////////////////////////////////////////////////////

    let mut a =Rc::new( 
        Node2::Cons(
            5,
            Rc::new(Node2::Cons(
                    10,
                    Rc::new(Node2::Nil)
                    )
                )
        )
    );

    println!("{}", Rc::strong_count(&a));   // counting the reference

    let b = Node2::Cons(
        3,
        Rc::clone(&a)
    );

    
    println!("{}", Rc::strong_count(&a));   // counting the reference


    let c = Node2::Cons(
        4,
        Rc::clone(&a)
    );

    // *a = Node2::Cons(500, Rc::new(Node2::Nil));

    println!("{}", Rc::strong_count(&a));   // counting the reference

    println!("{:#?}", b);

    println!("{:#?}", c);
    // println!();

////////////////////////////////////////////////////////////////////////


}
