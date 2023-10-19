extern crate proc_macro;

use proc_macro::TokenStream;

// use quote::quote;
// use syn::DeriveInput;

#[proc_macro]
pub fn staking_payout_input(_item:TokenStream)-> TokenStream{

    "#[derive(Default, Debug)]
    pub struct StakingPayoutInput {
            url:String,
            owner_key:String,
            address:String,
            era:u32,
    }".parse().unwrap()
}

/*
#[proc_macro]
pub fn make_struct(item:TokenStream)-> TokenStream{
    
    // parsing and creating struct out of it

    "#[derive(Default, Debug)]
    pub struct StakingPayoutInput {
            url:String,
            owner_key:String,
            address:String,
            era:u32,
    }".parse().unwrap()
}
*/