use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Attribute, DeriveInput, Meta, MetaNameValue};

pub(crate) struct Popupable {
    name: Ident,
    internal: bool,
}

impl Popupable {
    pub(crate) fn parse(attr: &Attribute, ast: &DeriveInput) -> syn::Result<Popupable> {
        let mut internal = false;

        if let Ok(meta) = attr.parse_meta() {
            match meta {
                Meta::NameValue(MetaNameValue { path, lit, .. }) => {
                    let ident = path.get_ident().unwrap();

                    match ident.to_string().as_str() {
                        "internal" => match lit {
                            syn::Lit::Bool(lit) => {
                                internal = lit.value();
                            }
                            _ => {
                                return Err(syn::Error::new_spanned(
                                    attr,
                                    "pupupable wrong attribute format.",
                                ))
                            }
                        },
                        _ => {
                            return Err(syn::Error::new_spanned(
                                attr,
                                "pupupable wrong attribute format.",
                            ))
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(Self {
            name: ast.ident.clone(),
            internal,
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
        let prefix = if self.internal { "crate" } else { "tmui" };
        let prefix = Ident::new(prefix, self.name.span());

        quote!(
            impl Popupable for #name {
                #[inline]
                fn add_popup(&mut self, mut popup: Box<dyn PopupImpl>) {
                    use #prefix::application_window::ApplicationWindow;

                    popup.set_parent(self);

                    ApplicationWindow::initialize_dynamic_component(popup.as_widget_impl_mut());

                    self.popup_field = Some(popup)
                }

                #[inline]
                fn show_popup(&mut self, basic_point: Point) {
                    let rect = self.rect();
                    if let Some(ref mut popup) = self.popup_field {
                        if popup.visible() {
                            return;
                        }

                        use #prefix::application_window::ApplicationWindow;

                        let pos = popup.calculate_position(rect, basic_point);
                        popup.set_fixed_x(pos.x());
                        popup.set_fixed_y(pos.y());
                        ApplicationWindow::window_of(popup.window_id()).layout_change(popup.as_widget_impl_mut());
                        popup.show();
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
}
