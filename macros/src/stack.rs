use quote::quote;
use syn::Ident;

pub(crate) fn generate_stack_add_child() -> syn::Result<proc_macro2::TokenStream> {
    Ok(quote! {
        use tmui::application_window::ApplicationWindow;
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

pub(crate) fn generate_stack_impl(
    name: &Ident,
    use_prefix: &Ident,
) -> syn::Result<proc_macro2::TokenStream> {
    Ok(quote! {
        impl StackTrait for #name {
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
                let index = self.current_index;
                self.children_mut().get_mut(index).unwrap().hide();

                self.current_index += 1;
                if self.current_index == self.container.children.len() {
                    self.current_index = 0;
                }

                let index = self.current_index;
                self.children_mut().get_mut(index).unwrap().show();

                ApplicationWindow::window_of(self.window_id()).layout_change(self);
                self.update()
            }

            #[inline]
            fn switch_index(&mut self, index: usize) {
                use #use_prefix::application_window::ApplicationWindow;
                if index >= self.container.children.len() {
                    log::warn!("`index` overrange, skip the `switch_index()`, max {}, get {}", self.container.children.len() - 1, index);
                    return
                }
                let old_index = self.current_index;
                self.children_mut().get_mut(old_index).unwrap().hide();

                self.current_index = index;

                self.children_mut().get_mut(index).unwrap().show();

                ApplicationWindow::window_of(self.window_id()).layout_change(self);
                self.update()
            }
        }
    })
}
