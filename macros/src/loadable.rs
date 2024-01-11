use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::DeriveInput;

pub(crate) struct Loadable {
    name: Ident,
}

impl Loadable {
    pub(crate) fn parse(ast: &DeriveInput) -> syn::Result<Loadable> {
        Ok(Self {
            name: ast.ident.clone()
        })
    }

    pub(crate) fn loadable_field(&self) -> TokenStream {
        quote!(loading_model: LoadingModel)
    }

    pub(crate) fn loadable_reflect(&self) -> TokenStream {
        let name = &self.name;

        quote!(
            type_registry.register::<#name, ReflectLoadable>();
        )
    }

    pub(crate) fn loadable_impl(&self) -> TokenStream {
        let name = &self.name;

        quote!(
            impl Loadable for #name {
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