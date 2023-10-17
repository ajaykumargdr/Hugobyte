extern crate serde_json;

use serde_derive::{Deserialize, Serialize};
use serde_json::{Error, Value};

fn unknown() -> String {
    "unknown".to_string()
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Input {
    #[serde(default = "unknown")]
    name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Output {
    greeting: String,
}

pub fn main(args: Value) -> Result<Value, Error> {
    let input: Input = serde_json::from_value::<Input>(args)?;

    match input.name.as_str() {
        "unknown" => Err(format!("person not found!")).map_err(serde::de::Error::custom),

        _ => {
            let output = Output {
                greeting: format!("Hello! {}", input.name),
            };
            serde_json::to_value(output)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn openwhisk_action_pass() {
        let ret = super::main(serde_json::json!(
            {
                "name": "Person"
            }
        )).unwrap();
        let output: Output = serde_json::from_value::<Output>(ret).unwrap();
        assert_eq!(output.greeting, "Hello! Person")
    }

    #[test]
    #[should_panic(expected = "person not found!")]
    fn openwhisk_action_fail() {
        let ret = super::main(serde_json::json!({})).unwrap();
        serde_json::from_value::<Output>(ret).unwrap();
    }
}
