mod calculator;
mod tests;

fn main() {
    println!("{:?} ", calculator::add(5, 50 ) );
    println!("{:?} ", calculator::sub(5, 50 ) );
    println!("{:?} ", calculator::mul(5, 50 ) );
    println!("{:?} ", calculator::div(5, 0 ) );
}
