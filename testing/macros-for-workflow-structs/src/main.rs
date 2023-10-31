/*
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
*/
mod example2 {
    macro_rules! make_struct {
        ($x:ident, $( $visibality:vis $element: ident: $ty: ty),*) => {
            #[derive(Debug)]
            struct $x { $($visibality  $element: $ty),* }
        }
    }

    make_struct!(StakingPayoutInput, pub url: String, owner_key: String, pub address: String, era: u32);

    #[test]
    fn declarative_approach() {
        let _spi = StakingPayoutInput {
            url: "https://url".to_string(),
            owner_key: "ownerkey1234".to_string(),
            address: "address".to_string(),
            era: 1,
        };

        println!("{:?}", _spi.url);
    }
}
/*
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
*/

fn main() {}