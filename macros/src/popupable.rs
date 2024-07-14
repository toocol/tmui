use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::DeriveInput;

use crate::SplitGenericsRef;

pub(crate) struct Popupable<'a> {
    name: Ident,
    generics: SplitGenericsRef<'a>,
}

impl<'a> Popupable<'a> {
    pub(crate) fn parse(
        ast: &DeriveInput,
        generics: SplitGenericsRef<'a>,
    ) -> syn::Result<Self> {
        Ok(Self {
            name: ast.ident.clone(),
            generics,
        })
    }

    pub(crate) fn popupable_field(&self) -> TokenStream {
        quote!(popup_field: Option<Box<dyn PopupImpl>>)
    }

    pub(crate) fn popupable_reflect(&self) -> TokenStream {
        let name = &self.name;
        let (_, ty_generics, _) = self.generics;
        quote!(
            type_registry.register::<#name #ty_generics, ReflectPopupable>();
        )
    }

    pub(crate) fn popupable_impl(&self) -> TokenStream {
        let name = &self.name;
        let (impl_generics, ty_generics, where_clause) = self.generics;

        quote!(
            impl #impl_generics Popupable for #name #ty_generics #where_clause {
                #[inline]
                fn set_popup(&mut self, mut popup: Box<dyn PopupImpl>) {
                    popup.set_supervisor(self);
                    self.popup_field = Some(popup)
                }

                #[inline]
                fn get_popup_ref(&self) -> Option<&dyn PopupImpl> {
                    self.popup_field.as_ref().and_then(|p| {Some(p.as_ref())})
                }

                #[inline]
                fn get_popup_mut(&mut self) -> Option<&mut dyn PopupImpl> {
                    self.popup_field.as_mut().and_then(|p| {Some(p.as_mut())})
                }

                #[inline]
                fn as_widget(&self) -> &dyn WidgetImpl {
                    self
                }

                #[inline]
                fn as_widget_mut(&mut self) -> &mut dyn WidgetImpl {
                    self
                }
            }
        )
    }
}
