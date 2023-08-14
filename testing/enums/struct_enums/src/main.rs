#[derive(Debug)]
enum IpAddrKind{
    V4,
    V6
}


#[derive(Debug)]
struct IpAddr{
    kind: IpAddrKind,
    address: String
}

fn main() {
    
    
    let home = IpAddr{
        kind : IpAddrKind::V6,
        address: String::from("120.0.0.1")
    };

    println!("{:?}", home);

}