use crate::{
    extend_element, extend_object, extend_widget,
    general_attr::GeneralAttr,
    pane::{generate_pane_inner_init, generate_pane_type_register},
    scroll_area::generate_scroll_area_inner_init,
    SplitGenericsRef,
};
use proc_macro2::Ident;
use quote::quote;
use syn::{
    parse::Parser, punctuated::Punctuated, spanned::Spanned, token::Pound, Attribute, DeriveInput,
    Path, Token,
};

pub(crate) fn expand(
    ast: &mut DeriveInput,
    ignore_default: bool,
    impl_children_construct: bool,
    has_content_alignment: bool,
    has_size_unified_adjust: bool,
    is_split_pane: bool,
    is_stack: bool,
    is_scroll_area: bool,
    is_pane: bool,
) -> syn::Result<proc_macro2::TokenStream> {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let general_attr = GeneralAttr::parse(ast, (&impl_generics, &ty_generics, &where_clause))?;

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
            match &mut struct_data.fields {
                syn::Fields::Named(fields) => {
                    fields.named.push(syn::Field::parse_named.parse2(quote! {
                        pub container: Container
                    })?);
                    if has_content_alignment {
                        fields.named.push(syn::Field::parse_named.parse2(quote! {
                            content_halign: Align
                        })?);
                        fields.named.push(syn::Field::parse_named.parse2(quote! {
                            content_valign: Align
                        })?);
                        fields.named.push(syn::Field::parse_named.parse2(quote! {
                            homogeneous: bool
                        })?);
                    }
                    if is_split_pane {
                        fields.named.push(syn::Field::parse_named.parse2(quote! {
                            split_infos: std::collections::HashMap<ObjectId, Box<SplitInfo>>
                        })?);
                        fields.named.push(syn::Field::parse_named.parse2(quote! {
                            split_infos_vec: Vec<std::option::Option<std::ptr::NonNull<SplitInfo>>>
                        })?);
                    }
                    if is_stack {
                        fields.named.push(syn::Field::parse_named.parse2(quote! {
                            current_index: usize
                        })?);
                    }
                    if is_scroll_area {
                        fields.named.push(syn::Field::parse_named.parse2(quote! {
                            #[derivative(Default(value = "Object::new(&[])"))]
                            scroll_bar: Box<ScrollBar>
                        })?);
                        fields.named.push(syn::Field::parse_named.parse2(quote! {
                            area: Option<Box<dyn WidgetImpl>>
                        })?);
                    }
                    if is_pane {
                        fields.named.push(syn::Field::parse_named.parse2(quote! {
                            direction: PaneDirection
                        })?);
                        fields.named.push(syn::Field::parse_named.parse2(quote! {
                            resize_zone: bool
                        })?);
                        fields.named.push(syn::Field::parse_named.parse2(quote! {
                            resize_pressed: bool
                        })?);
                    }

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

                    // If field with attribute `#[children]`,
                    // add attribute `#[derivative(Default(value = "Object::new(&[])"))]` to it:
                    for field in fields.named.iter_mut() {
                        let mut childrenable = false;
                        for attr in field.attrs.iter() {
                            if let Some(attr_ident) = attr.path.get_ident() {
                                if attr_ident.to_string() == "children" {
                                    childrenable = true;
                                    break;
                                }
                            }
                        }

                        if childrenable {
                            let mut segments = Punctuated::<syn::PathSegment, Token![::]>::new();
                            segments.push(syn::PathSegment {
                                ident: syn::Ident::new("derivative", field.span()),
                                arguments: syn::PathArguments::None,
                            });
                            let attr = Attribute {
                                pound_token: Pound {
                                    spans: [field.span()],
                                },
                                style: syn::AttrStyle::Outer,
                                bracket_token: syn::token::Bracket { span: field.span() },
                                path: Path {
                                    leading_colon: None,
                                    segments,
                                },
                                tokens: quote! {(Default(value = "Object::new(&[])"))},
                            };
                            field.attrs.push(attr);
                        }
                    }
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        ast,
                        "`extends(Container)` should defined on named fields struct.",
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
                "container",
                vec!["container", "widget", "element", "object"],
                true,
                (&impl_generics, &ty_generics, &where_clause),
            )?;

            let element_trait_impl_clause = extend_element::gen_element_trait_impl_clause(
                name,
                vec!["container", "widget", "element"],
                (&impl_generics, &ty_generics, &where_clause),
            )?;

            let widget_trait_impl_clause = extend_widget::gen_widget_trait_impl_clause(
                name,
                Some("container"),
                vec!["container", "widget"],
                (&impl_generics, &ty_generics, &where_clause),
            )?;

            let container_trait_impl_clause = gen_container_trait_impl_clause(
                name,
                vec!["container"],
                (&impl_generics, &ty_generics, &where_clause),
            )?;

            let mut children_construct_clause = proc_macro2::TokenStream::new();
            if impl_children_construct {
                children_construct_clause.extend(quote!(
                    impl #impl_generics ObjectChildrenConstruct for #name #ty_generics #where_clause {}
                ))
            }

            let reflect_content_alignment = if has_content_alignment {
                quote!(type_registry.register::<#name, ReflectContentAlignment>();)
            } else {
                proc_macro2::TokenStream::new()
            };

            let reflect_size_unified_adjust = if has_size_unified_adjust {
                quote!(type_registry.register::<#name, ReflectSizeUnifiedAdjust>();)
            } else {
                proc_macro2::TokenStream::new()
            };

            let reflect_split_infos_getter = if is_split_pane {
                quote!(
                    type_registry.register::<#name, ReflectSplitInfosGetter>();
                )
            } else {
                proc_macro2::TokenStream::new()
            };

            let reflect_stack_trait = if is_stack {
                quote!(type_registry.register::<#name, ReflectStackTrait>();)
            } else {
                proc_macro2::TokenStream::new()
            };

            let reflect_scroll_area = if is_scroll_area {
                quote!(type_registry.register::<#name, ReflectScrollAreaExt>();)
            } else {
                proc_macro2::TokenStream::new()
            };

            let reflect_pane = if is_pane {
                generate_pane_type_register(name)?
            } else {
                proc_macro2::TokenStream::new()
            };

            let scroll_area_inner_init = if is_scroll_area {
                generate_scroll_area_inner_init()?
            } else {
                proc_macro2::TokenStream::new()
            };

            let pane_inner_init = if is_pane {
                generate_pane_inner_init()?
            } else {
                proc_macro2::TokenStream::new()
            };

            Ok(quote!(
                #default_clause
                #ast

                #object_trait_impl_clause

                #element_trait_impl_clause

                #widget_trait_impl_clause

                #container_trait_impl_clause

                #children_construct_clause

                #animation_clause
                #animation_state_holder_impl

                #async_task_clause

                #popupable_impl_clause

                impl #impl_generics ContainerAcquire for #name #ty_generics #where_clause {}

                impl #impl_generics SuperType for #name #ty_generics #where_clause {
                    #[inline]
                    fn super_type(&self) -> Type {
                        Container::static_type()
                    }
                }

                impl #impl_generics InnerInitializer for #name #ty_generics #where_clause {
                    #[inline]
                    fn inner_type_register(&self, type_registry: &mut TypeRegistry) {
                        type_registry.register::<#name, ReflectWidgetImpl>();
                        type_registry.register::<#name, ReflectContainerImpl>();
                        type_registry.register::<#name, ReflectObjectChildrenConstruct>();
                        type_registry.register::<#name, ReflectChildContainerDiffRender>();
                        #reflect_content_alignment
                        #reflect_size_unified_adjust
                        #reflect_split_infos_getter
                        #reflect_stack_trait
                        #reflect_scroll_area
                        #popupable_reflect_clause
                        #animation_reflect
                        #animation_state_holder_reflect
                        #reflect_pane
                    }

                    #[inline]
                    fn inner_initialize(&mut self) {
                        #run_after_clause
                        #scroll_area_inner_init
                        #pane_inner_init
                    }

                    #[inline]
                    fn pretreat_construct(&mut self) {

                    }
                }

                impl #impl_generics PointEffective for #name #ty_generics #where_clause {
                    #[inline]
                    fn point_effective(&self, point: &Point) -> bool {
                        self.container_point_effective(point)
                    }
                }

                impl #impl_generics ChildRegionAcquirer for #name #ty_generics #where_clause {
                    #[inline]
                    fn child_region(&self) -> tlib::skia_safe::Region {
                        self.children_region()
                    }
                }

                impl #impl_generics #name #ty_generics #where_clause {
                    #async_method_clause
                }
            ))
        }
        _ => Err(syn::Error::new_spanned(
            ast,
            "`extends(Container)` has to be used with structs ",
        )),
    }
}

pub(crate) fn gen_container_trait_impl_clause<'a>(
    name: &Ident,
    container_path: Vec<&'static str>,
    (impl_generics, ty_generics, where_clause): SplitGenericsRef<'a>,
) -> syn::Result<proc_macro2::TokenStream> {
    let container_path: Vec<_> = container_path
        .iter()
        .map(|s| Ident::new(s, name.span()))
        .collect();

    Ok(quote!(
        impl #impl_generics ContainerExt for #name #ty_generics #where_clause {
            #[inline]
            fn is_strict_children_layout(&self) -> bool {
                self.#(#container_path).*.is_strict_children_layout()
            }

            #[inline]
            fn set_strict_children_layout(&mut self, strict_children_layout: bool) {
                self.#(#container_path).*.set_strict_children_layout(strict_children_layout)
            }
        }
    ))
}
