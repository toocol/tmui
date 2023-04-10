use quote::quote;
use syn::{DeriveInput, parse::Parser};

pub fn generate_extend_element(ast: &mut DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    match &mut ast.data {
        syn::Data::Struct(ref mut struct_data) => {
            match &mut struct_data.fields {
                syn::Fields::Named(fields) => {
                    fields.named.push(
                        syn::Field::parse_named
                            .parse2(quote! {
                                pub element: Element
                            })
                            .expect("Add field `element: Element` failed"),
                    );
                }
                _ => (),
            }

            return quote! {
                #ast

                impl ObjectImplExt for #name {
                    #[inline]
                    fn parent_construct(&mut self) {
                        self.element.construct()
                    }

                    #[inline]
                    fn parent_on_property_set(&self, name: &str, value: &Value) {
                        self.element.on_property_set(name, value)
                    }
                }

                impl ObjectOperation for #name {
                    #[inline]
                    fn id(&self) -> u16 {
                        self.element.object.id()
                    }

                    #[inline]
                    fn set_property(&self, name: &str, value: Value) {
                        self.on_property_set(name, &value);

                        self.element.object.set_property(name, value)
                    }

                    #[inline]
                    fn get_property(&self, name: &str) -> Option<std::cell::Ref<Box<Value>>> {
                        self.element.object.get_property(name)
                    }
                }

                impl ElementExt for #name {
                    #[inline]
                    fn update(&self) {
                        self.set_property("invalidate", true.to_value());
                    }

                    #[inline]
                    fn force_update(&self) {
                        self.set_property("invalidate", true.to_value());
                    }

                    #[inline]
                    fn rect(&self) -> Rect {
                        self.element.rect()
                    }

                    #[inline]
                    fn set_fixed_width(&self, width: i32) {
                        self.element.set_fixed_width(width)
                    }

                    #[inline]
                    fn set_fixed_height(&self, height: i32) {
                        self.element.set_fixed_height(height)
                    }

                    #[inline]
                    fn set_fixed_x(&self, x: i32) {
                        self.element.set_fixed_x(x)
                    }

                    #[inline]
                    fn set_fixed_y(&self, y: i32) {
                        self.element.set_fixed_y(y)
                    }

                    #[inline]
                    fn invalidate(&self) -> bool {
                        match self.get_property("invalidate") {
                            Some(invalidate) => invalidate.get::<bool>(),
                            None => false
                        }
                    }

                    #[inline]
                    fn validate(&self) {
                        self.set_property("invalidate", false.to_value());
                    }
                }

                impl ElementAcquire for #name {}

                impl ActionExt for #name {}

                impl ObjectType for #name {
                    #[inline]
                    fn object_type(&self) -> Type {
                        Self::static_type()
                    }
                }

                impl IsA<Object> for #name {}

                impl IsA<Element> for #name {}

                impl IsA<#name> for #name {}
            }
            .into();
        }
        _ => panic!("`extends_object` has to be used with structs "),
    }
}