// enum single valued 
#[derive(Debug)]
enum IpAddr{
    V4(String),
    V6(String)
}

// multi valued

#[derive(Debug)]
enum IpAddr2{
    V4(u8, u8, u8, u8),
    V6(String)
}

// more multivalued with var names

#[derive(Debug)]
enum Message{
    Quit,
    Move{ x:i32, y: i32},   // ** must use {}
    Write(String),
    ChangeColor(i32, i32, i32)
}

// single struct to include all these values
#[derive(Debug)]
struct pMessage(Message, Message, Message, Message);

fn main() {

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    
    println!("{:?}", home);

// multi valued

    let home = IpAddr2::V4(127, 0, 0, 1);
    let loopback = IpAddr2::V6(String::from("::1"));

    println!("{:?}", home);
    println!("{:?}", loopback);

// more named multivalued

    let quit_val = Message::Quit;
    let move_val = Message::Move{x:60, y:50};
    let write_val = Message::Write(String::from("Wrote"));
    let color_val = Message::ChangeColor(255, 255, 255); 

    let pmsg = pMessage(
        quit_val,
        move_val,
        write_val,
        color_val
    );

    println!("{:?}", pmsg);

}
