use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub(crate) fn generate_popable_impl(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;

    quote!(
        impl Popupable for #name {
            #[inline]
            fn add_popup(&mut self, mut popup: Box<dyn PopupImpl>) {
                popup.set_parent(self);

                ApplicationWindow::initialize_dynamic_component(popup.as_widget_impl_mut());

                self.popup_field = Some(popup)
            }

            #[inline]
            fn show_popup(&mut self) {
                if let Some(ref mut popup) = self.popup_field {
                    popup.show()
                }
            }

            #[inline]
            fn hide_popup(&mut self) {
                if let Some(ref mut popup) = self.popup_field {
                    popup.hide()
                }
            }

            #[inline]
            fn get_popup_ref(&self) -> Option<&dyn PopupImpl> {
                self.popup_field.as_ref().and_then(|p| {Some(p.as_ref())})
            }

            #[inline]
            fn get_popup_mut(&mut self) -> Option<&mut dyn PopupImpl> {
                self.popup_field.as_mut().and_then(|p| {Some(p.as_mut())})
            }
        }
    )
}
