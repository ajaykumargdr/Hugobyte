#[derive(Debug)]
enum List {
    Cons(i32, Rc<RefCell<List>>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // let value = 5;

    let a = RefCell::new(Cons(5, Rc::new(RefCell::new(Nil))));
    let a = Rc::new(a);

    let b = Cons(3, Rc::clone(&a) );


    let c = Cons(4, Rc::clone(&a));

    // *value.borrow_mut() += 100;

    println!("a before = {:#?}", a);
    println!("b before = {:#?}", b);
    println!("c before = {:#?}", c);

    *a.borrow_mut() = Cons(876, Rc::new(RefCell::new(Nil)));

    println!("a after = {:#?}", a);
    println!("b after = {:#?}", b);
    println!("c after = {:#?}", c);
}