#[derive(Debug)]
#[derive(PartialEq)]
// simple Enum
enum IpAddrKind{
    V4,
    V6
}


fn main() {
    
// Simple Enum print

    println!("{:#?}", IpAddrKind::V4);
    println!("{:?} ", IpAddrKind::V6);
    
    let x:IpAddrKind = IpAddrKind::V6;

    // not working (must derive (PartialEq))
    if x==IpAddrKind::V6{
        println!("x is euaqls to v6");
    }



}
