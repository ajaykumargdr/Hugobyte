extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn calculate_age(input: TokenStream) -> TokenStream {

    let input = parse_macro_input!(input as LitStr);
    let planet = input.value();

    let planet_age = match planet.as_str() {
        // TokenStream [Punct { char: '/', spacing: Alone }, Literal { lit: 0.2408467 }]
        "Mercury" => quote! { 0.2408467 },    // (AST -> TS)
        "Venus" => quote! { 0.61519726 },
        "Earth" => quote! { 1.0 },
        "Mars" => quote! { 1.8808158 },
        "Jupiter" => quote! { 11.862615 },
        "Saturn" => quote! { 29.447498 },
        "Uranus" => quote! { 84.016846 },
        "Neptune" => quote! { 164.79132 },
        _ => panic!("Unknown planet"),
    };

    let expanded = quote! {

        (|age_in_seconds: f64| {
            age_in_seconds / (31557600.0 * #planet_age)
        })
        
    };

    expanded.into()
}