fn main() {
    
// if

    let number = 3;

    if number < 5{
        println!("Condition was true");
    }
    else {
        println!("Condition is not working");
    }

// Note :-
    // condition should be bool
    // should not close condition -() 

// else if 

    let number = 2;

    if number % 4 == 0{
        println!("divisable by 4");
    } else if number % 3 == 0{
        println!("divisable by 3");
    } else if number % 2 == 0{
        println!("divisable by 2");
    } else{
        println!("not divisable by 4 | 3 | 2 ");
    }

// if in let statement

    let number = if 5>6 { "GT6" } else {"LTE6"};
    
    // not works
    // let number = if 5<6 {true} else {0}; 

    println!("{number}");

    // works 
    let number = if 6<2 { "GT2" } 
        else if 3<4 {"LTE4"}
        else {"L"};  

    println!("{number}");

}
