use crate::{
    childable::Childable, extend_element, extend_object, extend_widget, general_attr::GeneralAttr,
    layout, SplitGenericsRef,
};
use proc_macro2::Ident;
use quote::quote;
use syn::{parse::Parser, DeriveInput, Meta};

pub(crate) fn expand(
    ast: &mut DeriveInput,
    ignore_default: bool,
) -> syn::Result<proc_macro2::TokenStream> {
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let general_attr =
        GeneralAttr::parse(ast, (&impl_generics, &ty_generics, &where_clause), true)?;
    if general_attr.is_win_widget {
        return extend_widget::expand_with_general_attr(ast, ignore_default, general_attr);
    }

    let name = &ast.ident;

    let run_after_clause = &general_attr.run_after_clause;

    let animation_clause = &general_attr.animation_clause;
    let animation_reflect = &general_attr.animation_reflect;
    let animation_state_holder_field = &general_attr.animation_state_holder_field;
    let animation_state_holder_impl = &general_attr.animation_state_holder_impl;
    let animation_state_holder_reflect = &general_attr.animation_state_holder_reflect;

    let async_task_clause = &general_attr.async_task_impl_clause;
    let async_method_clause = &general_attr.async_task_method_clause;

    let popupable_impl_clause = &general_attr.popupable_impl_clause;
    let popupable_reflect_clause = &general_attr.popupable_reflect_clause;

    let global_watch_impl_clause = &general_attr.global_watch_impl_clause;
    let global_watch_reflect_clause = &general_attr.global_watch_reflect_clause;

    let iter_executor_reflect_clause = &general_attr.iter_executor_reflect_clause;

    let frame_animator_reflect_clause = &general_attr.frame_animator_reflect_clause;

    let close_handler_impl_clause = &general_attr.close_handler_impl_clause;
    let close_handler_reflect_clause = &general_attr.close_handler_reflect_clause;
    let close_handler_register_clause = &general_attr.close_handler_register_clause;

    let win_widget_sink_field = &general_attr.win_widget_sink_field;
    let win_widget_sink_impl = &general_attr.win_widget_sink_impl;
    let win_widget_sink_reflect = &general_attr.win_widget_sink_reflect;
    let win_widget_corr_struct_clause = &general_attr.win_widget_corr_struct_clause;

    match &mut ast.data {
        syn::Data::Struct(ref mut struct_data) => {
            let mut childable = Childable::new();

            match &mut struct_data.fields {
                syn::Fields::Named(fields) => {
                    fields.named.push(syn::Field::parse_named.parse2(quote! {
                        pub popup: Popup
                    })?);

                    if general_attr.is_animation {
                        let default = &general_attr.animation_parse_default;
                        let field = &general_attr.animation_field;
                        fields.named.push(syn::Field::parse_named.parse2(quote! {
                            #default
                            #field
                        })?);
                        fields.named.push(syn::Field::parse_named.parse2(quote! {
                            #animation_state_holder_field
                        })?);
                    }

                    if general_attr.is_async_task {
                        for async_field in general_attr.async_task_fields.iter() {
                            fields.named.push(syn::Field::parse_named.parse2(quote! {
                                #async_field
                            })?);
                        }
                    }

                    if general_attr.is_popupable {
                        let field = &general_attr.popupable_field_clause;
                        fields.named.push(syn::Field::parse_named.parse2(quote! {
                            #field
                        })?);
                    }

                    for field in win_widget_sink_field.iter() {
                        fields.named.push(syn::Field::parse_named.parse2(quote! {
                            #field
                        })?);
                    }

                    childable.parse_childable(fields)?;
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        ast,
                        "`extends(SharedWidget)` should defined on named fields struct.",
                    ))
                }
            }

            let default_clause = if ignore_default {
                quote!()
            } else {
                quote!(
                    #[derive(Derivative)]
                    #[derivative(Default)]
                )
            };

            let object_trait_impl_clause = extend_object::gen_object_trait_impl_clause(
                name,
                "popup",
                vec!["popup", "widget", "element", "object"],
                false,
                (&impl_generics, &ty_generics, &where_clause),
            )?;

            let element_trait_impl_clause = extend_element::gen_element_trait_impl_clause(
                name,
                vec!["popup", "widget", "element"],
                (&impl_generics, &ty_generics, &where_clause),
            )?;

            let widget_trait_impl_clause = extend_widget::gen_widget_trait_impl_clause(
                name,
                vec!["popup", "widget"],
                (&impl_generics, &ty_generics, &where_clause),
            )?;

            let popup_trait_impl_clause = gen_popup_trait_impl_clause(
                name,
                vec!["popup"],
                (&impl_generics, &ty_generics, &where_clause),
            )?;

            let child_ref_clause = childable.get_child_ref();

            Ok(quote! {
                #default_clause
                #ast

                #win_widget_corr_struct_clause

                #object_trait_impl_clause

                #element_trait_impl_clause

                #widget_trait_impl_clause

                #animation_clause
                #animation_state_holder_impl

                #async_task_clause

                #popupable_impl_clause

                #popup_trait_impl_clause

                #global_watch_impl_clause

                #close_handler_impl_clause

                #win_widget_sink_impl

                impl #impl_generics WidgetAcquire for #name #ty_generics #where_clause {}

                impl #impl_generics SuperType for #name #ty_generics #where_clause {
                    #[inline]
                    fn super_type(&self) -> Type {
                        Popup::static_type()
                    }
                }

                impl #impl_generics InnerInitializer for #name #ty_generics #where_clause {
                    #[inline]
                    fn inner_type_register(&self, type_registry: &mut TypeRegistry) {
                        type_registry.register::<#name, ReflectWidgetImpl>();
                        type_registry.register::<#name, ReflectPopupImpl>();
                        type_registry.register::<#name, ReflectOverlaid>();
                        #popupable_reflect_clause
                        #animation_reflect
                        #animation_state_holder_reflect
                        #global_watch_reflect_clause
                        #iter_executor_reflect_clause
                        #frame_animator_reflect_clause
                        #close_handler_reflect_clause
                        #win_widget_sink_reflect
                    }

                    #[inline]
                    fn inner_initialize(&mut self) {
                        #run_after_clause
                        self.set_property("visible", false.to_value());
                        if !self.background().is_opaque() {
                            self.set_background(Color::WHITE);
                        }
                        #close_handler_register_clause
                    }

                    #[inline]
                    fn pretreat_construct(&mut self) {
                        #child_ref_clause
                        let window = ApplicationWindow::window();
                        connect!(window, size_changed(), self, on_win_size_change(Size));
                    }
                }

                impl #impl_generics PointEffective for #name #ty_generics #where_clause {
                    #[inline]
                    fn point_effective(&self, point: &Point) -> bool {
                        self.popup.widget.point_effective(point)
                    }
                }

                impl #impl_generics ChildRegionAcquire for #name #ty_generics #where_clause {
                    #[inline]
                    fn child_region(&self) -> tlib::skia_safe::Region {
                        self.popup.widget.child_region()
                    }
                }

                impl #impl_generics #name #ty_generics #where_clause {
                    #async_method_clause
                }
            })
        }
        _ => Err(syn::Error::new_spanned(
            ast,
            "`extends(SharedWidget)` has to be used with structs ",
        )),
    }
}

pub(crate) fn gen_popup_trait_impl_clause(
    name: &Ident,
    popup_path: Vec<&'static str>,
    (impl_generics, ty_generics, where_clause): SplitGenericsRef<'_>,
) -> syn::Result<proc_macro2::TokenStream> {
    let popup_path: Vec<_> = popup_path
        .iter()
        .map(|s| Ident::new(s, name.span()))
        .collect();
    Ok(quote!(
        impl #impl_generics Overlaid for #name #ty_generics #where_clause {}

        impl #impl_generics PopupExt for #name #ty_generics #where_clause {
            #[inline]
            fn as_widget_impl(&self) -> &dyn WidgetImpl {
                self
            }

            #[inline]
            fn as_widget_impl_mut(&mut self) -> &mut dyn WidgetImpl {
                self
            }

            #[inline]
            fn set_supervisor(&mut self, widget: &mut dyn WidgetImpl) {
                self.#(#popup_path).*.set_supervisor(widget)
            }

            #[inline]
            fn supervisor(&self) -> &dyn WidgetImpl {
                self.#(#popup_path).*.supervisor()
            }

            #[inline]
            fn supervisor_mut(&mut self) -> &mut dyn WidgetImpl {
                self.#(#popup_path).*.supervisor_mut()
            }

            #[inline]
            fn calc_relative_position(&mut self) {
                self.#(#popup_path).*.calc_relative_position()
            }

            #[inline]
            fn layout_relative_position(&mut self) {
                self.#(#popup_path).*.layout_relative_position()
            }

            #[inline]
            fn is_hide_on_win_change(&self) -> bool {
                self.#(#popup_path).*.is_hide_on_win_change()
            }

            #[inline]
            fn set_hide_on_win_change(&mut self, on: bool) {
                self.#(#popup_path).*.set_hide_on_win_change(on)
            }
        }
    ))
}

#[inline]
pub(crate) fn expand_with_layout(
    ast: &mut DeriveInput,
    layout_meta: &Meta,
    layout: &str,
    ignore_default: bool,
) -> syn::Result<proc_macro2::TokenStream> {
    layout::expand(ast, layout_meta, layout, ignore_default, true)
}
