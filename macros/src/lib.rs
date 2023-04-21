extern crate proc_macro;

mod extend_container;
mod extend_element;
mod extend_object;
mod extend_widget;
mod reflect_trait;
mod trait_info;

use proc_macro::TokenStream;
use syn::{self, parse_macro_input, DeriveInput};
use trait_info::TraitInfo;

#[proc_macro_attribute]
pub fn extends_object(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);
    match extend_object::generate_extend_object(&mut ast) {
        Ok(tkn) => tkn.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[proc_macro_attribute]
pub fn extends_element(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);
    match extend_element::generate_extend_element(&mut ast) {
        Ok(tkn) => tkn.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[proc_macro_attribute]
pub fn extends_widget(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);
    match extend_widget::generate_extend_widget(&mut ast) {
        Ok(tkn) => tkn.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[proc_macro_attribute]
pub fn extends_container(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);
    match extend_container::generate_extend_container(&mut ast) {
        Ok(tkn) => tkn.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[proc_macro_attribute]
pub fn reflect_trait(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as TraitInfo);
    match reflect_trait::generate_reflect_trait(&mut ast) {
        Ok(tkn) => tkn.into(),
        Err(e) => e.to_compile_error().into(),
    }
}