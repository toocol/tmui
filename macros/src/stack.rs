use quote::quote;
use syn::Ident;

pub(crate) fn generate_stack_add_child(
    use_prefix: &Ident,
) -> syn::Result<proc_macro2::TokenStream> {
    Ok(quote! {
        use #use_prefix::application_window::ApplicationWindow;
        child.set_parent(self);
        if self.current_index == self.container.children.len() {
            child.show()
        } else {
            child.hide()
        }
        ApplicationWindow::initialize_dynamic_component(child.as_mut());
        self.container.children.push(child);
        self.update();
    })
}

pub(crate) fn generate_stack_inner_initial() -> syn::Result<proc_macro2::TokenStream> {
    Ok(quote!{
        let idx = self.current_index();
        for (i, c) in self.children_mut().iter_mut().enumerate() {
            if i != idx {
                c.set_property("visible", false.to_value());
            }
        }
    })
}

pub(crate) fn generate_stack_inner_on_property_set(use_prefix: &Ident) -> syn::Result<proc_macro2::TokenStream> {
    Ok(quote!{
        use #use_prefix::widget::InnerEventProcess;
        match name {
            "visible" => {
                let visible = value.get::<bool>();
                emit!(self.visibility_changed(), visible);
                self.inner_visibility_changed(visible);
                self.on_visibility_changed(visible);
                if visible {
                    if let Some(c) = self.current_child_mut() {
                        if !c.visibility_check() {
                            return true;
                        }
                        if let Some(iv) = cast!(c as IsolatedVisibility) {
                            if iv.auto_hide() {
                                return true;
                            }
                        }

                        c.set_property("visible", true.to_value());
                        c.set_render_styles(true);
                    }
                } else {
                    for c in self.children_mut() {
                        c.set_property("visible", false.to_value());
                    }
                }
                true
            }
            _ => false,
        }
    })
}

pub(crate) fn generate_stack_impl(
    name: &Ident,
    use_prefix: &Ident,
) -> syn::Result<proc_macro2::TokenStream> {
    Ok(quote! {
        impl StackImpl for #name {
            fn current_child(&self) -> Option<&dyn WidgetImpl> {
                if self.children().len() < self.current_index() + 1 {
                    None
                } else {
                    Some(self.children().remove(self.current_index()))
                }
            }

            fn current_child_mut(&mut self) -> Option<&mut dyn WidgetImpl> {
                if self.children().len() < self.current_index() + 1 {
                    None
                } else {
                    let idx = self.current_index();
                    Some(self.children_mut().remove(idx))
                }
            }

            #[inline]
            fn current_index(&self) -> usize {
                self.current_index
            }

            #[inline]
            fn switch(&mut self) {
                use #use_prefix::application_window::ApplicationWindow;
                let last_index = self.current_index;

                self.current_index += 1;
                if self.current_index == self.container.children.len() {
                    self.current_index = 0;
                }

                if !self.visible() {
                    return;
                }

                self.children_mut().get_mut(last_index).unwrap().hide();

                let index = self.current_index;
                self.children_mut().get_mut(index).unwrap().show();

                self.update();
                self.set_render_styles(true);
            }

            #[inline]
            fn switch_index(&mut self, index: usize) {
                use #use_prefix::application_window::ApplicationWindow;
                if index >= self.children().len() {
                    log::warn!("`index` overrange, skip the `switch_index()`, children len {}, get index {}", self.children().len(), index);
                    return
                }

                if !self.visible() {
                    self.current_index = index;
                    return
                }

                let old_index = self.current_index;
                self.children_mut().get_mut(old_index).unwrap().hide();

                self.current_index = index;

                self.children_mut().get_mut(index).unwrap().show();

                self.update();
                self.set_render_styles(true);
            }
        }
    })
}
