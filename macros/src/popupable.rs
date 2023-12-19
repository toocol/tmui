use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::DeriveInput;

pub(crate) struct Popupable {
    name: Ident,
}

impl Popupable {
    pub(crate) fn parse(ast: &DeriveInput) -> syn::Result<Popupable> {
        Ok(Self {
            name: ast.ident.clone(),
        })
    }

    pub(crate) fn popupable_field(&self) -> TokenStream {
        quote!(popup_field: Option<Box<dyn PopupImpl>>)
    }

    pub(crate) fn popupable_reflect(&self) -> TokenStream {
        let name = &self.name;
        quote!(
            type_registry.register::<#name, ReflectPopupable>();
        )
    }

    pub(crate) fn popupable_impl(&self) -> TokenStream {
        let name = &self.name;

        quote!(
            impl Popupable for #name {
                #[inline]
                fn set_popup(&mut self, popup: Box<dyn PopupImpl>) {
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
