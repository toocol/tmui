use proc_macro2::Ident;
use quote::quote;
use syn::{parse::Parser, DeriveInput};

use crate::SplitGenericsRef;

pub(crate) fn expand(
    ast: &mut DeriveInput,
    ignore_default: bool,
) -> syn::Result<proc_macro2::TokenStream> {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    match &mut ast.data {
        syn::Data::Struct(ref mut struct_data) => {
            match &mut struct_data.fields {
                syn::Fields::Named(fields) => {
                    fields.named.push(syn::Field::parse_named.parse2(quote! {
                        pub object: Object
                    })?);
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        ast,
                        "`extends(Object)` should defined on named fields struct.",
                    ))
                }
            }

            let object_trait_impl_clause = gen_object_trait_impl_clause(
                name,
                "object",
                vec!["object"],
                false,
                (&impl_generics, &ty_generics, &where_clause),
            )?;

            let default_clause = if ignore_default {
                quote!()
            } else {
                quote!(
                    #[derive(Derivative)]
                    #[derivative(Default)]
                )
            };

            Ok(quote! {
                #default_clause
                #ast

                #object_trait_impl_clause

                impl #impl_generics ObjectAcquire for #name #ty_generics #where_clause {}

                impl #impl_generics SuperType for #name #ty_generics #where_clause {
                    #[inline]
                    fn super_type(&self) -> Type {
                        Object::static_type()
                    }
                }

                impl #impl_generics InnerInitializer for #name #ty_generics #where_clause {
                    #[inline]
                    fn inner_type_register(&self, type_registry: &mut TypeRegistry) {
                        type_registry.register::<#name #ty_generics, ReflectObjectImpl>();
                        type_registry.register::<#name #ty_generics, ReflectObjectImplExt>();
                        type_registry.register::<#name #ty_generics, ReflectObjectOperation>();
                    }
                }
            })
        }
        _ => Err(syn::Error::new_spanned(
            ast,
            "`extends(Object)` has to be used with structs ",
        )),
    }
}

pub(crate) fn gen_object_trait_impl_clause(
    name: &Ident,
    super_field: &'static str,
    object_path: Vec<&'static str>,
    children_construct: bool,
    (impl_generics, ty_generics, where_clause): SplitGenericsRef<'_>,
) -> syn::Result<proc_macro2::TokenStream> {
    let super_field = Ident::new(super_field, name.span());
    let object_path: Vec<_> = object_path
        .iter()
        .map(|s| Ident::new(s, name.span()))
        .collect();

    let mut children_construct_clause = proc_macro2::TokenStream::new();
    if children_construct {
        children_construct_clause.extend(quote!(
            self.children_construct();
        ))
    }

    Ok(quote!(
        impl #impl_generics ObjectImplExt for #name #ty_generics #where_clause {
            #[inline]
            fn parent_construct(&mut self) {
                #children_construct_clause
                self.#super_field.construct()
            }

            #[inline]
            fn parent_on_property_set(&mut self, name: &str, value: &Value) {
                self.#super_field.on_property_set(name, value)
            }
        }

        impl #impl_generics ObjectOperation for #name #ty_generics #where_clause {
            #[inline]
            fn id(&self) -> ObjectId {
                self.#(#object_path).*.id()
            }

            #[inline]
            fn set_property(&mut self, name: &str, value: Value) {
                if !self.inner_on_property_set(name, &value) {
                    self.on_property_set(name, &value);
                }
                self.#(#object_path).*.set_property(name, value)
            }

            #[inline]
            fn get_property(&self, name: &str) -> Option<&Value> {
                self.#(#object_path).*.get_property(name)
            }

            #[inline]
            fn constructed(&self) -> bool {
                self.#(#object_path).*.constructed()
            }

            #[inline]
            fn set_signal_source(&mut self, id: Option<ObjectId>) {
                self.#(#object_path).*.set_signal_source(id)
            }

            #[inline]
            fn get_signal_source(&self) -> Option<ObjectId> {
                self.#(#object_path).*.get_signal_source()
            }
        }

        impl #impl_generics ObjectType for #name #ty_generics #where_clause {
            #[inline]
            fn object_type(&self) -> Type {
                Self::static_type()
            }
        }

        impl #impl_generics AsAny for #name #ty_generics #where_clause {
            #[inline]
            fn as_any(&self) -> &dyn Any {
                self
            }

            #[inline]
            fn as_any_mut(&mut self) -> &mut dyn Any {
                self
            }

            #[inline]
            fn as_any_boxed(self: Box<Self>) -> Box<dyn Any> {
                self
            }

        }

        impl #impl_generics Reflect for #name #ty_generics #where_clause {
            #[inline]
            fn as_reflect(&self) -> &dyn Reflect {
                self
            }

            #[inline]
            fn as_reflect_mut(&mut self) -> &mut dyn Reflect {
                self
            }

            #[inline]
            fn as_reflect_boxed(self: Box<Self>) -> Box<dyn Reflect> {
                self
            }
        }

        impl #impl_generics ActionExt for #name #ty_generics #where_clause {}

        impl #impl_generics IsA<Object> for #name #ty_generics #where_clause {}

        impl #impl_generics IsA<#name #ty_generics> for #name #ty_generics #where_clause {}
    ))
}
