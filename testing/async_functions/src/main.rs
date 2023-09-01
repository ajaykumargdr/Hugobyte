// use std::process::Output;

#[tokio::main]
async fn main() {
    
    // this function do nothing unless you do await or poll
    // my_function();

    // my_function().await;

    let f = my_function();

    println!("I will print first, becuase we are running asyncronised");

    f.await;    // function will print

}

/*
enum Poll<T>{
    Ready(T),
    Pending
}

trait Future{
    type Output;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

fn my_function() -> impl Future<Output = ()>{
    println!("I'm an async function!");
} 
*/

///////////////////

async fn my_function(){
    println!("I'm an async function");
    let s1 = read_from_database().await;
    println!("First result: {s1}");
    let s2 = read_from_database().await;
    println!("Second result: {s2}");
}

async fn read_from_database() -> String{
    "DB Result".to_owned()
}