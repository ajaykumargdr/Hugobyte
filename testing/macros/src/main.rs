use std::vec;

#[macro_export]
macro_rules! vec_ { // macro name
    
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {

    // let x = vec_!(1, 2, 3, 4);
    let x = vec_![1, 2, 3, 4, 5];

    println!("{:?}", x);

}
