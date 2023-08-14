#[derive(Debug)]
enum Message{
    Quit,
    Move{ x:i32, y: i32},   // ** must use {}
    Write(String),
    ChangeColor(i32, i32, i32)
}

// Note: Always remember enum is group of single units 
// method implementation for enum 
// single implementation for all 
// !(can't print everything at same time)

impl Message{
    fn print(&self){
        dbg!(self);
    }
}

fn main() {
    
    let quit_val = Message::Quit;
    quit_val.print();

    Message::Move{x:-64, y:64}.print();
    Message::Write(String::from("Wrote msg")).print();
    Message::ChangeColor(1,2,3).print();


}
