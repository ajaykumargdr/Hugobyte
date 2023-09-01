// declarative macros
////////////////////////////////////////////////////

#[macro_export]
macro_rules! vec_ { // macro name
    
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();

            // 
            $(
                // println!("{}", $x);
                temp_vec.push($x);
            )*

            temp_vec
        }
    };
}
////////////////////////////////////////////////////

// macro name 
macro_rules! add{

    // match arm
    ($a:expr, $b:expr) => {

        // this will be replaced with the values
        {
            $a + $b
        }

    }

}

////////////////////////////////////////////////////
macro_rules! add2 {

    // first arm
    ($a:expr, $b:expr) => {
        {
            $a + $b
        }
    };

    // second arm
    ($a:expr)=>{
        {
            $a + 1
        }
    }

}

////////////////////////////////////////////////////
macro_rules! add_as {

    // 2expr and 1 type input
    ($a:expr, $b:expr, $typ:ty) => {

        $a as $typ + $b as $typ

    }

}

////////////////////////////////////////////////////
macro_rules! add_as2{
    
    (
        $($a:expr)  // repeated block 
        ,           // separator
        *           // zero or more
    ) =>{

        {
            0
            $(+$a)*

        }
    }
    
}

macro_rules! add_as3{
    
    ($a:expr)=>{
        $a
    };

    ($a:expr,$b:expr)=>{
        {
            $a+$b
        }
    };

    ( $a:expr, $($b:tt)* )=>{
        {
            $a + add!( $($b)*  )
        }
    }
    
}

////////////////////////////////////////////////////



fn main() {

    // let x = vec_!(1, 2, 3, 4);
    let x = vec_![1, 2, 3, 4, 5];
    println!("{:?}", x);

    let x = add!(5,2);
    println!("{x}");


    let x = add2!(10, 20);
    println!("{x}");
    let x = add2!(10);
    println!("{x}");


    let x = add_as!(4, 2, u8);
    println!("{x}");


    let x = add_as2!(1, 2, 3);
    println!("{x}");

    let x = add_as3!(1 , 2 + 3 , 4);
    // println!("{x}");

}
