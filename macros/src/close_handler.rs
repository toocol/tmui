use crate::SplitGenericsRef;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::DeriveInput;

pub(crate) struct CloseHandler<'a> {
    name: Ident,
    generics: SplitGenericsRef<'a>,
}

impl<'a> CloseHandler<'a> {
    pub(crate) fn parse(ast: &DeriveInput, generics: SplitGenericsRef<'a>) -> syn::Result<Self> {
        Ok(Self {
            name: ast.ident.clone(),
            generics,
        })
    }

    pub(crate) fn close_handler_reflect(&self) -> TokenStream {
        let name = &self.name;
        let (_, ty_generics, _) = self.generics;
        quote!(
            type_registry.register::<#name #ty_generics, ReflectCloseHandler>();
        )
    }

    pub(crate) fn close_handler_impl(&self) -> TokenStream {
        let name = &self.name;
        let (impl_generics, ty_generics, where_clause) = self.generics;

        quote!(
            impl #impl_generics CloseHandlerRequire for #name #ty_generics #where_clause {}
        )
    }

    pub(crate) fn close_handler_register(&self) -> TokenStream {
        quote!(
            if let Some(ch) = cast_mut!(self as CloseHandler) {
                CloseHandlerMgr::register(ch)
            }
        )
    }
}
