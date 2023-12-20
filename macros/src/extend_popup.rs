use crate::{
    childable::Childable, extend_element, extend_object, extend_widget, general_attr::GeneralAttr,
};
use proc_macro2::Ident;
use quote::quote;
use syn::{parse::Parser, DeriveInput};

pub(crate) fn expand(ast: &mut DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &ast.ident;

    let general_attr = GeneralAttr::parse(ast)?;

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

    match &mut ast.data {
        syn::Data::Struct(ref mut struct_data) => {
            let mut childable = Childable::new();

            match &mut struct_data.fields {
                syn::Fields::Named(fields) => {
                    fields.named.push(syn::Field::parse_named.parse2(quote! {
                        pub popup: Popup
                    })?);

                    if general_attr.is_animation {
                        let default = general_attr.animation.as_ref().unwrap().parse_default()?;
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

                    childable.parse_childable(fields)?;
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        ast,
                        "`extends(SharedWidget)` should defined on named fields struct.",
                    ))
                }
            }

            let object_trait_impl_clause = extend_object::gen_object_trait_impl_clause(
                name,
                "popup",
                vec!["popup", "widget", "element", "object"],
                false,
            )?;

            let element_trait_impl_clause = extend_element::gen_element_trait_impl_clause(
                name,
                vec!["popup", "widget", "element"],
            )?;

            let widget_trait_impl_clause = extend_widget::gen_widget_trait_impl_clause(
                name,
                Some("popup"),
                vec!["popup", "widget"],
            )?;

            let popup_trait_impl_clause = gen_popup_trait_impl_clause(name, vec!["popup"])?;

            let child_ref_clause = childable.get_child_ref();

            Ok(quote! {
                #[derive(Derivative)]
                #[derivative(Default)]
                #ast

                #object_trait_impl_clause

                #element_trait_impl_clause

                #widget_trait_impl_clause

                #animation_clause
                #animation_state_holder_impl

                #async_task_clause

                #popupable_impl_clause

                #popup_trait_impl_clause

                impl WidgetAcquire for #name {}

                impl SuperType for #name {
                    #[inline]
                    fn super_type(&self) -> Type {
                        Popup::static_type()
                    }
                }

                impl InnerInitializer for #name {
                    #[inline]
                    fn inner_type_register(&self, type_registry: &mut TypeRegistry) {
                        type_registry.register::<#name, ReflectWidgetImpl>();
                        type_registry.register::<#name, ReflectPopupImpl>();
                        type_registry.register::<#name, ReflectOverlaid>();
                        #popupable_reflect_clause
                        #animation_reflect
                        #animation_state_holder_reflect
                    }

                    #[inline]
                    fn inner_initialize(&mut self) {
                        #run_after_clause
                        self.set_property("visible", false.to_value());
                    }

                    #[inline]
                    fn pretreat_construct(&mut self) {
                        #child_ref_clause
                    }
                }

                impl PointEffective for #name {
                    #[inline]
                    fn point_effective(&self, point: &Point) -> bool {
                        self.popup.widget.point_effective(point)
                    }
                }

                impl ChildRegionAcquirer for #name {
                    #[inline]
                    fn child_region(&self) -> tlib::skia_safe::Region {
                        self.popup.widget.child_region()
                    }
                }

                impl Overlaid for #name {}

                impl #name {
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
    _popup_path: Vec<&'static str>,
) -> syn::Result<proc_macro2::TokenStream> {
    Ok(quote!(
        impl PopupExt for #name {
            #[inline]
            fn as_widget_impl(&self) -> &dyn WidgetImpl {
                self
            }

            #[inline]
            fn as_widget_impl_mut(&mut self) -> &mut dyn WidgetImpl {
                self
            }
        }
    ))
}
