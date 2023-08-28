extern crate my_macro;

use my_macro::show_streams;

// Example: Basic function
#[show_streams]
pub fn invoke1() {}
// out: attr: ""
// out: item: "fn invoke1() {}"

// Example: Attribute with input
#[show_streams(bar)]
pub fn invoke2() {}
// out: attr: "bar"
// out: item: "fn invoke2() {}"

// Example: Multiple tokens in the input
#[show_streams(multiple => tokens)]
pub fn invoke3() {}
// out: attr: "multiple => tokens"
// out: item: "fn invoke3() {}"

#[show_streams { delimiters }]
pub fn invoke4() {}
// out: attr: "delimiters"
// out: item: "fn invoke4() {}"

#[show_streams [ some tokens ] ]
pub fn invoke5() {}
// out: attr: "some tokens"
// out: item: "pub fn invoke5() {}"