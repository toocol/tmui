use quote::quote;
use syn::Ident;

pub(crate) fn generate_stack_add_child() -> syn::Result<proc_macro2::TokenStream> {
    Ok(quote! {
        use tmui::application_window::ApplicationWindow;
        ApplicationWindow::initialize_dynamic_component(self, child.as_mut());
        if self.current_index == self.container.children.len() {
            child.show()
        } else {
            child.hide()
        }
        self.container.children.push(child);
        self.update();
    })
}

pub(crate) fn generate_stack_impl(
    name: &Ident,
    use_prefix: &str,
) -> syn::Result<proc_macro2::TokenStream> {
    let use_prefix = Ident::new(use_prefix, name.span());
    Ok(quote! {
        impl StackTrait for #name {
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