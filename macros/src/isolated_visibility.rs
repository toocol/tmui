use crate::SplitGenericsRef;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::DeriveInput;

pub(crate) struct IsolatedVisibility<'a> {
    name: Ident,
    generics: SplitGenericsRef<'a>,
}

impl<'a> IsolatedVisibility<'a> {
    pub(crate) fn parse(ast: &DeriveInput, generics: SplitGenericsRef<'a>) -> syn::Result<Self> {
        Ok(Self {
            name: ast.ident.clone(),
            generics,
        })
    }

    pub(crate) fn isolated_visibility_field(&self) -> Vec<TokenStream> {
        vec![
            quote!(auto_hide: bool),
            quote!(shadow_rect: FRect),
        ]
    }

    pub(crate) fn isolated_visibility_reflect(&self) -> TokenStream {
        let name = &self.name;
        let (_, ty_generics, _) = self.generics;

        quote!(
            type_registry.register::<#name #ty_generics, ReflectIsolatedVisibility>();
        )
    }

    pub(crate) fn isolated_visibility_impl(&self) -> TokenStream {
        let name = &self.name;
        let (impl_generics, ty_generics, where_clause) = self.generics;

        quote!(
            impl #impl_generics IsolatedVisibility for #name #ty_generics #where_clause {
                #[inline]
                fn auto_hide(&self) -> bool {
                    self.auto_hide
                }

                #[inline]
                fn set_auto_hide(&mut self, auto_hide: bool) {
                    self.auto_hide = auto_hide;
                }

                #[inline]
                fn shadow_rect(&self) -> FRect {
                    self.shadow_rect
                }

                #[inline]
                fn set_shadow_rect(&mut self, rect: FRect) {
                    self.shadow_rect = rect
                }
            }
        )
    }
}
