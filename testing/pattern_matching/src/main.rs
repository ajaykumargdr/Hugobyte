fn _pattern_matching(){
    let x = 33;

    match x{
        3 => println!("Three"),
        4 => println!("Four"),
        5 => println!("Five"),
        _ => println!("UnIdentified Number")
    }

    let x = String::from("Zuno");

    match x.as_str(){
        "Zuno" => println!("Got Zuno"),
        "Teena" => println!("Got Teena"),
        "Simba" => println!("Got Simba"),
        _ => println!("unidentified string")
    }

}

fn _if_let(){

    let i = Some(String::from("Rust"));

    if let Some(s) = i {
        println!("{:?}", s);
    }

    // println!("{:?}", i );   // value moved

}

fn _tricky_matching(){
    let x = Some(5);
    let y = 10;

    match x {

        // Some of exact 50
        Some(50) => println!("Got 50"),

        // Some of i32
        Some(y) => println!("Matched, y = {y}"),  // here type of y only matched

        _ => println!("Default case, x = {:?}", x),
    }

    
    println!("at the end: x = {:?}, y = {y}", x);
}

fn _or_match(){
    let x = 1;

    match x {

        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("some number")
    }

    println!("{x}");
}

fn _rang_match(){
 
   let x = 15;

    match x {
        1..=5 => println!("one through five"),
        10..=20 => println!("ten through twenty"),
        _ => println!("something else"),
    }

///////////////// char rang match ///////////////////

    let x = 'c';

    match x {
        'a'..='j' => println!("a through j"),
        'k'..='z' => println!("k through z"),
        _ => println!("something else"),
    }

}

fn _extra_conditional_match(){

    let num = Some(3);

    match num{
        Some(x) if x % 2 == 0 => println!("Even number"),
        Some(_) => println!("Odd number"),
        None => ()
    }

}

enum Message {
    Hello { id: i32 },
}

fn _bind_match(){

    let msg = Message::Hello { id: 5 };

    match msg {

        Message::Hello { id: id_variable @ 3..=7,} => 
            println!("Found an id in range: {}", id_variable),

        Message::Hello { id: 10..=12 } => 
            println!("Found an id in another range"),

        Message::Hello { id } => 
            println!("Found some other id: {}", id),
    }

}

fn main() {

    // _pattern_matching();
    // _if_let();
    // _tricky_matching();
    // _or_match();
    // _rang_match();
    // _extra_conditional_match();

    _bind_match();
}
