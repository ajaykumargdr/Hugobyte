use std::collections::HashMap;
use serde_json::Value;
use std::fmt::{self, Display};

// impl Display for HashMap<String, String>{
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{} + {}i", self.real, self.imaginary)
//     }
// }

fn main() {

    let mut hm = HashMap::<String, String>::new();
    hm.insert("1key".to_string(), "1val".to_string());
    hm.insert("2key".to_string(), "2val".to_string());
    hm.insert("3key".to_string(), "3val".to_string());


    // let val = serde_json::to_value( format!("{:#?}", hm.to_string())  ).unwrap();

    // let op:String = serde_json::from_value(val).unwrap();

    println!("{:?}", format!("{:?}",hm) );

}
