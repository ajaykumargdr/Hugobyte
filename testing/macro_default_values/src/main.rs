use serde_derive::{Serialize, Deserialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct T{

    // #[serde(default = "test")]
    #[serde(default = "empty_string")]
    var_1: String 
}

fn main() {


    // Deserialize JSON with name but without age
    let json_data = r#"{ }"#;
    let person: T = serde_json::from_str(json_data).expect("Failed to deserialize JSON");

    // The age field will be set to its default value (0 in this case)
    println!("Name: {}", person.var_1);

}
