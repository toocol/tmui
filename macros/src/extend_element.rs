use crate::{extend_object, general_attr::GeneralAttr, SplitGenericsRef};
use proc_macro2::Ident;
use quote::quote;
use syn::{parse::Parser, DeriveInput};

pub(crate) fn expand(
    ast: &mut DeriveInput,
    ignore_default: bool,
) -> syn::Result<proc_macro2::TokenStream> {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let general_attr =
        GeneralAttr::parse(ast, (&impl_generics, &ty_generics, &where_clause), false)?;

    let async_task_clause = &general_attr.async_task_impl_clause;
    let async_method_clause = &general_attr.async_task_method_clause;

    match &mut ast.data {
        syn::Data::Struct(ref mut struct_data) => {
            match &mut struct_data.fields {
                syn::Fields::Named(fields) => {
                    fields.named.push(syn::Field::parse_named.parse2(quote! {
                        pub element: Element
                    })?);

                    if general_attr.is_async_task {
                        for async_field in general_attr.async_task_fields.iter() {
                            fields.named.push(syn::Field::parse_named.parse2(quote! {
                                #async_field
                            })?);
                        }
                    }
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        ast,
                        "`extends(Element)` should defined on named fields struct.",
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
                "element",
                vec!["element", "object"],
                false,
                (&impl_generics, &ty_generics, &where_clause),
            )?;

            let element_trait_impl_clause = gen_element_trait_impl_clause(
                name,
                vec!["element"],
                (&impl_generics, &ty_generics, &where_clause),
            )?;

            Ok(quote! {
                #default_clause
                #ast

                #object_trait_impl_clause

                #element_trait_impl_clause

                #async_task_clause

                impl #impl_generics ElementAcquire for #name #ty_generics #where_clause {}

                impl #impl_generics SuperType for #name #ty_generics #where_clause {
                    #[inline]
                    fn super_type(&self) -> Type {
                        Element::static_type()
                    }
                }

                impl #impl_generics InnerInitializer for #name #ty_generics #where_clause {
                    #[inline]
                    fn inner_type_register(&self, type_registry: &mut TypeRegistry) {
                        type_registry.register::<#name #ty_generics, ReflectElementImpl>();
                    }
                }

                impl #impl_generics #name #ty_generics #where_clause {
                    #async_method_clause
                }
            })
        }
        _ => Err(syn::Error::new_spanned(
            ast,
            "`extends(Element)` has to be used with structs ",
        )),
    }
}

pub(crate) fn gen_element_trait_impl_clause(
    name: &Ident,
    element_path: Vec<&'static str>,
    (impl_generics, ty_generics, where_clause): SplitGenericsRef<'_>,
) -> syn::Result<proc_macro2::TokenStream> {
    let element_path: Vec<_> = element_path
        .iter()
        .map(|s| Ident::new(s, name.span()))
        .collect();

    Ok(quote!(
        impl #impl_generics ElementPropsAcquire for #name #ty_generics #where_clause {
            #[inline]
            fn element_props(&self) -> &Element {
                &self.#(#element_path).*
            }

            #[inline]
            fn element_props_mut(&mut self) -> &mut Element {
                &mut self.#(#element_path).*
            }
        }

        impl #impl_generics IsA<Element> for #name #ty_generics #where_clause {}
    ))
}
