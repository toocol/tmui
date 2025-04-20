use proc_macro2::Ident;
use quote::quote;

pub(crate) fn generate_scroll_area_add_child(
    name: &Ident,
) -> syn::Result<proc_macro2::TokenStream> {
    Ok(quote!(panic!(
        "Please use `set_area()` instead in `{}`",
        stringify!(#name)
    )))
}

pub(crate) fn generate_scroll_area_remove_children() -> syn::Result<proc_macro2::TokenStream> {
    Ok(quote!(
        // TODO
    ))
}

pub(crate) fn generate_scroll_area_get_children() -> syn::Result<proc_macro2::TokenStream> {
    Ok(quote!(
        #[inline]
        fn children(&self) -> Vec<&dyn WidgetImpl> {
            self.container.children.iter().map(|w| w.bind()).collect()
        }

        #[inline]
        fn children_mut(&mut self) -> Vec<&mut dyn WidgetImpl> {
            self.container
                .children
                .iter_mut()
                .map(|w| w.bind_mut())
                .collect()
        }
    ))
}

pub(crate) fn generate_scroll_area_pre_construct() -> syn::Result<proc_macro2::TokenStream> {
    Ok(quote!(
        self.set_render_difference(true);
        self.container
            .children
            .push(ScrollBar::new(Orientation::Vertical).into());

        let use_occupy = self.layout_mode == LayoutMode::Normal;
        self.scroll_bar_mut()
            .set_occupy_space(use_occupy);

        let parent = self as *mut dyn WidgetImpl;
        self.scroll_bar_mut().set_parent(parent);
    ))
}

pub(crate) fn generate_scroll_area_impl(name: &Ident) -> syn::Result<proc_macro2::TokenStream> {
    Ok(quote!(
        use log::debug;

        impl ScrollAreaExt for #name {
            #[inline]
            fn area(&self) -> Option<&dyn WidgetImpl> {
                if self.container.children.len() == 1 {
                    return None;
                }
                self.container.children.first().map(|w| w.bind())
            }

            #[inline]
            fn area_mut(&mut self) -> Option<&mut dyn WidgetImpl> {
                if self.container.children.len() == 1 {
                    return None;
                }
                self.container.children.first_mut().map(|w| w.bind_mut())
            }

            #[inline]
            fn scroll_bar(&self) -> &ScrollBar {
                self.container
                    .children
                    .last()
                    .map(|w| w.downcast_ref::<ScrollBar>().unwrap())
                    .unwrap()
            }

            #[inline]
            fn scroll_bar_mut(&mut self) -> &mut ScrollBar {
                self.container
                    .children
                    .last_mut()
                    .map(|w| w.downcast_mut::<ScrollBar>().unwrap())
                    .unwrap()
            }

            #[inline]
            fn set_scroll_bar_position(&mut self, scroll_bar_position: ScrollBarPosition) {
                self.scroll_bar_mut().set_scroll_bar_position(scroll_bar_position);
                self.update();
            }

            #[inline]
            fn set_orientation(&mut self, orientation: Orientation) {
                self.scroll_bar_mut().set_orientation(orientation);
                self.update();
            }

            /// Scroll the widget. </br>
            /// delta was positive value when scroll down/right.
            #[inline]
            fn scroll(&mut self, delta: i32, delta_type: DeltaType) {
                self.scroll_bar_mut()
                    .scroll_by_delta(KeyboardModifier::NoModifier, delta, delta_type);
            }

            #[inline]
            fn layout_mode(&self) -> LayoutMode {
                self.layout_mode
            }

            #[inline]
            fn set_layout_mode(&mut self, layout_mode: LayoutMode) {
                self.layout_mode = layout_mode;
                self.scroll_bar_mut().set_occupy_space(layout_mode == LayoutMode::Normal);
                self.scroll_bar_mut().set_overlaid(layout_mode == LayoutMode::Overlay);

                if self.area().is_some() {
                    if layout_mode == LayoutMode::Normal {
                        disconnect!(self.area_mut().unwrap(), invalidated(), self, null);
                        disconnect!(self.scroll_bar_mut(), geometry_changed(), self.area_mut().unwrap(), null);
                        disconnect!(self.scroll_bar_mut(), need_update(), self.area_mut().unwrap(), null);
                        self.area_mut().unwrap().set_invalid_area(FRect::default());
                    } else {
                        connect!(self.area_mut().unwrap(), invalidated(), self, update_scroll_bar());
                        connect!(self.scroll_bar_mut(), geometry_changed(), self.area_mut().unwrap(), set_invalid_area(FRect));
                        connect!(self.scroll_bar_mut(), need_update(), self.area_mut().unwrap(), update());
                    }
                }

                self.window().layout_change(self.scroll_bar_mut())
            }
        }

        impl ScrollAreaGenericExt for #name {
            #[inline]
            fn set_area<T: WidgetImpl>(&mut self, mut area: Tr<T>) {
                area.set_parent(self);
                area.set_vexpand(true);
                area.set_hexpand(true);
                if self.layout_mode == LayoutMode::Overlay {
                    connect!(area, invalidated(), self, update_scroll_bar());
                    connect!(self.scroll_bar_mut(), geometry_changed(), area, set_invalid_area(FRect));
                    connect!(self.scroll_bar_mut(), need_update(), self.area_mut().unwrap(), update());
                }

                self.container.children.insert(0, area.clone().into());
                ApplicationWindow::initialize_dynamic_component(area.as_dyn_mut(), self.is_in_tree());
            }

            #[inline]
            fn get_area_cast<T: WidgetImpl + ObjectSubclass>(&self) -> Option<&T> {
                self.area().and_then(|w| w.downcast_ref::<T>())
            }

            #[inline]
            fn get_area_cast_mut<T: WidgetImpl + ObjectSubclass>(&mut self) -> Option<&mut T> {
                self.area_mut().and_then(|w| w.downcast_mut::<T>())
            }
        }
    ))
}
