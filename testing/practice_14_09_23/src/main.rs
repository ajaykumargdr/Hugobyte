use serde::{Serialize, Deserialize };

use serde_json::Value;

#[derive(Serialize, Deserialize)]
struct ActionName{
    name: String
}

fn main() {

    let json_value = r#"{"name": "icon-eth-notification", "trigger": "0a36fd24-84ac-420e-9187-912929c782ea"}"#;

    let data:Value = serde_json::from_str(json_value).unwrap();

    if let Some(value) =  data.get("name") {

        // let value = String::from("test"); 
        println!("{:?}", value.to_string().replace("\"", "") )
    } else {
        println!("");
    }
}
