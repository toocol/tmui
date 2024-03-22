use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::DeriveInput;

use crate::SplitGenericsRef;

pub(crate) struct Loadable<'a> {
    name: Ident,
    generics: SplitGenericsRef<'a>,
}

impl<'a> Loadable<'a> {
    pub(crate) fn parse(ast: &DeriveInput, generics: SplitGenericsRef<'a>) -> syn::Result<Self> {
        Ok(Self {
            name: ast.ident.clone(),
            generics,
        })
    }

    pub(crate) fn loadable_field(&self) -> TokenStream {
        quote!(loading_model: LoadingModel)
    }

    pub(crate) fn loadable_reflect(&self) -> TokenStream {
        let name = &self.name;
        let (_, ty_generics, _) = self.generics;

        quote!(
            type_registry.register::<#name #ty_generics, ReflectLoadable>();
        )
    }

    pub(crate) fn loadable_impl(&self) -> TokenStream {
        let name = &self.name;
        let (impl_generics, ty_generics, where_clause) = self.generics;

        quote!(
            impl #impl_generics Loadable for #name #ty_generics #where_clause {
                fn loading_model(&self) -> &LoadingModel {
                    &self.loading_model
                }

                fn loading_model_mut(&mut self) -> &mut LoadingModel {
                    &mut self.loading_model
                }
            }
        )
    }
}
