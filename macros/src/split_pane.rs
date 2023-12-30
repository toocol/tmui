use quote::quote;
use syn::Ident;

pub(crate) fn generate_split_pane_add_child() -> syn::Result<proc_macro2::TokenStream> {
    Ok(quote! {
        use tmui::application_window::ApplicationWindow;
        if self.container.children.len() != 0 {
            panic!("Only first widget can use function `add_child()` to add, please use `split_left()`,`split_top()`,`split_right()` or `split_down()`")
        }
        child.set_parent(self);
        ApplicationWindow::initialize_dynamic_component(child.as_mut());
        let widget_ptr: std::option::Option<std::ptr::NonNull<dyn WidgetImpl>> = std::ptr::NonNull::new(child.as_mut());
        let mut split_info = Box::new(SplitInfo::new(
            child.id(),
            widget_ptr.clone(),
            None,
            SplitType::SplitNone,
        ));
        self.split_infos_vec
            .push(std::ptr::NonNull::new(split_info.as_mut()));
        self.split_infos.insert(child.id(), split_info);
        self.container.children.push(child);
        self.update();
    })
}

pub(crate) fn generate_split_pane_impl(name: &Ident, use_prefix: &str) -> syn::Result<proc_macro2::TokenStream> {
    let use_prefix = Ident::new(use_prefix, name.span());
    Ok(quote! {
        impl SizeUnifiedAdjust for #name {
            fn size_unified_adjust(&mut self) {
                use #use_prefix::tlib::nonnull_mut;
                use #use_prefix::split_widget;
                let parent_rect = self.contents_rect(None);
                let split_infos_getter = cast_mut!(self as SplitInfosGetter).unwrap();
                for split_info in split_infos_getter.split_infos_vec() {
                    nonnull_mut!(split_info).calculate_layout(parent_rect, false)
                }
                for split_info in split_infos_getter.split_infos_vec() {
                    let split_info = nonnull_mut!(split_info);
                    let widget = split_widget!(split_info);
                    emit!(#name::size_unified_adjust => widget.size_changed(), widget.size());
                }
            }
        }

        impl SplitInfosGetter for #name {
            fn split_infos(&mut self) -> &mut std::collections::HashMap<ObjectId, Box<SplitInfo>> {
                &mut self.split_infos
            }

            fn split_infos_vec(&mut self) -> &mut Vec<std::option::Option<std::ptr::NonNull<SplitInfo>>> {
                &mut self.split_infos_vec
            }
        }

        impl SplitPaneExt for #name {
            #[inline]
            fn split_left<T: WidgetImpl>(&mut self, id: ObjectId, widget: Box<T>) {
                self.split(id, widget, SplitType::SplitLeft)
            }

            #[inline]
            fn split_up<T: WidgetImpl>(&mut self, id: ObjectId, widget: Box<T>) {
                self.split(id, widget, SplitType::SplitUp)
            }

            #[inline]
            fn split_right<T: WidgetImpl>(&mut self, id: ObjectId, widget: Box<T>) {
                self.split(id, widget, SplitType::SplitRight)
            }

            #[inline]
            fn split_down<T: WidgetImpl>(&mut self, id: ObjectId, widget: Box<T>) {
                self.split(id, widget, SplitType::SplitDown)
            }

            fn close_pane(&mut self, id: ObjectId) {
                use #use_prefix::application_window::ApplicationWindow;
                use #use_prefix::{split_widget, split_from};
                use #use_prefix::tlib::nonnull_mut;
                use std::ptr::NonNull;
                use std::collections::VecDeque;

                if let Some(split_info) = self.split_infos.get_mut(&id) {
                    let remove_id_vec = if split_info.ty == SplitType::SplitNone {
                        let mut idx = 0;
                        let mut new_head = None;

                        // Make the second splitted widget to the head widget:
                        for split_to in split_info.split_to.iter_mut() {
                            let split_to_mut = nonnull_mut!(split_to);
                            if idx == 0 {
                                new_head = NonNull::new(split_to_mut);
                                split_to_mut.ty = SplitType::SplitNone;
                            } else {
                                let new_head_mut = unsafe { new_head.as_mut().unwrap().as_mut() };
                                new_head_mut.split_to.push(NonNull::new(split_to_mut));
                                split_to_mut.split_from = new_head;
                            }

                            idx += 1;
                        }

                        vec![split_info.id]
                    } else {
                        let split_from = split_from!(split_info);
                        split_from
                            .split_to
                            .retain(|st| unsafe { st.as_ref().unwrap().as_ref().id != id });

                        let mut remove_id_collect = vec![];
                        let mut deque: VecDeque<&SplitInfo> = VecDeque::new();
                        deque.push_back(split_info);

                        while !deque.is_empty() {
                            let split_info = deque.pop_front().unwrap();
                            remove_id_collect.push(split_info.id);

                            for split_to in split_info.split_to.iter() {
                                if let Some(ptr) = split_to {
                                    deque.push_back(unsafe { ptr.as_ref() })
                                }
                            }
                        }

                        remove_id_collect
                    };

                    for id in remove_id_vec.iter() {
                        self.split_infos.remove(id);
                        self.split_infos_vec
                            .retain(|st| unsafe { st.as_ref().unwrap().as_ref().id } != *id);
                        self.children_mut().retain(|child| child.id() != *id);
                    }

                    // Tell the `ApplicationWindow` that widget's layout has changed:
                    if self.window_id() == 0 {
                        panic!("`close_pane()` in SplitPane should invoke after window initialize.")
                    }
                    ApplicationWindow::window_of(self.window_id()).layout_change(self);
                    self.update()
                }
            }

            fn split<T: WidgetImpl>(&mut self, id: ObjectId, mut widget: Box<T>, ty: SplitType) {
                use #use_prefix::application_window::ApplicationWindow;
                use #use_prefix::{split_widget, split_from};
                use #use_prefix::tlib::nonnull_mut;
                use std::ptr::NonNull;

                let mut split_from = if let Some(split_info) = self.split_infos.get_mut(&id) {
                    NonNull::new(split_info.as_mut())
                } else {
                    panic!("The widget with id {} is not exist in SplitPane.", id)
                };

                widget.set_parent(self);
                ApplicationWindow::initialize_dynamic_component(widget.as_mut());
                let mut split_info = Box::new(SplitInfo::new(
                    widget.id(),
                    NonNull::new(widget.as_mut()),
                    split_from,
                    ty,
                ));

                nonnull_mut!(split_from).split_to.push(NonNull::new(split_info.as_mut()));
                self.split_infos_vec
                    .push(NonNull::new(split_info.as_mut()));
                self.split_infos.insert(widget.id(), split_info);
                self.container.children.push(widget);

                // Tell the `ApplicationWindow` that widget's layout has changed:
                if self.window_id() == 0 {
                    panic!("`split()` in SplitPane should invoke after window initialize.")
                }
                ApplicationWindow::window_of(self.window_id()).layout_change(self);
                self.update()
            }
        }
        })
}
