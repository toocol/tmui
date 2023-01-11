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
                    fn parent_construct(&self) {
                        self.object.construct()
                    }
                }

                impl ObjectOperation for #name {
                    fn id(&self) -> u16 {
                        self.object.id()
                    }

                    fn set_property(&self, name: &str, value: Value) {
                        self.object.on_property_set(name, &value);
                        self.object.set_property(name, value)
                    }

                    fn get_property(&self, name: &str) -> Option<std::cell::Ref<Box<Value>>> {
                        self.object.get_property(name)
                    }
                }

                impl ActionExt for #name {}

                impl ObjectType for #name {
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
                    fn parent_construct(&self) {
                        self.element.construct()
                    }
                }

                impl ObjectOperation for #name {
                    fn id(&self) -> u16 {
                        self.element.object.id()
                    }

                    fn set_property(&self, name: &str, value: Value) {
                        self.element.object.on_property_set(name, &value);
                        self.element.on_property_set(name, &value);

                        self.element.object.set_property(name, value)
                    }

                    fn get_property(&self, name: &str) -> Option<std::cell::Ref<Box<Value>>> {
                        self.element.object.get_property(name)
                    }
                }

                impl ElementExt for #name {
                    fn update(&self) {
                        self.element.update()
                    }

                    fn force_update(&self) {
                        self.element.force_update()
                    }

                    fn rect(&self) -> Ref<Rect> {
                        self.element.rect()
                    }

                    fn set_fixed_width(&self, width: i32) {
                        self.element.set_fixed_width(width)
                    }

                    fn set_fixed_height(&self, height: i32) {
                        self.element.set_fixed_height(height)
                    }

                    fn set_fixed_x(&self, x: i32) {
                        self.element.set_fixed_x(x)
                    }

                    fn set_fixed_y(&self, y: i32) {
                        self.element.set_fixed_y(y)
                    }

                    fn invalidate(&self) -> bool {
                        self.element.invalidate()
                    }

                    fn validate(&self) {
                        self.element.validate()
                    }
                }

                impl ElementAcquire for #name {}

                impl ActionExt for #name {}

                impl ObjectType for #name {
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
                    fn parent_construct(&self) {
                        self.widget.construct()
                    }
                }

                impl ObjectOperation for #name {
                    fn id(&self) -> u16 {
                        self.widget.element.object.id()
                    }

                    fn set_property(&self, name: &str, value: Value) {
                        self.widget.element.object.on_property_set(name, &value);
                        self.widget.element.on_property_set(name, &value);
                        self.widget.on_property_set(name, &value);

                        self.widget.element.object.set_property(name, value)
                    }

                    fn get_property(&self, name: &str) -> Option<std::cell::Ref<Box<Value>>> {
                        self.widget.element.object.get_property(name)
                    }
                }

                impl ElementExt for #name {
                    fn update(&self) {
                        self.widget.element.update()
                    }

                    fn force_update(&self) {
                        self.widget.element.force_update()
                    }

                    fn rect(&self) -> Ref<Rect> {
                        self.widget.element.rect()
                    }

                    fn set_fixed_width(&self, width: i32) {
                        self.widget.element.set_fixed_width(width)
                    }

                    fn set_fixed_height(&self, height: i32) {
                        self.widget.element.set_fixed_height(height)
                    }

                    fn set_fixed_x(&self, x: i32) {
                        self.widget.element.set_fixed_x(x)
                    }

                    fn set_fixed_y(&self, y: i32) {
                        self.widget.element.set_fixed_y(y)
                    }

                    fn invalidate(&self) -> bool {
                        self.widget.element.invalidate()
                    }

                    fn validate(&self) {
                        self.widget.element.validate()
                    }
                }

                impl WidgetExt for #name {
                    fn set_parent(&self, parent: *const dyn WidgetImpl) {
                        self.widget.set_parent(parent)
                    }

                    fn get_raw_child(&self) -> Option<*const dyn WidgetImpl> {
                        self.widget.get_raw_child()
                    }

                    fn get_raw_parent(&self) -> Option<*const dyn WidgetImpl> {
                        self.widget.get_raw_parent()
                    }

                    fn width_request(&self, width: i32) {
                        self.widget.width_request(width)
                    }

                    fn height_request(&self, width: i32) {
                        self.widget.height_request(width)
                    }
                }

                impl WidgetImplExt for #name {
                    fn child<T: WidgetImpl + ElementImpl + IsA<Widget>>(&self, child: T) {
                        self.widget.child_internal(self, child)
                    }
                }

                impl WidgetAcquire for #name {}

                impl ActionExt for #name {}

                impl ObjectType for #name {
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
