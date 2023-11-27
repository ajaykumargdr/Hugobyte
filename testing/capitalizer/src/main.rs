pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn main() {
    println!("Hello, world!");

    // println!("{:?}", capitalize("thisis"));

    // let v1:Vec<String> = vec![String::from("t1:String"), String::from("t2:usize"), String::from("t3:Vector<String>"), String::from("t4:i32")];

    // let x = v1.join(",");
    // println!("{:#?}", format!(",[{x}],"));

    // println!("{:#?}", v1);

    // let mut s = String::from("Testing");

    // let x = s.

    // println!("{:?} {:?}", s.to_lowercase(), s);


    ///////////////////////// string split

    // let x = String::from("t1:String");
    // let y = x.split(":").collect::<Vec<&str>>()[0];
    // println!("{:#?}", y );

    // 2

    let v1:Vec<String> = vec![String::from("t1:String"), String::from("t2:usize"), String::from("t3:Vector<String>"), String::from("t4:i32"),];

    let x:Vec<String> = v1.iter().map(|x|  format!("input.{}",x.split(":").collect::<Vec<&str>>()[0])).collect();

    println!("{:#?}", x.join(",")); 
     


}
