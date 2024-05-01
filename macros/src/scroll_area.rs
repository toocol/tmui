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
            let mut children: Vec<&dyn WidgetImpl> = vec![];
            if self.area.is_some() {
                children.push(self.area.as_ref().unwrap().as_ref())
            }
            children.push(self.scroll_bar.as_ref());
            children
        }

        fn children_mut(&mut self) -> Vec<&mut dyn WidgetImpl> {
            let mut children: Vec<&mut dyn WidgetImpl> = vec![];
            if self.area.is_some() {
                children.push(self.area.as_mut().unwrap().as_mut())
            }
            children.push(self.scroll_bar.as_mut());
            children
        }
    ))
}

pub(crate) fn generate_scroll_area_inner_init(use_prefix: &Ident) -> syn::Result<proc_macro2::TokenStream> {
    Ok(quote!(
        self.set_rerender_difference(true);

        self.scroll_bar
            .set_occupy_space(self.layout_mode == #use_prefix::scroll_area::LayoutMode::Normal);

        let parent = self as *mut dyn WidgetImpl;
        self.scroll_bar.set_parent(parent);
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
            fn layout_mode(&self) -> #use_prefix::scroll_area::LayoutMode {
                self.layout_mode
            }

            #[inline]
            fn set_layout_mode(&mut self, layout_mode: #use_prefix::scroll_area::LayoutMode) {
                self.layout_mode = layout_mode;
                self.scroll_bar.set_occupy_space(layout_mode == #use_prefix::scroll_area::LayoutMode::Normal);

                if self.area.is_some() {
                    if layout_mode == #use_prefix::scroll_area::LayoutMode::Normal {
                        #use_prefix::tlib::disconnect!(self.get_area_mut().unwrap(), invalidated(), self.scroll_bar, null);
                    } else {
                        #use_prefix::tlib::connect!(self.get_area_mut().unwrap(), invalidated(), self.scroll_bar, update());
                    }
                }

                if self.window().initialized() {
                    self.window().layout_change(self.scroll_bar.as_mut())
                }
            }
        }

        impl ScrollAreaGenericExt for #name {
            #[inline]
            fn set_area<T: WidgetImpl>(&mut self, mut area: Box<T>) {
                use #use_prefix::application_window::ApplicationWindow;

                area.set_parent(self);
                area.set_vexpand(true);
                area.set_hexpand(true);
                if self.layout_mode == #use_prefix::scroll_area::LayoutMode::Overlay {
                    #use_prefix::tlib::connect!(area, invalidated(), self.scroll_bar, update());
                    area.set_overlaid_rect(self.scroll_bar.rect());
                }

                ApplicationWindow::initialize_dynamic_component(area.as_mut());
                self.area = Some(area);
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
