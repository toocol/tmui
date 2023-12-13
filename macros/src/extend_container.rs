use crate::{extend_element, extend_object, extend_widget, scroll_area::generate_scroll_area_inner_init, async_task::AsyncTask, animation};
use quote::quote;
use syn::{
    parse::Parser, punctuated::Punctuated, spanned::Spanned, token::Pound, Attribute, DeriveInput,
    Path, Token,
};

pub(crate) fn expand(
    ast: &mut DeriveInput,
    impl_children_construct: bool,
    has_content_alignment: bool,
    is_split_pane: bool,
    is_stack: bool,
    is_scroll_area: bool,
) -> syn::Result<proc_macro2::TokenStream> {
    let name = &ast.ident;

    let mut run_after = false;
    let mut animation = false;
    let mut is_async_task = false;
    let mut async_tasks = vec![];

    for attr in ast.attrs.iter() {
        if let Some(attr_ident) = attr.path.get_ident() {
            let attr_str = attr_ident.to_string();

            match attr_str.as_str() {
                "run_after" => run_after = true,
                "animatable" => animation = true,
                "async_task" => {
                    is_async_task = true;
                    async_tasks.push(AsyncTask::parse_attr(attr));
                }
                _ => {}
            }
        }
    }

    let run_after_clause = if run_after {
        quote!(
            ApplicationWindow::run_afters_of(self.window_id()).push(
                std::ptr::NonNull::new(self)
            );
        )
    } else {
        proc_macro2::TokenStream::new()
    };

    let animation_clause = if animation {
        animation::generate_animation(name)?
    } else {
        proc_macro2::TokenStream::new()
    };

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

                    if animation {
                        fields.named.push(syn::Field::parse_named.parse2(quote! {
                            pub animation: AnimationModel
                        })?);
                    }

                    if is_async_task {
                        for async_task in async_tasks.iter() {
                            if async_task.is_none() {
                                return Err(syn::Error::new_spanned(
                                    ast,
                                    "proc_macro `async_task` format error.",
                                ));
                            }
                            let task = async_task.as_ref().unwrap();
                            let task_name = task.name.as_ref().unwrap();
                            let field = task.field.as_ref().unwrap();

                            fields.named.push(syn::Field::parse_named.parse2(quote! {
                                #field: Option<Box<#task_name>>
                            })?);
                        }
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

            let object_trait_impl_clause = extend_object::gen_object_trait_impl_clause(
                name,
                "container",
                vec!["container", "widget", "element", "object"],
                true,
            )?;

            let element_trait_impl_clause = extend_element::gen_element_trait_impl_clause(
                name,
                vec!["container", "widget", "element"],
            )?;

            let widget_trait_impl_clause = extend_widget::gen_widget_trait_impl_clause(
                name,
                Some("container"),
                vec!["container", "widget"],
            )?;

            let mut children_construct_clause = proc_macro2::TokenStream::new();
            if impl_children_construct {
                children_construct_clause.extend(quote!(
                    impl ObjectChildrenConstruct for #name {}
                ))
            }

            let reflect_content_alignment = if has_content_alignment {
                quote!(type_registry.register::<#name, ReflectContentAlignment>();)
            } else {
                proc_macro2::TokenStream::new()
            };

            let reflect_split_infos_getter = if is_split_pane {
                quote!(
                    type_registry.register::<#name, ReflectSplitInfosGetter>();
                    type_registry.register::<#name, ReflectSizeUnifiedAdjust>();
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

            let scroll_area_inner_init = if is_scroll_area {
                generate_scroll_area_inner_init()?
            } else {
                proc_macro2::TokenStream::new()
            };

            let async_task_clause = if is_async_task {
                let mut clause = proc_macro2::TokenStream::new();
                for async_task in async_tasks.iter() {
                    clause.extend(async_task.as_ref().unwrap().expand(ast)?)
                }
                clause
            } else {
                proc_macro2::TokenStream::new()
            };

            let async_method_clause = if is_async_task {
                let mut clause = proc_macro2::TokenStream::new();
                for async_task in async_tasks.iter() {
                    clause.extend(async_task.as_ref().unwrap().expand_method(ast)?)
                }
                clause
            } else {
                proc_macro2::TokenStream::new()
            };

            Ok(quote!(
                #[derive(Derivative)]
                #[derivative(Default)]
                #ast

                #object_trait_impl_clause

                #element_trait_impl_clause

                #widget_trait_impl_clause

                #children_construct_clause

                #animation_clause

                #async_task_clause

                impl ContainerAcquire for #name {}

                impl SuperType for #name {
                    #[inline]
                    fn super_type(&self) -> Type {
                        Container::static_type()
                    }
                }

                impl InnerInitializer for #name {
                    #[inline]
                    fn inner_type_register(&self, type_registry: &mut TypeRegistry) {
                        type_registry.register::<#name, ReflectWidgetImpl>();
                        type_registry.register::<#name, ReflectContainerImpl>();
                        type_registry.register::<#name, ReflectObjectChildrenConstruct>();
                        type_registry.register::<#name, ReflectChildContainerDiffRender>();
                        #reflect_content_alignment
                        #reflect_split_infos_getter
                        #reflect_stack_trait
                        #reflect_scroll_area
                    }

                    #[inline]
                    fn inner_initialize(&mut self) {
                        #run_after_clause
                        #scroll_area_inner_init
                    }
                }

                impl PointEffective for #name {
                    #[inline]
                    fn point_effective(&self, point: &Point) -> bool {
                        self.container_point_effective(point)
                    }
                }

                impl #name {
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
