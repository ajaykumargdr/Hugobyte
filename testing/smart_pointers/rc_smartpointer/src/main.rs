#![allow(unused)]

use std::{rc::Rc, borrow::BorrowMut, cell::RefCell};

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

/*///////////////////////////////////////////////////////////////////////
/// Box 
  
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

    // println!("{:#?}", b);

/ *////////////////////////////////////////////////////////////////////////
/// Rc

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
    
    
    let x = Rc::clone(&a);

    
    // cannot assign to data in an `Rc`
    // trait `DerefMut` is required to modify through
    // a dereference, but it is not implemented for `Rc<Node2>`

    // *x = Node2::Cons(123, Rc::new(Node2::Nil));

    // println!("{:#?}", x.0);


    // println!("{}", Rc::strong_count(&a));   // counting the reference
    
    // println!("{:#?}", b);
    // println!("{:#?}", c);

/* ///////////////////////////////////////////////////////////////////////
/// RefCell

    let mut a:RefCell<i32> = RefCell::new(10); 
    *a.borrow_mut() = 50;

    println!("{:?}", a);

    let b = &a;
    let c = &a;
    
    println!("{:?} {:?}", c, b);

    *a.borrow_mut() = 25;

    println!("{:?} {:?}", c, b)

*/
///////////////////////////////////////////////////////////////////////

}


// Conclution:-
// ---------------
// * to have Node{ val, Node_another } we should use Box

// * but if you use Box, you cannot share it's reference to multiple vars
// * (i.e) Node1{val, Node_another}, 
//         Node2{val, &Node1}, 
//         Node3{val, &Node1},

// * so we have to use Rc smartpointer
//   but if you use Rc, you cannot mutate the value within it

// so we use ReffCell