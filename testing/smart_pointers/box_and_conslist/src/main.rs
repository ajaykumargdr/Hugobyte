#[derive(Debug)]
enum List{
    // Cons(i32, List),    // not work (size maybe infinite)

    Cons(i32, Box<List>),   // works (cz box can grow)
    Nil
}

fn main() {
    
// Box (creates data on heap storage)

    let b = Box::new(5);
    println!("b = {}", b);

// ##################################################

// Cons list    (1, (2, (3, Nil)))

    // look at 'enum List'

    use List::{Cons,Nil};
    
    let cons_list = List::Cons(1, Box::new(   List::Cons(2, Box::new(Nil))  ));
    
    // let cons_list2 = Cons(21, Box::new( Cons(4, Box::new(Cons(6, Box::new(Nil)) ) ) ) );

    let cons_list2 = Cons(21, Box::new( Cons(4, Box::new(Cons(6, Box::new(Nil)) ) ) ) );
    
    // let cons_list3 = Cons(22, Box::new( Cons(5, Box::new) ) );
    
    println!("{:#?}", cons_list);
    println!("{:#?}", cons_list2);

}
