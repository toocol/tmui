use crate::{
    extend_element, extend_object, extend_widget, general_attr::GeneralAttr, SplitGenericsRef,
};
use quote::quote;
use syn::{parse::Parser, DeriveInput, Ident};

pub(crate) fn expand(
    ast: &mut DeriveInput,
    id: Option<&String>,
    ignore_default: bool,
) -> syn::Result<proc_macro2::TokenStream> {
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let general_attr =
        GeneralAttr::parse(ast, (&impl_generics, &ty_generics, &where_clause), false)?;

    let name = &ast.ident;

    let async_task_clause = &general_attr.async_task_impl_clause;
    let async_method_clause = &general_attr.async_task_method_clause;

    let popupable_impl_clause = &general_attr.popupable_impl_clause;
    let popupable_reflect_clause = &general_attr.popupable_reflect_clause;

    let global_watch_impl_clause = &general_attr.global_watch_impl_clause;
    let global_watch_reflect_clause = &general_attr.global_watch_reflect_clause;

    let iter_executor_reflect_clause = &general_attr.iter_executor_reflect_clause;

    let set_shared_id_clause = match id {
        Some(id) => quote!(
            self.set_shared_id(#id);
        ),
        None => proc_macro2::TokenStream::new(),
    };

    match &mut ast.data {
        syn::Data::Struct(ref mut struct_data) => {
            match &mut struct_data.fields {
                syn::Fields::Named(fields) => {
                    fields.named.push(syn::Field::parse_named.parse2(quote! {
                        pub shared_widget: SharedWidget
                    })?);

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
                "shared_widget",
                vec!["shared_widget", "widget", "element", "object"],
                false,
                (&impl_generics, &ty_generics, &where_clause),
            )?;

            let element_trait_impl_clause = extend_element::gen_element_trait_impl_clause(
                name,
                vec!["shared_widget", "widget", "element"],
                (&impl_generics, &ty_generics, &where_clause),
            )?;

            let widget_trait_impl_clause = extend_widget::gen_widget_trait_impl_clause(
                name,
                vec!["shared_widget", "widget"],
                (&impl_generics, &ty_generics, &where_clause),
            )?;

            let shared_widget_trait_impl_clause = gen_shared_widget_trait_impl_clause(
                name,
                vec!["shared_widget"],
                (&impl_generics, &ty_generics, &where_clause),
            )?;

            Ok(quote! {
                #default_clause
                #ast

                #object_trait_impl_clause

                #element_trait_impl_clause

                #widget_trait_impl_clause

                #shared_widget_trait_impl_clause

                #async_task_clause

                #popupable_impl_clause

                #global_watch_impl_clause

                impl #impl_generics WidgetAcquire for #name #ty_generics #where_clause {}

                impl #impl_generics SuperType for #name #ty_generics #where_clause {
                    #[inline]
                    fn super_type(&self) -> Type {
                        SharedWidget::static_type()
                    }
                }

                impl #impl_generics InnerInitializer for #name #ty_generics #where_clause {
                    #[inline]
                    fn inner_type_register(&self, type_registry: &mut TypeRegistry) {
                        type_registry.register::<#name, ReflectWidgetImpl>();
                        type_registry.register::<#name, ReflectSharedWidgetImpl>();
                        #popupable_reflect_clause
                        #global_watch_reflect_clause
                        #iter_executor_reflect_clause
                    }

                    #[inline]
                    fn inner_initialize(&mut self) {
                        #set_shared_id_clause
                        ApplicationWindow::run_afters_of(self.window_id()).push(
                            std::ptr::NonNull::new(self)
                        );
                    }
                }

                impl #impl_generics PointEffective for #name #ty_generics #where_clause {
                    #[inline]
                    fn point_effective(&self, point: &Point) -> bool {
                        self.shared_widget.widget.point_effective(point)
                    }
                }

                impl #impl_generics ChildRegionAcquire for #name #ty_generics #where_clause {
                    #[inline]
                    fn child_region(&self) -> tlib::skia_safe::Region {
                        self.shared_widget.widget.child_region()
                    }
                }

                impl #impl_generics InnerRunAfter for #name #ty_generics #where_clause {
                    #[inline]
                    fn inner_run_after(&mut self) {
                        self.shared_widget.run_after();
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

pub(crate) fn gen_shared_widget_trait_impl_clause(
    name: &Ident,
    shared_widget_path: Vec<&'static str>,
    (impl_generics, ty_generics, where_clause): SplitGenericsRef<'_>,
) -> syn::Result<proc_macro2::TokenStream> {
    let shared_widget_path: Vec<_> = shared_widget_path
        .iter()
        .map(|s| Ident::new(s, name.span()))
        .collect();

    Ok(quote!(
        impl #impl_generics SharedWidgetExt for #name #ty_generics #where_clause {
            #[inline]
            fn shared_id(&self) -> &'static str {
                self.#(#shared_widget_path).*.shared_id()
            }

            #[inline]
            fn set_shared_id(&mut self, id: &'static str) {
                self.#(#shared_widget_path).*.set_shared_id(id)
            }

            #[inline]
            fn image_info(&self) -> &tlib::skia_safe::ImageInfo {
                self.#(#shared_widget_path).*.image_info()
            }

            #[inline]
            fn is_shared_invalidate(&self) -> bool {
                self.#(#shared_widget_path).*.is_shared_invalidate()
            }

            #[inline]
            fn shared_validate(&self) {
                self.#(#shared_widget_path).*.shared_validate()
            }

            #[inline]
            fn pixels_render(&mut self, painter: &mut Painter) {
                self.#(#shared_widget_path).*.pixels_render(painter)
            }
        }

        impl #impl_generics IsA<SharedWidget> for #name #ty_generics #where_clause {}
    ))
}
