use quote::quote;
use syn::Ident;

pub(crate) fn generate_split_pane_add_child() -> syn::Result<proc_macro2::TokenStream> {
    Ok(quote! {
        if self.container.children.len() != 0 {
            panic!("Only first widget can use function `add_child()` to add, please use `split_left()`,`split_top()`,`split_right()` or `split_down()`")
        }
        child.set_parent(self);
        let widget_ptr: std::option::Option<std::ptr::NonNull<dyn WidgetImpl>> = std::ptr::NonNull::new(child.bind_mut());
        let mut split_info = Box::new(SplitInfo::new(
            child.id(),
            widget_ptr.clone(),
            None,
            SplitType::SplitNone,
        ));
        self.split_infos_vec
            .push(std::ptr::NonNull::new(split_info.as_mut()));
        self.split_infos.insert(child.id(), split_info);
        self.container.children.push(child.clone().into());
        ApplicationWindow::initialize_dynamic_component(child.as_dyn_mut(), self.is_in_tree());
        self.update();
    })
}

pub(crate) fn generate_split_pane_remove_children() -> syn::Result<proc_macro2::TokenStream> {
    Ok(quote! {
        if let Some(index) = self.container.children.iter().position(|w| w.id() == id) {
            let removed = self.container.children.remove(index);
            self.close_pane(removed.id());

            let window = ApplicationWindow::window();
            window._add_removed_widget(removed);
            window.layout_change(self);
        }
    })
}

pub(crate) fn generate_split_pane_impl(name: &Ident) -> syn::Result<proc_macro2::TokenStream> {
    Ok(quote! {
    impl SizeUnifiedAdjust for #name {
        fn size_unified_adjust(&mut self) {
            let parent_rect = self.contents_rect(None);
            let split_infos_getter = cast_mut!(self as SplitInfosGetter).unwrap();
            for split_info in split_infos_getter.split_infos_vec() {
                nonnull_mut!(split_info).calculate_layout(parent_rect, false)
            }
            for split_info in split_infos_getter.split_infos_vec() {
                let split_info = nonnull_mut!(split_info);
                let widget = split_widget!(split_info);
                emit!(#name::size_unified_adjust => widget, size_changed(widget.size()));
            }
        }
    }

    impl SplitInfosGetter for #name {
        fn split_infos(&mut self) -> &mut nohash_hasher::IntMap<ObjectId, Box<SplitInfo>> {
            &mut self.split_infos
        }

        fn split_infos_vec(&mut self) -> &mut Vec<std::option::Option<std::ptr::NonNull<SplitInfo>>> {
            &mut self.split_infos_vec
        }
    }

    impl SplitPaneExt for #name {
        #[inline]
        fn split_left<T: WidgetImpl>(&mut self, id: ObjectId, widget: Tr<T>) {
            self.split(id, widget, SplitType::SplitLeft)
        }

        #[inline]
        fn split_up<T: WidgetImpl>(&mut self, id: ObjectId, widget: Tr<T>) {
            self.split(id, widget, SplitType::SplitUp)
        }

        #[inline]
        fn split_right<T: WidgetImpl>(&mut self, id: ObjectId, widget: Tr<T>) {
            self.split(id, widget, SplitType::SplitRight)
        }

        #[inline]
        fn split_down<T: WidgetImpl>(&mut self, id: ObjectId, widget: Tr<T>) {
            self.split(id, widget, SplitType::SplitDown)
        }

        fn close_pane(&mut self, id: ObjectId) {
            let mut split = match self.split_infos.remove(&id) {
                Some(s) => s,
                None => return,
            };

            let mut split_from = if split.split_from.is_some() {
                Some(nonnull_mut!(split.split_from))
            } else {
                None
            };
            if let Some(split_from) = split_from.as_mut() {
                split_from.split_to.retain(|s| nonnull_ref!(s).id != id);
            }

            for split_to_ptr in split.split_to.iter_mut() {
                let split_to = nonnull_mut!(split_to_ptr);
                split_to.split_from = split.split_from;
                split_to.ty = split.ty;

                if let Some(split_from) = split_from.as_mut() {
                    split_from.split_to.push(split_to_ptr.clone());
                }
            }
            self.split_infos_vec.retain(|s| nonnull_ref!(s).id != id);
        }

        fn split<T: WidgetImpl>(&mut self, id: ObjectId, mut widget: Tr<T>, ty: SplitType) {
            use std::ptr::NonNull;

            let mut split_from = if let Some(split_info) = self.split_infos.get_mut(&id) {
                NonNull::new(split_info.as_mut())
            } else {
                panic!("The widget with id {} is not exist in SplitPane.", id)
            };

            widget.set_parent(self);
            let mut split_info = Box::new(SplitInfo::new(
                widget.id(),
                NonNull::new(widget.as_dyn_mut()),
                split_from,
                ty,
            ));

            nonnull_mut!(split_from).split_to.push(NonNull::new(split_info.as_mut()));
            self.split_infos_vec
                .push(NonNull::new(split_info.as_mut()));
            self.split_infos.insert(widget.id(), split_info);
            self.container.children.push(widget.clone().into());

            // Tell the `ApplicationWindow` that widget's layout has changed:
            if self.window_id() == 0 {
                panic!("`split()` in SplitPane should invoke after window initialize.")
            }
            ApplicationWindow::initialize_dynamic_component(widget.as_dyn_mut(), self.is_in_tree());
            self.update()
        }
    }
    })
}
