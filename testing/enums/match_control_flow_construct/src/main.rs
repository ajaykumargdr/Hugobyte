#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska
}


#[derive(Debug)]
// #[derive(PartialEq)]
enum Coin{
    Penny,
    Nickel,
    Dime,
    Quater
}

// pattern matching (matches the keys and return)

fn value_in_coin(coin: Coin) -> u8{
    match coin{
        Coin::Penny => 1,
        Coin::Nickel => 5,

        // also you can have block of code

        Coin::Dime => {
            println!("Got inside the block");
            10
        }

        Coin::Quater(state) => {
10
        }

    }
}

fn main() {

    let coin = Coin::Penny;
    println!("{}", value_in_coin(coin));    // 1

    println!("{}", value_in_coin(Coin::Nickel));//5
    println!("{}", value_in_coin(Coin::Dime)); //msg +10
    println!("{}", value_in_coin(Coin::Quater(UsState::Alaska)));//

}
