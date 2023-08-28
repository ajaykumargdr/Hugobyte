extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(SolarFam)]
pub fn solarfam_derive_macro(item: TokenStream) -> TokenStream {
    // parse TokenStream object to DeriveInput(Abstract Syntax Tree) object
    let ast: DeriveInput = syn::parse(item).unwrap();
    let ident = ast.ident;

    // Find the planet name using the identifier name
    let planet_age = match ident.to_string().as_str() {
        "Mercury" => quote! { 0.2408467 },
        "Venus" => quote! { 0.61519726 },
        "Earth" => quote! { 1.0 },
        "Mars" => quote! { 1.8808158 },
        "Jupiter" => quote! { 11.862615 },
        "Saturn" => quote! { 29.447498 },
        "Uranus" => quote! { 84.016846 },
        "Neptune" => quote! { 164.79132 },
        _ => panic!("Unknown planet"),
    };

    // implement the function and return as TokenStream object
    quote::quote! {
        impl #ident{
            fn get_age(age_in_seconds: f64) -> f64{
                age_in_seconds / (31557600.0 * #planet_age)
            }
        }
    }
    .into()
}
