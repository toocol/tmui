use quote::quote;
use syn::{DeriveInput, parse::Parser};

pub(crate) fn expand(ast: &mut DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &ast.ident;

    match &mut ast.data {
        syn::Data::Struct(ref mut struct_data) => match &mut struct_data.fields {
            syn::Fields::Named(fields) => {
                fields.named.push(syn::Field::parse_named.parse2(quote! {
                    pub container: Container
                })?);
                fields.named.push(syn::Field::parse_named.parse2(quote! {
                    pub container: Container
                })?);
                Ok(quote! {})
            }
            _ => {
                return Err(syn::Error::new_spanned(
                    ast,
                    "`extends(ScrollArea)` should defined on named fields struct.",
                ))
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(
                ast,
                "`extend(ScrollArea)` has to be used with structs.",
            ))
        }
    }
}
