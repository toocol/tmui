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
                    fn parent_construct(&mut self) {
                        self.widget.construct()
                    }

                    #[inline]
                    fn parent_on_property_set(&self, name: &str, value: &Value) {
                        self.widget.on_property_set(name, value)
                    }
                }

                impl ObjectOperation for #name {
                    #[inline]
                    fn id(&self) -> u16 {
                        self.widget.element.object.id()
                    }

                    #[inline]
                    fn set_property(&self, name: &str, value: Value) {
                        self.on_property_set(name, &value);

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
                    fn rect(&self) -> Rect {
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
                    fn as_element(&mut self) -> *mut dyn ElementImpl {
                        self as *mut Self as *mut dyn ElementImpl
                    }

                    #[inline]
                    fn set_parent(&self, parent: *const dyn WidgetImpl) {
                        self.widget.set_parent(parent)
                    }

                    #[inline]
                    fn get_raw_child(&self) -> Option<*const dyn WidgetImpl> {
                        self.widget.get_raw_child()
                    }

                    #[inline]
                    fn get_raw_child_mut(&self) -> Option<*mut dyn WidgetImpl> {
                        self.widget.get_raw_child_mut()
                    }

                    #[inline]
                    fn get_raw_parent(&self) -> Option<*const dyn WidgetImpl> {
                        self.widget.get_raw_parent()
                    }

                    #[inline]
                    fn hide(&self) {
                        self.widget.hide()
                    }

                    #[inline]
                    fn show(&self) {
                        self.widget.show()
                    }

                    #[inline]
                    fn visible(&self) -> bool {
                        self.widget.visible()
                    }

                    #[inline]
                    fn set_focus(&self, focus: bool) {
                        self.widget.set_focus(focus)
                    }

                    #[inline]
                    fn is_focus(&self) -> bool {
                        self.widget.is_focus()
                    }

                    #[inline]
                    fn resize(&self, width: i32, height: i32) {
                        self.widget.resize(width, height)
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
                    fn set_halign(&self, halign: Align) {
                        self.set_property("halign", halign.to_value())
                    }

                    #[inline]
                    fn set_valign(&self, valign: Align) {
                        self.set_property("valign", valign.to_value())
                    }

                    #[inline]
                    fn halign(&self) -> Align {
                        self.widget.halign()
                    }

                    #[inline]
                    fn valign(&self) -> Align {
                        self.widget.valign()
                    }

                    #[inline]
                    fn set_font(&mut self, font: Font) {
                        self.widget.set_font(font);
                        self.font_changed()
                    }

                    #[inline]
                    fn font(&self) -> Font {
                        self.widget.font()
                    }

                    #[inline]
                    fn set_font_family(&mut self, family: String) {
                        self.widget.set_font_family(family)
                    }

                    #[inline]
                    fn font_family(&self) -> &str {
                        self.widget.font_family()
                    }

                    #[inline]
                    fn size(&self) -> Size {
                        self.widget.size()
                    }

                    #[inline]
                    fn image_rect(&self) -> Rect {
                        self.widget.image_rect()
                    }

                    #[inline]
                    fn origin_rect(&self, coord: Option<Coordinate>) -> Rect {
                        self.widget.origin_rect(coord)
                    }

                    #[inline]
                    fn contents_rect(&self, coord: Option<Coordinate>) -> Rect {
                        self.widget.contents_rect(coord)
                    }

                    #[inline]
                    fn background(&self) -> Color {
                        self.widget.background()
                    }

                    #[inline]
                    fn set_background(&mut self, color: Color) {
                        self.widget.set_background(color)
                    }

                    #[inline]
                    fn margins(&self) -> (i32, i32, i32, i32) {
                        self.widget.margins()
                    }

                    #[inline]
                    fn margin_top(&self) -> i32 {
                        self.widget.margin_top()
                    }

                    #[inline]
                    fn margin_right(&self) -> i32 {
                        self.widget.margin_right()
                    }

                    #[inline]
                    fn margin_bottom(&self) -> i32 {
                        self.widget.margin_bottom()
                    }

                    #[inline]
                    fn margin_left(&self) -> i32 {
                        self.widget.margin_left()
                    }

                    #[inline]
                    fn set_margins(&mut self, top: i32, right: i32, bottom: i32, left: i32) {
                        self.widget.set_margins(top, right, bottom, left)
                    }

                    #[inline]
                    fn set_margin_top(&mut self, val: i32) {
                        self.widget.set_margin_top(val)
                    }

                    #[inline]
                    fn set_margin_right(&mut self, val: i32) {
                        self.widget.set_margin_right(val)
                    }

                    #[inline]
                    fn set_margin_bottom(&mut self, val: i32) {
                        self.widget.set_margin_bottom(val)
                    }

                    #[inline]
                    fn set_margin_left(&mut self, val: i32) {
                        self.widget.set_margin_left(val)
                    }

                    #[inline]
                    fn paddings(&self) -> (i32, i32, i32, i32) {
                        self.widget.paddings()
                    }

                    #[inline]
                    fn padding_top(&self) -> i32 {
                        self.widget.padding_top()
                    }

                    #[inline]
                    fn padding_right(&self) -> i32 {
                        self.widget.padding_right()
                    }

                    #[inline]
                    fn padding_bottom(&self) -> i32 {
                        self.widget.padding_bottom()
                    }

                    #[inline]
                    fn padding_left(&self) -> i32 {
                        self.widget.padding_left()
                    }

                    #[inline]
                    fn set_paddings(&mut self, top: i32, right: i32, bottom: i32, left: i32) {
                        self.widget.set_paddings(top, right, bottom, left)
                    }

                    #[inline]
                    fn set_padding_top(&mut self, val: i32) {
                        self.widget.set_padding_top(val)
                    }

                    #[inline]
                    fn set_padding_right(&mut self, val: i32) {
                        self.widget.set_padding_right(val)
                    }

                    #[inline]
                    fn set_padding_bottom(&mut self, val: i32) {
                        self.widget.set_padding_bottom(val)
                    }

                    #[inline]
                    fn set_padding_left(&mut self, val: i32) {
                        self.widget.set_padding_left(val)
                    }

                    #[inline]
                    fn set_borders(&mut self, top: f32, right: f32, bottom: f32, left: f32) {
                        self.widget.set_borders(top, right, bottom, left)
                    }

                    #[inline]
                    fn set_border_style(&mut self, style: BorderStyle) {
                        self.widget.set_border_style(style)
                    }

                    #[inline]
                    fn set_border_color(&mut self, color: Color) {
                        self.widget.set_border_color(color)
                    }

                    #[inline]
                    fn borders(&self) -> [f32; 4] {
                        self.widget.borders()
                    }

                    #[inline]
                    fn border_style(&self) -> BorderStyle {
                        self.widget.border_style()
                    }

                    #[inline]
                    fn border_color(&self) -> Color {
                        self.widget.border_color()
                    }

                    #[inline]
                    fn set_cursor_shape(&mut self, cursor: SystemCursorShape) {
                        self.widget.set_cursor_shape(cursor)
                    }
                }

                impl WidgetImplExt for #name {
                    #[inline]
                    fn child<T: WidgetImpl + ElementImpl + IsA<Widget>>(&self, child: T) {
                        self.widget.child_internal(child)
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
