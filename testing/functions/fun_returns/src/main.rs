fn main() {
    let x = five();
    println!("{x}");
    println!("{}", number(760));
}

fn five()-> i32{
    5
}

fn number(x: i32) -> i32{
    if x<20 {
        return 0
    };

    x+1
}

