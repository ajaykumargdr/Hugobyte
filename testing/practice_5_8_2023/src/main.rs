#![allow(unused)]
fn main() {

    let a = [1, 2, 3];

    let mut x = Some(a);

    let mut s = String::from("hugobyte"); 

    let ref mut a = s;

    a.push('b');

    println!("{a}");

}