use std::fmt::Display;

pub fn largest<T: PartialOrd>(list: &Vec<T> ) -> &T{

    let mut largest = &list[0];

    for i in list{
        if i > largest{     // ****###########
            largest = i;
        }
    }

    largest
}

pub fn sum<T:std::ops::Add<Output = T > + Clone>(list: &Vec<T>) -> T{

    let mut sum = list[0].clone() ;

    for i in &list[1..]{
        sum = sum + i.clone();
    }

    sum
}


#[derive(Debug)]
struct Point<T>{
    x: T,
    y: T
}

#[derive(Debug, Clone, Copy)]
struct PointMixed<T, U>{
    x: T,
    y: U
}

impl<U, T> PointMixed<T, U>{

    // here 'Self' means PointMixed<T, U> // not 'self'
    fn new(x: T, y: U) -> Self{
        Self{ 
            x: x,
            y: y 
        }
    }

    fn x(&self) -> &T{
        &self.x
    }  

    fn y(&self) -> &U{
        &self.y
    }

}

fn main() {

// Generic Function
    let list = vec![ 4, 30, 83, 34, 0];
    println!("{}", largest(&list)); 
    println!("{}", sum(&list));

/*
    let point_in_int = Point{x:50, y:65};
    let point_in_float = Point{x: 50.00, y:65.00};

// Generic Struct
    println!("{:?} {:?}", point_in_int, point_in_float);

// Multi Type Generic Struct

    let point_in_mixed = PointMixed{x: "And", y:true };

    println!("{:?}",point_in_mixed);

    println!("{}", point_in_mixed.x()); // And
    println!("{}", point_in_mixed.y()); // true
*/
// Mixup function



    let point_fi = PointMixed{ x:5.00, y:2}; 
    let point_if = PointMixed{x:5, y:2.00};

    
    // let point_ff = point_fi.mixup(&point_if);
    // let point_ii = point_if.mixup(&point_fi); 


    println!("{:?}", point_fi);
    println!("{:?}", point_if);
    // println!("{:?}", point_ff);
    // println!("{:?}", point_ii);






















/*/ Creating new instance

    println!("\n\nNew Instance of values 5 and 5.00");

    let pm_new = PointMixed::new(5, 5.00);
    
    println!("{:?}", pm_new);

*/

}
