use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse::Parse, punctuated::Punctuated, Ident, Token};

use crate::SplitGenericsRef;

pub(crate) struct GlobalWatch<'a> {
    events: Vec<Ident>,
    generics: Option<SplitGenericsRef<'a>>,
}

impl Parse for GlobalWatch<'_> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let events: Punctuated<Ident, Token![,]> = input.parse_terminated(Ident::parse)?;

        Ok(GlobalWatch {
            events: events.into_iter().collect(),
            generics: None,
        })
    }
}

impl<'a> GlobalWatch<'a> {
    pub(crate) fn set_generics(&mut self, generics: SplitGenericsRef<'a>) {
        self.generics = Some(generics)
    }

    pub(crate) fn expand_impl(&self, name: &Ident) -> syn::Result<TokenStream> {
        let events = &self.events;
        let (impl_generics, ty_generics, where_clause) = self.generics.unwrap();

        Ok(quote!(
            impl #impl_generics GlobalWatch for #name #ty_generics #where_clause {
                #[inline]
                fn watch_list(&self) -> Vec<GlobalWatchEvent> {
                    vec![
                        #(GlobalWatchEvent::#events),*
                    ]
                }

                #[inline]
                fn as_widget(&mut self) -> &mut dyn WidgetImpl {
                    self
                }
            }
        ))
    }

    pub(crate) fn expand_reflect(&self, name: &Ident) -> syn::Result<TokenStream> {
        let (_, ty_generics, _) = self.generics.unwrap();

        Ok(quote!(
            type_registry.register::<#name #ty_generics, ReflectGlobalWatch>();
        ))
    }
}
