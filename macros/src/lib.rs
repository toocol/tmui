extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse::Parser, parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn extends_object(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);
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
                    fn parent_construct(&self) {
                        self.object.construct()
                    }
                }

                impl ObjectOperation for #name {
                    #[inline]
                    fn id(&self) -> u16 {
                        self.object.id()
                    }

                    #[inline]
                    fn set_property(&self, name: &str, value: Value) {
                        self.object.on_property_set(name, &value);
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

#[proc_macro_attribute]
pub fn extends_element(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);
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
                    fn parent_construct(&self) {
                        self.element.construct()
                    }
                }

                impl ObjectOperation for #name {
                    #[inline]
                    fn id(&self) -> u16 {
                        self.element.object.id()
                    }

                    #[inline]
                    fn set_property(&self, name: &str, value: Value) {
                        self.element.object.on_property_set(name, &value);
                        self.element.on_property_set(name, &value);

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
                    fn rect(&self) -> Ref<Rect> {
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

#[proc_macro_attribute]
pub fn extends_widget(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    match &mut ast.data {
        syn::Data::Struct(ref mut struct_data) => {
            match &mut struct_data.fields {
                syn::Fields::Named(fields) => {
                    fields.named.push(
                        syn::Field::parse_named
                            .parse2(quote! {
                                pub widget: Widget 
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
                    fn parent_construct(&self) {
                        self.widget.construct()
                    }
                }

                impl ObjectOperation for #name {
                    #[inline]
                    fn id(&self) -> u16 {
                        self.widget.element.object.id()
                    }

                    #[inline]
                    fn set_property(&self, name: &str, value: Value) {
                        self.widget.element.object.on_property_set(name, &value);
                        self.widget.element.on_property_set(name, &value);
                        self.widget.on_property_set(name, &value);

                        self.widget.element.object.set_property(name, value)
                    }

                    #[inline]
                    fn get_property(&self, name: &str) -> Option<std::cell::Ref<Box<Value>>> {
                        self.widget.element.object.get_property(name)
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
                    fn rect(&self) -> Ref<Rect> {
                        self.widget.element.rect()
                    }

                    #[inline]
                    fn set_fixed_width(&self, width: i32) {
                        self.widget.element.set_fixed_width(width)
                    }

                    #[inline]
                    fn set_fixed_height(&self, height: i32) {
                        self.widget.element.set_fixed_height(height)
                    }

                    #[inline]
                    fn set_fixed_x(&self, x: i32) {
                        self.widget.element.set_fixed_x(x)
                    }

                    #[inline]
                    fn set_fixed_y(&self, y: i32) {
                        self.widget.element.set_fixed_y(y)
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

                impl WidgetExt for #name {
                    #[inline]
                    fn set_parent(&self, parent: *const dyn WidgetImpl) {
                        self.widget.set_parent(parent)
                    }

                    #[inline]
                    fn get_raw_child(&self) -> Option<*const dyn WidgetImpl> {
                        self.widget.get_raw_child()
                    }

                    #[inline]
                    fn get_raw_parent(&self) -> Option<*const dyn WidgetImpl> {
                        self.widget.get_raw_parent()
                    }

                    #[inline]
                    fn width_request(&self, width: i32) {
                        self.widget.width_request(width)
                    }

                    #[inline]
                    fn height_request(&self, width: i32) {
                        self.widget.height_request(width)
                    }

                    #[inline]
                    fn notify_invalidate(&self) {
                        self.widget.notify_invalidate()
                    }

                    #[inline]
                    fn set_halign(&self, halign: Align) {
                        self.set_property("halign", halign.to_value())
                    }

                    #[inline]
                    fn set_valign(&self, valign: Align) {
                        self.set_property("halign", valign.to_value())
                    }
                }

                impl WidgetImplExt for #name {
                    #[inline]
                    fn child<T: WidgetImpl + ElementImpl + IsA<Widget>>(&self, child: T) {
                        self.widget.child_internal(self, child)
                    }
                }

                impl WidgetAcquire for #name {}

                impl ActionExt for #name {}

                impl ObjectType for #name {
                    #[inline]
                    fn object_type(&self) -> Type {
                        Self::static_type()
                    }
                }

                impl IsA<Object> for #name {}

                impl IsA<Element> for #name {}

                impl IsA<Widget> for #name {}

                impl IsA<#name> for #name {}
            }
            .into();
        }
        _ => panic!("`extends_object` has to be used with structs "),
    }
}
