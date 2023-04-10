extern crate proc_macro;

mod extend_object;
mod extend_element;
mod extend_widget;

use proc_macro::TokenStream;
use syn::{self, parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn extends_object(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);
    extend_object::generate_extend_object(&mut ast).into()
}

#[proc_macro_attribute]
pub fn extends_element(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);
    extend_element::generate_extend_element(&mut ast).into()
}

#[proc_macro_attribute]
pub fn extends_widget(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);
    extend_widget::generate_extend_widget(&mut ast).into()
}
