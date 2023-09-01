extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{self, DeriveInput};


#[proc_macro]
pub fn make_answer(_item:TokenStream)-> TokenStream{

    "fn answer() -> &'static str { 
        \"macro created function\" 
    } ".parse().unwrap()
}

#[proc_macro]
pub fn make_another(item:TokenStream) -> TokenStream{

    "fn answer2() -> &'static str { 
        \"another macro created function\" 
    } ".parse().unwrap()

}