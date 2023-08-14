fn main() {
    println!("Hello, world!");
    test_func();
    test_arg_func(50);
    test_arg2_func(10, 'i');
}

fn test_func(){
    println!("Test function is called");
}

fn test_arg_func(x : i32){
    println!("value of argument passed is: {x}");
}

fn test_arg2_func(x: i32, y: char){
    println!("Two parameters passed are: {x} {y}");
}

