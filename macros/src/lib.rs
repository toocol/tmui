extern crate proc_macro;

mod cast;
mod extend_container;
mod extend_element;
mod extend_object;
mod extend_widget;
mod reflect_trait;
mod tasync;
mod trait_info;

use cast::CastInfo;
use proc_macro::TokenStream;
use syn::{self, parse::Parse, parse_macro_input, DeriveInput, Ident};
use tasync::AsyncTaskParser;
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

/// Let struct to extend specific type.<br>
/// This macro will implement a large number of traits automatically,
/// enable struct to obtain corresponding functions.<br>
/// ### Supported extends type:
///     - Object
///     - Element
///     - Widget
///     - Container
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
        _ => syn::Error::new_spanned(
            ast,
            format!("`{}` was not supported to extends.", extend_str),
        )
        .to_compile_error()
        .into(),
    }
}

/// Enable the trait has the ability of reflect, create the trait reflect struct.<br>
/// The struct implemented the reflected trait should defined [`extends`](crate::extends),
/// and register the reflect info to [`TypeRegistry`] in function [`ObjectImpl::type_register()`], like: <br>
/// ```ignore
/// ...
/// #[reflect_trait]
/// pub trait Trait {}
/// impl Trait for Foo {}
///
/// impl ObjectImpl for Foo {
///     fn type_register(&self, type_registry: &mut TypeRegistry) {
///         type_registry.register::<Foo, Trait>();
///     }
/// }
/// ...
/// ```
#[proc_macro_attribute]
pub fn reflect_trait(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as TraitInfo);
    match reflect_trait::generate_reflect_trait(&mut ast) {
        Ok(tkn) => tkn.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

/// Crate and run an async task in tokio worker threads. <br>
/// The return value in async block must implements [`tmui::tlib::values::ToValue`] trait. 
/// If there is no return value, `()` was needed.
/// ### without callback:
/// ```ignore
/// tasync!({
///     ...
/// });
/// ```
/// ### with callback:
/// ```ignore
/// tasync!({
///     ...  
/// } => {
///     ...
/// });
/// ```
/// ### callback with return value:
/// ```ignore
/// tasync!({
///     ...  
///    "result"
/// } => |result| {
///     ...
/// });
/// ```
#[proc_macro]
pub fn tasync(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as AsyncTaskParser).expand().into()
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
