// cargo build --target wasm32-unknown-unknown
// wasm-bindgen --target web --out-dir pkg ./target/wasm32-unknown-unknown/debug/wasm_building_1.wasm  

// rustup target add wasm32-wasi

// cargo build --release --target wasm32-wasi 

use serde_json::Value;
use serde_derive::{Serialize, Deserialize};

macro_rules! make_input_struct {
    (
        $x:ident,
        // list of field and it's type
        [$($visibility:vis $element:ident : $ty:ty),*],
        // list of derive macros
        [$($der:ident),*]
) => {
        #[derive($($der),*)]
        pub struct $x { $($visibility  $element: $ty),*}
    }
}

macro_rules! make_main_struct {
    (
        $name:ident,
        $input:ty,
        [$($der:ident),*],
        // list of attributes
        [$($key:ident : $val:expr),*]
) => {
        #[derive($($der),*)]
        $(
            #[$key = $val]
        )*
        pub struct $name {
            action_name: String,
            pub input: $input,
            pub output: Value,
        }
        impl $name{
            pub fn output(&self) -> Value {
                self.output.clone()
            }
        }
    }
}

macro_rules! impl_new {
    (
        $name:ident,
        $input:ident,
        []
    ) => {
        impl $name{
            pub fn new(action_name:String) -> Self{
                Self{
                    action_name,
                    input: $input{
                        ..Default::default()
                    },
                    ..Default::default()
                }      
            }
        }
    };
    (
        $name:ident,
        $input:ident,
        [$($element:ident : $ty:ty),*]
    ) => {
        impl $name{
            pub fn new($( $element: $ty),*, action_name:String) -> Self{
                Self{
                    action_name,
                    input: $input{
                        $($element),*,
                        ..Default::default()
                    },
                    ..Default::default()
                }      
            }
        }
    }
}

macro_rules! impl_setter {
    (
        $name:ty,
        [$($element:ident : $key:expr),*]
    ) => {
        impl $name{
            pub fn setter(&mut self, val: Value) {
                $(
                let value = val.get($key).unwrap();
                self.input.$element = serde_json::from_value(value.clone()).unwrap();
                )*
            }
        }
    }
}

make_input_struct!(
    CartypeInput,
    [car_type:String],
	[Debug, Clone, Default, Serialize, Deserialize]);
make_main_struct!(
    Cartype,
    CartypeInput,
    [Debug, Clone, Default, Serialize, Deserialize],
    []
);
impl_new!(
    Cartype,
    CartypeInput,
    [car_type:String]
);
impl_setter!(Cartype, []);

make_input_struct!(
    ModelspriceInput,
    [models:Vec<String>],
	[Debug, Clone, Default, Serialize, Deserialize]);

fn main(){

    let cartype_input = CartypeInput{
        ..Default::default()
    };

    let cartype = Cartype{
        action_name: "test".to_string(),
        input : cartype_input,
        ..Default::default()
    };

    println!("{cartype:#?}");

    println!("hello world");
}

// wasmtime wasm32-wasi/release/