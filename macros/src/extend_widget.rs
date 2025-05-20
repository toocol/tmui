use crate::{
    childable::Childable, extend_element, extend_object, general_attr::GeneralAttr, layout,
    SplitGenericsRef,
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
        GeneralAttr::parse(ast, (&impl_generics, &ty_generics, &where_clause), false)?;

    expand_with_general_attr(ast, ignore_default, general_attr)
}

pub(crate) fn expand_with_general_attr(
    ast: &mut DeriveInput,
    ignore_default: bool,
    general_attr: GeneralAttr,
) -> syn::Result<proc_macro2::TokenStream> {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

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

    let loadable_impl_clause = &general_attr.loadable_impl_clause;
    let loadable_reflect_clause = &general_attr.loadable_reflect_clause;

    let global_watch_impl_clause = &general_attr.global_watch_impl_clause;
    let global_watch_reflect_clause = &general_attr.global_watch_reflect_clause;

    let iter_executor_reflect_clause = &general_attr.iter_executor_reflect_clause;

    let frame_animator_reflect_clause = &general_attr.frame_animator_reflect_clause;

    let isolated_visibility_impl_clause = &general_attr.isolated_visibility_impl_clause;
    let isolated_visibility_reflect_clause = &general_attr.isolated_visibility_reflect_clause;

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
                        pub widget: Widget
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

                    if general_attr.is_loadable {
                        let field = &general_attr.loadable_field_clause;
                        fields.named.push(syn::Field::parse_named.parse2(quote! {
                            #field
                        })?);
                    }

                    if general_attr.is_isolated_visibility {
                        let iv_fields = &general_attr.isolated_visibility_field_clause;
                        for field in iv_fields.iter() {
                            fields.named.push(syn::Field::parse_named.parse2(quote! {
                                #field
                            })?);
                        }
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
                        "`extends(Widget)` should defined on named fields struct.",
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
                "widget",
                vec!["widget", "element", "object"],
                false,
                (&impl_generics, &ty_generics, &where_clause),
            )?;

            let element_trait_impl_clause = extend_element::gen_element_trait_impl_clause(
                name,
                vec!["widget", "element"],
                (&impl_generics, &ty_generics, &where_clause),
            )?;

            let widget_trait_impl_clause = gen_widget_trait_impl_clause(
                name,
                vec!["widget"],
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

                #loadable_impl_clause

                #global_watch_impl_clause

                #isolated_visibility_impl_clause

                #close_handler_impl_clause

                #win_widget_sink_impl

                impl #impl_generics WidgetAcquire for #name #ty_generics #where_clause {}

                impl #impl_generics SuperType for #name #ty_generics #where_clause {
                    #[inline]
                    fn super_type(&self) -> Type {
                        Widget::static_type()
                    }
                }

                impl #impl_generics InnerInitializer for #name #ty_generics #where_clause {
                    #[inline]
                    fn inner_type_register(&self, type_registry: &mut TypeRegistry) {
                        type_registry.register::<#name #ty_generics, ReflectWidgetImpl>();
                        #popupable_reflect_clause
                        #animation_reflect
                        #animation_state_holder_reflect
                        #loadable_reflect_clause
                        #global_watch_reflect_clause
                        #iter_executor_reflect_clause
                        #frame_animator_reflect_clause
                        #isolated_visibility_reflect_clause
                        #close_handler_reflect_clause
                        #win_widget_sink_reflect
                    }

                    #[inline]
                    fn inner_initialize(&mut self) {
                        #run_after_clause
                        #close_handler_register_clause
                    }

                    #[inline]
                    fn pretreat_construct(&mut self) {
                        #child_ref_clause
                    }

                    #[inline]
                    fn inner_on_property_set(&mut self, name: &str, value: &Value) -> bool {
                        match name {
                            "visible" => {
                                let visible = value.get::<bool>();
                                emit!(self, visibility_changed(visible));
                                self.inner_visibility_changed(visible);
                                self.on_visibility_changed(visible);
                                if let Some(child) = self.get_child_mut() {
                                    if visible {
                                        if !child.visibility_check() {
                                            return true;
                                        }
                                        if let Some(iv) = cast!(child as IsolatedVisibility) {
                                            if iv.auto_hide() {
                                                return true;
                                            }
                                        }

                                        child.set_property("visible", true.to_value());
                                        child.set_render_styles(true);
                                    } else {
                                        child.set_property("visible", false.to_value());
                                    }
                                }
                                true
                            }
                            _ => false,
                        }
                    }
                }

                impl #impl_generics PointEffective for #name #ty_generics #where_clause {
                    #[inline]
                    fn point_effective(&self, point: &Point) -> bool {
                        self.widget.point_effective(point)
                    }
                }

                impl #impl_generics ChildRegionAcquire for #name #ty_generics #where_clause {
                    #[inline]
                    fn child_region(&self) -> tlib::skia_safe::Region {
                        self.widget.child_region()
                    }
                }

                impl #impl_generics InnerRunAfter for #name #ty_generics #where_clause {
                    #[inline]
                    fn inner_run_after(&mut self) {}
                }

                impl #impl_generics #name #ty_generics #where_clause {
                    #async_method_clause
                }
            })
        }
        _ => Err(syn::Error::new_spanned(
            ast,
            "`extends(Widget)` has to be used with structs ",
        )),
    }
}

#[inline]
pub(crate) fn expand_with_layout(
    ast: &mut DeriveInput,
    layout_meta: &Meta,
    layout: &str,
    ignore_default: bool,
) -> syn::Result<proc_macro2::TokenStream> {
    layout::expand(ast, layout_meta, layout, ignore_default, false)
}

pub(crate) fn gen_widget_trait_impl_clause(
    name: &Ident,
    widget_path: Vec<&'static str>,
    (impl_generics, ty_generics, where_clause): SplitGenericsRef<'_>,
) -> syn::Result<proc_macro2::TokenStream> {
    let widget_path: Vec<_> = widget_path
        .iter()
        .map(|s| Ident::new(s, name.span()))
        .collect();

    Ok(quote!(
        impl #impl_generics WidgetPropsAcquire for #name #ty_generics #where_clause {
            #[inline]
            fn widget_props(&self) -> &Widget {
                self.#(#widget_path).*.widget_props()
            }

            #[inline]
            fn widget_props_mut(&mut self) -> &mut Widget {
                self.#(#widget_path).*.widget_props_mut()
            }
        }

        impl #impl_generics ChildOp for #name #ty_generics #where_clause {
            #[inline]
            fn child<_T: WidgetImpl>(&mut self, mut child: Tr<_T>) {
                if self.super_type().is_a(Container::static_type()) {
                    panic!("function `child()` was invalid in `Container`, use `add_child()` instead")
                }
                child.set_parent(self);
                self.#(#widget_path).*._child_internal(child)
            }

            #[inline]
            fn remove_child(&mut self) {
                if self.super_type().is_a(Container::static_type()) {
                    panic!("function `remove_child()` was invalid in `Container`, use `remove_children()` instead")
                }
                self.#(#widget_path).*._remove_child_internal()
            }
        }

        impl #impl_generics IsA<Widget> for #name #ty_generics #where_clause {}
    ))
}
