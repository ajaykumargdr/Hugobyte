fn main() {

    let test_closures = || println!("No para closures");
    test_closures();

    let test_closures = |x| x;
    println!("{}",test_closures(8));
    // println!("{}",test_closures("This is string"));  //error
    

    let test_closures = |x:u32| x;
    println!("{}",test_closures(5) );

// #########################################################

    let print_ = |x:&str| println!("Printed by print closure {x}");
    print_("first");
    print_("second");

    let print_multi= || {
        println!("Printed first line");
        println!("Printed second line");
    };

    print_multi();

// #########################################################

    let mut list = vec![1, 2, 3];
    println!("{:?}", list);

    let mut mutable_closure = || list.push(0); 

    mutable_closure();          // mut
    println!("{:?}", list);     // immut

    // mutable_closure();          // mut
    // println!("{:?}", list);    

    // Note: it is taking mut/immutable reference only 
// #########################################################

    // if we want to move the ownership we can use 'move' 

    let list= vec![1,2,3];

    let moving_closure =move || println!("List is moved {:?}", list);  

    moving_closure();
    moving_closure();   // works 

    // println!("Moved list is {:?}", list);   // not works (moved)

// #########################################################


    let mut list = [
        Rectangle{width: 100, height: 1},
        Rectangle { width: 3, height: 50 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|r| r.width);
    // list.sort_by_key(|r| r.height * r.width);
    println!("{:#?}", list);
}

/*
fn ret_test<T,F >(f:F) -> T
where F: FnOnce() ->  T
{
    f()
} */


#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32
}

