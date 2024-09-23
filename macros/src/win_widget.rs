use crate::SplitGenericsRef;
use proc_macro2::{Ident, TokenStream};
use syn::DeriveInput;
use quote::quote;

pub(crate) struct WinWidget<'a> {
    name: Ident,
    generics: SplitGenericsRef<'a>,
}

impl<'a> WinWidget<'a> {
    pub(crate) fn parse(ast: &DeriveInput, generics: SplitGenericsRef<'a>) -> syn::Result<Self> {
        Ok(Self {
            name: ast.ident.clone(),
            generics,
        })
    }

    pub(crate) fn field(&self) -> TokenStream {
        quote!(win_widget_effect: bool)
    }

    pub(crate) fn relfect_clause(&self) -> TokenStream {
        let name = &self.name;
        let (_, ty_generics, _) = self.generics;

        quote!(
            type_registry.register::<#name #ty_generics, ReflectWinWidget>();
        )
    }

    pub(crate) fn impl_clause(&self) -> TokenStream {
        let name = &self.name;
        let (impl_generics, ty_generics, where_clause) = self.generics;

        quote!(
            impl #impl_generics WinWidget for #name #ty_generics #where_clause {
                #[inline]
                fn child_process_fn(&self) -> Box<dyn Fn(&mut ApplicationWindow) + Send + Sync> {
                    Box::new(|win| {
                        win.child(Object::new::<#name>(&[]))
                    })
                }

                #[inline]
                fn is_win_widget_effect(&self) -> bool {
                    self.win_widget_effect
                }

                #[inline]
                fn set_win_widget_effect(&mut self, effect: bool) {
                    self.win_widget_effect = effect
                }
            }
        )
    }
}
