macro_rules! make_struct {
    ($name:ident, [$($field_name:ident: $field_type:ident),*]) => {
        // converting literals into identifiers
            struct $name {
                $( $field_name: $field_type, )*
            }
    };
}

make_struct!(Action1Input, [owner_key:String,address:String,url:String,era:u32]);
make_struct!(Action5Input, [owner_key:String,era:u32,url:String,address:String]);
make_struct!(Action3Input, [era:u32,url:String,address:String,owner_key:String]);
make_struct!(Action2Input, [url:String,address:String,owner_key:String,era:u32]);
make_struct!(Action4Input, [url:String,address:String,era:u32,owner_key:String]);

fn main() {}

#[test]
fn generated_structs_test() {
    let a1 = Action1Input {
        url: "http".to_string(),
        era: 1,
        address: "aurras".to_string(),
        owner_key: "ow234bdn234ciouwndfuwbfo456wefc".to_string(),
    };

    let a2 = Action2Input {
        url: "http".to_string(),
        era: 1,
        address: "aurras".to_string(),
        owner_key: "ow234bdn234ciouwndfuwbfo456wefc".to_string(),
    };

    let a3 = Action3Input {
        url: "http".to_string(),
        era: 1,
        address: "aurras".to_string(),
        owner_key: "ow234bdn234ciouwndfuwbfo456wefc".to_string(),
    };

    let a4 = Action4Input {
        url: "http".to_string(),
        era: 1,
        address: "aurras".to_string(),
        owner_key: "ow234bdn234ciouwndfuwbfo456wefc".to_string(),
    };

    let a5 = Action5Input {
        url: "http".to_string(),
        era: 1,
        address: "aurras".to_string(),
        owner_key: "ow234bdn234ciouwndfuwbfo456wefc".to_string(),
    };
}
        
