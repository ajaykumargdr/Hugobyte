fn main() {
    println!("Hello, world!");
    println!("{}",factorial(5));
}

fn factorial(x: i32) -> i32{

    if x == 1{
        return 1;
    }

    x * factorial(x-1)
}

#[cfg(test)]
#[test]
fn factorial_(){
    assert_eq!(factorial(5), 120);
}

