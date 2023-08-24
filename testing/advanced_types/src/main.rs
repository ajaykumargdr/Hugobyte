#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}

fn pass_fn_name(){

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> =
        // list_of_numbers.iter().map(|i| i.to_string()).collect();

        list_of_numbers.iter().map(ToString::to_string).collect();
    
    println!("{:?}", list_of_strings);
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn main() {

    type Kilometers = i32;
    let x: Kilometers = 5;
    println!("{x}");

    type Res = Result<i32, String>;
    let y: Res = Ok(1);
    let z: Res = Err(String::from("Error"));

    println!("{:?} {:?}", y, z);
    
    let answer = do_twise(add_one, 5);
    println!("{answer}");

    pass_fn_name();

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    println!("{:?}", list_of_statuses);


    let cl1 = returns_closure();

    println!("{}", cl1(10));  
}

// ! that never returns any value
fn never_type_return(x: Option<i32>) -> !{

    match x{
        Some(val) => panic!("Got some value"),
        None => panic!("Didn't get any value")
    }
} 

fn add_one(x: i32) -> i32{
    x + 1 
}

fn do_twise(f: fn(i32)->i32, arg: i32) -> i32{
    f(arg) + f(arg)
}