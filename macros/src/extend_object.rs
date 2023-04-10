use quote::quote;
use syn::{DeriveInput, parse::Parser};

pub fn generate_extend_object(ast: &mut DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    match &mut ast.data {
        syn::Data::Struct(ref mut struct_data) => {
            match &mut struct_data.fields {
                syn::Fields::Named(fields) => {
                    fields.named.push(
                        syn::Field::parse_named
                            .parse2(quote! {
                                pub object: Object
                            })
                            .unwrap(),
                    );
                }
                _ => (),
            }

            return quote! {
                #ast

                impl ObjectImplExt for #name {
                    #[inline]
                    fn parent_construct(&mut self) {
                        self.object.construct()
                    }

                    #[inline]
                    fn parent_on_property_set(&self, name: &str, value: &Value) {
                        self.object.on_property_set(name, value)
                    }
                }

                impl ObjectOperation for #name {
                    #[inline]
                    fn id(&self) -> u16 {
                        self.object.id()
                    }

                    #[inline]
                    fn set_property(&self, name: &str, value: Value) {
                        self.on_property_set(name, &value);
                        self.object.set_property(name, value)
                    }

                    #[inline]
                    fn get_property(&self, name: &str) -> Option<std::cell::Ref<Box<Value>>> {
                        self.object.get_property(name)
                    }
                }

                impl ActionExt for #name {}

                impl ObjectType for #name {
                    #[inline]
                    fn object_type(&self) -> Type {
                        Self::static_type()
                    }
                }

                impl IsA<Object> for #name {}

                impl IsA<#name> for #name {}
            }
            .into();
        }
        _ => panic!("`extends_object` has to be used with structs "),
    }
}