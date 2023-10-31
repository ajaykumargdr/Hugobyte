use serde_json::Value;

macro_rules! make_struct {
    ($name:ident, [$($visibality:vis $field_name:ident: $field_type:ty),*]) => {
            struct $name {
                $($visibality $field_name: $field_type, )*
            }
    };
}


make_struct!(Action4Input, [address:String,url:String,owner_key:String,era:u32]);
make_struct!(Action4, [action_name: String, pub input: Action4Input, pub output: Value]);

make_struct!(Action3Input, [era:u32,owner_key:String,url:String,address:String]);
make_struct!(Action3, [action_name: String, pub input: Action3Input, pub output: Value]);

make_struct!(Action2Input, [owner_key:String,era:u32,url:String,address:String]);
make_struct!(Action2, [action_name: String, pub input: Action2Input, pub output: Value]);

make_struct!(Action1Input, [owner_key:String,address:String,url:String,era:u32]);
make_struct!(Action1, [action_name: String, pub input: Action1Input, pub output: Value]);

make_struct!(Action5Input, [url:String,address:String,owner_key:String,era:u32]);
make_struct!(Action5, [action_name: String, pub input: Action5Input, pub output: Value]);


fn main() {}

#[test]
fn generated_structs_test() {
    let a1 = Action1Input {
        url: "http".to_string(),
        era: 1,
        address: "aurras".to_string(),
        owner_key: "ow234bdn234ciouwndfuwbfo456wefc".to_string(),
    };

    let _ = Action1{
        action_name : "Action1".to_string(),
        input : a1,
        output : serde_json::to_value("sample output").unwrap(), 
    };

    let a2 = Action2Input {
        url: "http".to_string(),
        era: 1,
        address: "aurras".to_string(),
        owner_key: "ow234bdn234ciouwndfuwbfo456wefc".to_string(),
    };

    let _ = Action2{
        action_name : "Action2".to_string(),
        input : a2,
        output : serde_json::to_value("sample output").unwrap(), 
    };

    let a3 = Action3Input {
        url: "http".to_string(),
        era: 1,
        address: "aurras".to_string(),
        owner_key: "ow234bdn234ciouwndfuwbfo456wefc".to_string(),
    };

    let _ = Action3{
        action_name : "Action3".to_string(),
        input : a3,
        output : serde_json::to_value("sample output").unwrap(), 
    };

    let a4 = Action4Input {
        url: "http".to_string(),
        era: 1,
        address: "aurras".to_string(),
        owner_key: "ow234bdn234ciouwndfuwbfo456wefc".to_string(),
    };

    let _ = Action4{
        action_name : "Action4".to_string(),
        input : a4,
        output : serde_json::to_value("sample output").unwrap(), 
    };

    let a5 = Action5Input {
        url: "http".to_string(),
        era: 1,
        address: "aurras".to_string(),
        owner_key: "ow234bdn234ciouwndfuwbfo456wefc".to_string(),
    };

    let _ = Action5{
        action_name : "Action5".to_string(),
        input : a5,
        output : serde_json::to_value("sample output").unwrap(), 
    };
}
        
