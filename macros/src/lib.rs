extern crate proc_macro;

mod cast;
mod extend_container;
mod extend_element;
mod extend_object;
mod extend_widget;
mod reflect_trait;
mod trait_info;

use cast::CastInfo;
use proc_macro::TokenStream;
use syn::{self, parse::Parse, parse_macro_input, DeriveInput, Ident};
use trait_info::TraitInfo;

struct ExtendAttr {
    extend: Ident,
}
impl Parse for ExtendAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let extend: Ident = input.parse()?;
        Ok(Self { extend })
    }
}

#[proc_macro_attribute]
pub fn extends(args: TokenStream, input: TokenStream) -> TokenStream {
    let extend_attr = parse_macro_input!(args as ExtendAttr);
    let mut ast = parse_macro_input!(input as DeriveInput);

    let extend_str = extend_attr.extend.to_string();
    match extend_str.as_str() {
        "Object" => match extend_object::generate_extend_object(&mut ast) {
            Ok(tkn) => tkn.into(),
            Err(e) => e.to_compile_error().into(),
        },
        "Element" => match extend_element::generate_extend_element(&mut ast) {
            Ok(tkn) => tkn.into(),
            Err(e) => e.to_compile_error().into(),
        },
        "Widget" => match extend_widget::generate_extend_widget(&mut ast) {
            Ok(tkn) => tkn.into(),
            Err(e) => e.to_compile_error().into(),
        },
        "Container" => match extend_container::generate_extend_container(&mut ast) {
            Ok(tkn) => tkn.into(),
            Err(e) => e.to_compile_error().into(),
        },
        _ => (syn::Error::new_spanned(
            ast,
            format!("`{}` was not supported to extends.", extend_str),
        ))
        .to_compile_error()
        .into(),
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

#[proc_macro]
pub fn cast(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as CastInfo);
    match ast.expand() {
        Ok(tkn) => tkn.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[proc_macro]
pub fn cast_mut(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as CastInfo);
    match ast.expand_mut() {
        Ok(tkn) => tkn.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[proc_macro]
pub fn cast_boxed(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as CastInfo);
    match ast.expand_boxed() {
        Ok(tkn) => tkn.into(),
        Err(e) => e.to_compile_error().into(),
    }
}
