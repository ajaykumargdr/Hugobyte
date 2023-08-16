use std::rc::Rc;

#[derive(Debug)]
struct S1{
    val1: i32
}

impl S1{

    fn get_val1(&self)-> i32{ self.val1 }

    fn change_val(&mut self) { self.val1 += 1; }
}

fn _check_rc_mutate(){

    let x = Rc::new(S1{
        val1: 5
    });


    // not possible
    
    x.get_val1();   // can be done
    
    // x.val1 = 10; 
    // x.change_val(); // cannot be done 
                    
    println!("{:?}", x.val1); // can be done
    println!("{:#?}", x); 

/*
    * Conclusion:-
        * a value within the cannot mutate
        * but it allows access of the elements within the rc
*/

}

/////////////////////////////

#[derive(Debug)]
enum E1{
    Tuple(i32, i32, i32)    
}

fn main(){

    _check_rc_mutate();

    let x = Rc::new(S1{
        val1: 5
    });

    let y = Rc::new(
        E1::Tuple(5, 10, 15)
    );

    
    println!("{:?} {:?}",
        x.val1, 
        y //.0          // uncommenting this gives error
    );


}
