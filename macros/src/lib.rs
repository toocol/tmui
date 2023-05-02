extern crate proc_macro;

mod cast;
mod extend_container;
mod extend_element;
mod extend_object;
mod extend_widget;
mod layout;
mod reflect_trait;
mod tasync;
mod trait_info;

use cast::CastInfo;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{self, parse::Parse, parse_macro_input, DeriveInput, Ident, Meta, Token};
use tasync::AsyncTaskParser;
use trait_info::TraitInfo;

struct ExtendAttr {
    extend: Ident,
    layout_meta: Option<Meta>,
    layout: Option<String>,
}
impl ExtendAttr {
    fn error<T: ToTokens>(span: T, msg: &'static str) -> syn::Result<Self> {
        Err(syn::Error::new_spanned(span, msg))
    }
}
impl Parse for ExtendAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let extend: Ident = input.parse()?;
        let mut layout = None;
        let layout_meta = if let Some(_) = input.parse::<Option<Token!(,)>>()? {
            let layout_meta = input.parse::<Meta>()?;
            if let Meta::List(syn::MetaList {
                ref path,
                ref nested,
                ..
            }) = layout_meta
            {
                if let Some(ident) = path.get_ident() {
                    if ident.to_string() != "Layout" {
                        return Self::error(
                            layout_meta,
                            "Only support attribute formmat `Layout(xxx)`",
                        );
                    }

                    if nested.len() != 1 {
                        return Self::error(
                            layout_meta,
                            "Only support attribute formmat `Layout(xxx)`",
                        );
                    }

                    if let Some(syn::NestedMeta::Meta(syn::Meta::Path(path))) = nested.first() {
                        if let Some(layout_ident) = path.get_ident() {
                            layout = Some(layout_ident.to_string())
                        }
                    } else {
                        return Self::error(
                            layout_meta,
                            "Only support attribute formmat `Layout(xxx)`",
                        );
                    }
                } else {
                    return Self::error(
                        layout_meta,
                        "Only support attribute formmat `Layout(xxx)`",
                    );
                }
                eprintln!("{:?}", path);
            } else {
                return Self::error(layout_meta, "Only support attribute formmat `Layout(xxx)`");
            }
            Some(layout_meta)
        } else {
            None
        };
        Ok(Self {
            extend,
            layout_meta,
            layout,
        })
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
/// ### Supported layouts:
///     - Stack
///     - VBox
///     - HBox
#[proc_macro_attribute]
pub fn extends(args: TokenStream, input: TokenStream) -> TokenStream {
    let extend_attr = parse_macro_input!(args as ExtendAttr);
    let mut ast = parse_macro_input!(input as DeriveInput);

    let extend_str = extend_attr.extend.to_string();
    if extend_str != "Widget" && extend_attr.layout_meta.is_some() {
        return syn::Error::new_spanned(
            extend_attr.layout_meta.unwrap(),
            format!(
                "`{}` was not supported layout, only `Widget` has layout.",
                extend_str
            ),
        )
        .to_compile_error()
        .into();
    }
    match extend_str.as_str() {
        "Object" => match extend_object::expand(&mut ast) {
            Ok(tkn) => tkn.into(),
            Err(e) => e.to_compile_error().into(),
        },
        "Element" => match extend_element::expand(&mut ast) {
            Ok(tkn) => tkn.into(),
            Err(e) => e.to_compile_error().into(),
        },
        "Widget" => match extend_attr.layout {
            Some(ref layout) => match extend_widget::expand_with_layout(
                &mut ast,
                extend_attr.layout_meta.as_ref().unwrap(),
                layout,
            ) {
                Ok(tkn) => tkn.into(),
                Err(e) => e.to_compile_error().into(),
            },
            None => match extend_widget::expand(&mut ast) {
                Ok(tkn) => tkn.into(),
                Err(e) => e.to_compile_error().into(),
            },
        },
        "Container" => match extend_container::expand(&mut ast, true) {
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

#[proc_macro_derive(Childrenable, attributes(children))]
pub fn childrenable_derive(_: TokenStream) -> TokenStream {
    TokenStream::new()
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
