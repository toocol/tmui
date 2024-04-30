use proc_macro2::Ident;
use quote::quote;

pub(crate) fn generate_scroll_area_add_child(name: &Ident) -> syn::Result<proc_macro2::TokenStream> {
    Ok(quote!(panic!(
        "Please use `set_area()` instead in `{}`", stringify!(#name)
    )))
}

pub(crate) fn generate_scroll_area_get_children() -> syn::Result<proc_macro2::TokenStream> {
    Ok(quote!(
        fn children(&self) -> Vec<&dyn WidgetImpl> {
            let mut children: Vec<&dyn WidgetImpl> = vec![self.scroll_bar.as_ref()];
            if self.area.is_some() {
                children.push(self.area.as_ref().unwrap().as_ref())
            }
            children
        }

        fn children_mut(&mut self) -> Vec<&mut dyn WidgetImpl> {
            let mut children: Vec<&mut dyn WidgetImpl> = vec![self.scroll_bar.as_mut()];
            if self.area.is_some() {
                children.push(self.area.as_mut().unwrap().as_mut())
            }
            children
        }
    ))
}

pub(crate) fn generate_scroll_area_inner_init() -> syn::Result<proc_macro2::TokenStream> {
    Ok(quote!(
        self.set_rerender_difference(true);
        self.set_vexpand(true);
        self.set_hexpand(true);

        self.scroll_bar.set_vexpand(true);
        self.scroll_bar.set_hscale(10.);

        let parent = self as *mut dyn WidgetImpl;
        self.scroll_bar.set_parent(parent);

        use tlib::connect;
        connect!(self, size_changed(), self, adjust_area_layout(Size));
    ))
}

pub(crate) fn generate_scroll_area_impl(
    name: &Ident,
    use_prefix: &Ident,
) -> syn::Result<proc_macro2::TokenStream> {
    Ok(quote!(
        use #use_prefix::scroll_bar::{ScrollBar, ScrollBarPosition};
        use #use_prefix::scroll_area::ScrollArea;
        use tlib::{
            events::{DeltaType, MouseEvent},
            namespace::{KeyboardModifier, Orientation},
        };
        use log::debug;

        impl ScrollAreaExt for #name {
            #[inline]
            fn get_area(&self) -> Option<&dyn WidgetImpl> {
                self.area.as_ref().and_then(|w| Some(w.as_ref()))
            }

            #[inline]
            fn get_area_mut(&mut self) -> Option<&mut dyn WidgetImpl> {
                self.area.as_mut().and_then(|w| Some(w.as_mut()))
            }

            #[inline]
            fn get_scroll_bar(&self) -> &ScrollBar {
                &self.scroll_bar
            }

            #[inline]
            fn get_scroll_bar_mut(&mut self) -> &mut ScrollBar {
                &mut self.scroll_bar
            }

            #[inline]
            fn set_scroll_bar_position(&mut self, scroll_bar_position: ScrollBarPosition) {
                self.scroll_bar.set_scroll_bar_position(scroll_bar_position);
                self.update();
            }

            #[inline]
            fn set_orientation(&mut self, orientation: Orientation) {
                self.scroll_bar.set_orientation(orientation);
                self.update();
            }

            /// Scroll the widget. </br>
            /// delta was positive value when scroll down/right.
            #[inline]
            fn scroll(&mut self, delta: i32, delta_type: DeltaType) {
                self.scroll_bar
                    .scroll_by_delta(KeyboardModifier::NoModifier, delta, delta_type);
            }

            #[inline]
            fn adjust_area_layout(&mut self, size: Size) {
                if size.width() == 0 || size.height() == 0 {
                    debug!("The size of `{}` was not specified, skip adjust_area_layout()", stringify!(#name));
                    return;
                }

                if let Some(area) = self.get_area_mut() {
                    area.set_vexpand(true);
                    area.set_hexpand(true);
                    area.set_hscale(size.width() as f32 - 10.);
                }
            }

            #[inline]
            fn layout_mode(&self) -> #use_prefix::scroll_area::LayoutMode {
                self.layout_mode
            }
        }

        impl ScrollAreaGenericExt for #name {
            #[inline]
            fn set_area<T: WidgetImpl>(&mut self, mut area: Box<T>) {
                use #use_prefix::application_window::ApplicationWindow;

                area.set_parent(self);
                area.set_vexpand(true);
                area.set_hexpand(true);

                ApplicationWindow::initialize_dynamic_component(area.as_mut());
                self.area = Some(area);

                self.adjust_area_layout(self.size());
            }

            #[inline]
            fn get_area_cast<T: WidgetImpl + ObjectSubclass>(&self) -> Option<&T> {
                self.area.as_ref().and_then(|w| w.downcast_ref::<T>())
            }

            #[inline]
            fn get_area_cast_mut<T: WidgetImpl + ObjectSubclass>(&mut self) -> Option<&mut T> {
                self.area.as_mut().and_then(|w| w.downcast_mut::<T>())
            }
        }
    ))
}
