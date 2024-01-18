use proc_macro2::Ident;
use quote::quote;

pub(crate) fn generate_pane_add_child() -> syn::Result<proc_macro2::TokenStream> {
    Ok(quote!(
        use tmui::application_window::ApplicationWindow;

        if self.container.children.len() >= 2 {
            log::error!("`Pane` can only have two child component.");
            return;
        }

        child.set_parent(self);
        ApplicationWindow::initialize_dynamic_component(child.as_mut());
        self.container.children.push(child);
        self.update();
    ))
}

pub(crate) fn generate_pane_type_register(name: &Ident) -> syn::Result<proc_macro2::TokenStream> {
    Ok(quote!(
        type_registry.register::<#name, ReflectSizeUnifiedAdjust>();
        type_registry.register::<#name, ReflectPaneExt>();
        type_registry.register::<#name, ReflectInnerCustomizeEventProcess>();
    ))
}

pub(crate) fn generate_pane_inner_init() -> syn::Result<proc_macro2::TokenStream> {
    Ok(quote!(
        self.set_mouse_tracking(true);
        self.set_passing_mouse_tracking(true);

        self.enable_bubble(EventBubble::MOUSE_MOVE);
        self.enable_bubble(EventBubble::MOUSE_PRESSED);
        self.enable_bubble(EventBubble::MOUSE_RELEASED);
        self.set_passing_event_bubble(true);
    ))
}

pub(crate) fn generate_pane_impl(name: &Ident) -> syn::Result<proc_macro2::TokenStream> {
    Ok(quote!(
        impl PaneExt for #name {
            #[inline]
            fn direction(&self) -> PaneDirection {
                self.direction
            }

            #[inline]
            fn set_direction(&mut self, direction: PaneDirection) {
                self.direction = direction;

                if self.window().initialized() {
                    self.window().layout_change(self)
                }
            }

            #[inline]
            fn is_resize_zone(&self) -> bool {
                self.resize_zone
            }

            #[inline]
            fn set_resize_zone(&mut self, resize_zone: bool) {
                self.resize_zone = resize_zone;
            }

            #[inline]
            fn is_resize_pressed(&self) -> bool {
                self.resize_pressed
            }

            #[inline]
            fn set_resize_pressed(&mut self, resize_pressed: bool) {
                self.resize_pressed = resize_pressed
            }
        }
    ))
}