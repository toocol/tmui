use proc_macro2::Ident;
use quote::quote;
use syn::{parse::Parser, DeriveInput};

pub fn generate_extend_object(ast: &mut DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &ast.ident;
    match &mut ast.data {
        syn::Data::Struct(ref mut struct_data) => {
            match &mut struct_data.fields {
                syn::Fields::Named(fields) => {
                    fields.named.push(syn::Field::parse_named.parse2(quote! {
                        pub object: Object
                    })?);
                }
                _ => (),
            }

            let object_trait_impl_clause =
                gen_object_trait_impl_clause(name, "object", vec!["object"])?;

            return Ok(quote! {
                #ast

                #object_trait_impl_clause

                impl ParentType for #name {
                    fn parent_type(&self) -> Type {
                        Object::static_type()
                    }
                }
            });
        }
        _ => Err(syn::Error::new_spanned(
            ast,
            "`extends_object` has to be used with structs ",
        )),
    }
}

pub fn gen_object_trait_impl_clause(
    name: &Ident,
    super_field: &'static str,
    object_path: Vec<&'static str>,
) -> syn::Result<proc_macro2::TokenStream> {
    let super_field = Ident::new(super_field, name.span());
    let object_path: Vec<_> = object_path
        .iter()
        .map(|s| Ident::new(s, name.span()))
        .collect();

    Ok(quote!(
        impl ObjectImplExt for #name {
            #[inline]
            fn parent_construct(&mut self) {
                self.#super_field.construct()
            }

            #[inline]
            fn parent_on_property_set(&mut self, name: &str, value: &Value) {
                self.#super_field.on_property_set(name, value)
            }
        }

        impl ObjectOperation for #name {
            #[inline]
            fn id(&self) -> u16 {
                self.#(#object_path).*.id()
            }

            #[inline]
            fn set_property(&mut self, name: &str, value: Value) {
                self.on_property_set(name, &value);
                self.#(#object_path).*.set_property(name, value)
            }

            #[inline]
            fn get_property(&self, name: &str) -> Option<&Value> {
                self.#(#object_path).*.get_property(name)
            }
        }

        impl ObjectType for #name {
            #[inline]
            fn object_type(&self) -> Type {
                Self::static_type()
            }
        }

        impl ActionExt for #name {}

        impl IsA<Object> for #name {}

        impl IsA<#name> for #name {}
    ))
}
