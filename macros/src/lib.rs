extern crate proc_macro;

mod animation;
mod async_do;
mod async_task;
mod cast;
mod childable;
mod extend_attr;
mod extend_container;
mod extend_element;
mod extend_object;
mod extend_popup;
mod extend_shared_widget;
mod extend_widget;
mod general_attr;
mod layout;
mod loadable;
mod pane;
mod popupable;
mod reflect_trait;
mod scroll_area;
mod split_pane;
mod stack;
mod trait_info;

use async_do::AsyncDoParser;
use cast::CastInfo;
use extend_attr::ExtendAttr;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use syn::{self, parse_macro_input, DeriveInput, ImplGenerics, TypeGenerics, WhereClause};
use trait_info::TraitInfo;

pub(crate) type SplitGenericsRef<'a> = (
    &'a ImplGenerics<'a>,
    &'a TypeGenerics<'a>,
    &'a Option<&'a WhereClause>,
);

/// Let struct to extend specific type.<br>
/// This macro will implement a large number of traits automatically,
/// enable struct to obtain corresponding functions.<br>
///  
/// - ### Supported extends type:
///     - Object
///     - Element
///     - Widget
///     - SharedWidget
///     - Container
///     - Popup
/// - ### Supported layouts:
///     - Stack
///     - VBox
///     - HBox
///     - SplitPane
///     - ScrollArea
///     - Pane
#[proc_macro_attribute]
pub fn extends(args: TokenStream, input: TokenStream) -> TokenStream {
    let extend_attr = parse_macro_input!(args as ExtendAttr);
    let mut ast = parse_macro_input!(input as DeriveInput);

    match extend_attr.check() {
        Err(e) => return e.to_compile_error().into(),
        _ => {}
    }

    let extend_str = extend_attr.extend.to_string();
    match extend_str.as_str() {
        "Object" => match extend_object::expand(&mut ast, extend_attr.ignore_default) {
            Ok(tkn) => tkn.into(),
            Err(e) => e.to_compile_error().into(),
        },
        "Element" => match extend_element::expand(&mut ast, extend_attr.ignore_default) {
            Ok(tkn) => tkn.into(),
            Err(e) => e.to_compile_error().into(),
        },
        "Widget" => match extend_attr.layout {
            Some(ref layout) => match extend_widget::expand_with_layout(
                &mut ast,
                extend_attr.layout_meta.as_ref().unwrap(),
                layout,
                extend_attr.internal,
                extend_attr.ignore_default,
            ) {
                Ok(tkn) => tkn.into(),
                Err(e) => e.to_compile_error().into(),
            },
            None => match extend_widget::expand(&mut ast, extend_attr.ignore_default) {
                Ok(tkn) => tkn.into(),
                Err(e) => e.to_compile_error().into(),
            },
        },
        "SharedWidget" => match extend_shared_widget::expand(&mut ast, extend_attr.id.as_ref(), extend_attr.ignore_default) {
            Ok(tkn) => tkn.into(),
            Err(e) => e.to_compile_error().into(),
        },
        "Container" => {
            match extend_container::expand(
                &mut ast,
                extend_attr.ignore_default,
                true,
                false,
                false,
                false,
                false,
                false,
                false,
            ) {
                Ok(tkn) => tkn.into(),
                Err(e) => e.to_compile_error().into(),
            }
        }
        "Popup" => match extend_popup::expand(&mut ast, extend_attr.ignore_default) {
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

#[proc_macro_derive(Childable, attributes(child))]
pub fn childable_derive(_: TokenStream) -> TokenStream {
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
///         type_registry.register::<Foo, ReflectTrait>();
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

/// The widget annotated `[run_after]` will execute the [`WidgetImpl::run_after()`] function
/// after the application was started.
///
/// ### Only taing effect when struct annotated `#[extends(Widget)]`
#[proc_macro_attribute]
pub fn run_after(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
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
pub fn async_do(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as AsyncDoParser).expand().into()
}

/// arguments:
///
/// `name`: Upper camel case needed, the name of async task struct, same as the async function.
///
/// `value``: General param of async block's return value
#[proc_macro_attribute]
pub fn async_task(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
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

#[proc_macro]
pub fn split_pane_impl(input: TokenStream) -> TokenStream {
    let ident = parse_macro_input!(input as Ident);
    let use_prefix = Ident::new("crate", ident.span());
    match split_pane::generate_split_pane_impl(&ident, &use_prefix) {
        Ok(tkn) => tkn.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[proc_macro]
pub fn stack_impl(input: TokenStream) -> TokenStream {
    let ident = parse_macro_input!(input as Ident);
    let use_prefix = Ident::new("crate", ident.span());
    match stack::generate_stack_impl(&ident, &use_prefix) {
        Ok(tkn) => tkn.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[proc_macro]
pub fn pane_impl(input: TokenStream) -> TokenStream {
    let ident = parse_macro_input!(input as Ident);
    match pane::generate_pane_impl(&ident) {
        Ok(tkn) => tkn.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[proc_macro]
pub fn pane_init(_: TokenStream) -> TokenStream {
    match pane::generate_pane_inner_init() {
        Ok(tkn) => tkn.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[proc_macro]
pub fn pane_type_register(input: TokenStream) -> TokenStream {
    let ident = parse_macro_input!(input as Ident);
    match pane::generate_pane_type_register(&ident) {
        Ok(tkn) => tkn.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

/// arguments:
///
/// `ty`: the type([`tmui::animation::Animation`]) of animation.
///
/// `direction`: the direction([`tmui::animation::Direction`]) of animation.
///
/// `duration`: the time duration of animation(millis).
#[proc_macro_attribute]
pub fn animatable(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn popupable(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn loadable(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}
