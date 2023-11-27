mod example1 {
    use types_macro::*;
    staking_payout_input!();

    #[test]
    fn procedural_approach() {
        let _spi = StakingPayoutInput {
            url: "https://url".to_string(),
            owner_key: "ownerkey1234".to_string(),
            address: "address".to_string(),
            era: 1,
        };
    }
}

mod example2 {
    use serde_json::Value;

    macro_rules! make_input_struct {
        (
            $name:ident,
            [$($visibality:vis $element:ident : $ty:ty),*],
            [$($der:ident),*]
    ) => {
            #[derive($($der),*)]
            struct $name { $($visibality  $element: $ty),*}
        }
    }

    macro_rules! make_main_struct {
        (
            $name:ident,
            $input:ty,
            [$($der:ident),*],
            [$($key:ident : $val:expr),*]
    ) => {
            #[derive($($der),*)]
            $(
                #[$key = $val]
            )*
            struct $name {
                action_name: String,
                pub input: $input,
                pub output: Value,
            }
        }
    }

    macro_rules! make_impl{
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

    macro_rules! impl_setter{
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
        [url: String, owner_key: String, address: String, era: u32],
        [Default, Debug, Clone]
    );  

    make_main_struct!(
        Cartype,
        CartypeInput,
        [Default, Debug, Clone],
        []  // attributes
    );

    // make_impl!(Cartype, CartypeInput,[]);
    make_impl!(Cartype, CartypeInput,[address:String, era:u32]);
    impl_setter!(Cartype, [url:"endpoint", owner_key:"auth_key"]);

    #[test]
    fn declarative_approach() {
        let _spi = CartypeInput {
            url: "https://url".to_string(),
            owner_key: "ownerkey1234".to_string(),
            address: "address".to_string(),
            era: 1,
        };

        let mut ct = Cartype::new("https://url".to_string(),1,"actionX".to_string());
        
        let str_ =r#"{"endpoint":"https://url","auth_key":"ownerkey1234"}"#;
        let mut val:Value = serde_json::from_str(str_).unwrap();

        ct.setter(val);

        eprintln!("{:#?}", ct);

        // {
        //     make_input_struct!(Employee_idsInput, [role:String], [Debug, Clone]);
        //     make_main_struct!(Employee_ids, Employee_idsInput, [Debug, Clone], []);
        //     make_input_struct!(SalaryInput, [details:HashMap<i32,(i32,String)>], [Debug, Clone]);
        //     make_main_struct!(Salary, SalaryInput, [Debug, Clone], []);
        //     make_input_struct!(GetaddressInput, [id:i32], [Debug, Clone]);
        //     make_main_struct!(Getaddress, GetaddressInput, [Debug, Clone], []);
        //     make_input_struct!(GetsalariesInput, [id:i32], [Debug, Clone]);
        //     make_main_struct!(Getsalaries, GetsalariesInput, [Debug, Clone], []);
        //     make_input_struct!(Input, [role:String], []);
        // }
    }
}

mod example3 {

    // to convert literals into identifiers
    use paste::paste;

    // creating macro
    macro_rules! make_struct {
        ($name:literal, [$(($field_name:literal: $field_type:literal)),*]) => {

            // converting literals into identifiers
            paste!{
                struct [<$name>] {
                    $( [<$field_name>]: [<$field_type>], )*
                }
            }
        };
    }

    // invoking the macro
    make_struct!("StakingPayoutInput", [("url": "String"), ("owner_key": "String"), ("address": "String"), ("era": "u32")]);

    #[test]
    fn procedural_approach() {
        let _spi = StakingPayoutInput {
            url: "https://url".to_string(),
            owner_key: "ownerkey1234".to_string(),
            address: "address".to_string(),
            era: 1,
        };

    }
}

fn main() {

    println!("{:?}", stringify!(sdf));

}
