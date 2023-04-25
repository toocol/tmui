use crate::extend_element;
use crate::extend_object;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse::Parser, DeriveInput};

pub(crate) fn generate_extend_widget(
    ast: &mut DeriveInput,
) -> syn::Result<proc_macro2::TokenStream> {
    let name = &ast.ident;
    match &mut ast.data {
        syn::Data::Struct(ref mut struct_data) => {
            match &mut struct_data.fields {
                syn::Fields::Named(fields) => {
                    fields.named.push(syn::Field::parse_named.parse2(quote! {
                        pub widget: Widget
                    })?);
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        ast,
                        "`extend_widget` should defined on named fields struct.",
                    ))
                }
            }

            let object_trait_impl_clause = extend_object::gen_object_trait_impl_clause(
                name,
                "widget",
                vec!["widget", "element", "object"],
            )?;

            let element_trait_impl_clause =
                extend_element::gen_element_trait_impl_clause(name, vec!["widget", "element"])?;

            let widget_trait_impl_clause = gen_widget_trait_impl_clause(name, vec!["widget"])?;

            Ok(quote! {
                #ast

                #object_trait_impl_clause

                #element_trait_impl_clause

                #widget_trait_impl_clause

                impl WidgetAcquire for #name {}

                impl ParentType for #name {
                    #[inline]
                    fn parent_type(&self) -> Type {
                        Widget::static_type()
                    }
                }

                impl InnerTypeRegister for #name {
                    #[inline]
                    fn inner_type_register(&self, type_registry: &mut TypeRegistry) {
                        type_registry.register::<#name, ReflectWidgetImpl>();
                    }
                }
            })
        }
        _ => Err(syn::Error::new_spanned(
            ast,
            "`extends_widget` has to be used with structs ",
        )),
    }
}

pub(crate) fn gen_widget_trait_impl_clause(
    name: &Ident,
    widget_path: Vec<&'static str>,
) -> syn::Result<proc_macro2::TokenStream> {
    let widget_path: Vec<_> = widget_path
        .iter()
        .map(|s| Ident::new(s, name.span()))
        .collect();

    Ok(quote!(
        impl WidgetExt for #name {
            #[inline]
            fn as_element(&mut self) -> *mut dyn ElementImpl {
                self as *mut Self as *mut dyn ElementImpl
            }

            #[inline]
            fn set_parent(&mut self, parent: *mut dyn WidgetImpl) {
                self.#(#widget_path).*.set_parent(parent)
            }

            #[inline]
            fn get_raw_child(&self) -> Option<*const dyn WidgetImpl> {
                self.#(#widget_path).*.get_raw_child()
            }

            #[inline]
            fn get_raw_child_mut(&mut self) -> Option<*mut dyn WidgetImpl> {
                self.#(#widget_path).*.get_raw_child_mut()
            }

            #[inline]
            fn get_raw_parent(&self) -> Option<*const dyn WidgetImpl> {
                self.#(#widget_path).*.get_raw_parent()
            }

            #[inline]
            fn get_raw_parent_mut(&mut self) -> Option<*mut dyn WidgetImpl> {
                self.#(#widget_path).*.get_raw_parent_mut()
            }

            #[inline]
            fn hide(&mut self) {
                self.#(#widget_path).*.hide()
            }

            #[inline]
            fn show(&mut self) {
                self.#(#widget_path).*.show()
            }

            #[inline]
            fn visible(&mut self) -> bool {
                self.#(#widget_path).*.visible()
            }

            #[inline]
            fn set_focus(&mut self, focus: bool) {
                self.#(#widget_path).*.set_focus(focus)
            }

            #[inline]
            fn is_focus(&self) -> bool {
                self.#(#widget_path).*.is_focus()
            }

            #[inline]
            fn resize(&mut self, width: i32, height: i32) {
                self.#(#widget_path).*.resize(width, height)
            }

            #[inline]
            fn width_request(&mut self, width: i32) {
                self.#(#widget_path).*.width_request(width)
            }

            #[inline]
            fn height_request(&mut self, width: i32) {
                self.#(#widget_path).*.height_request(width)
            }

            #[inline]
            fn set_halign(&mut self, halign: Align) {
                self.set_property("halign", halign.to_value())
            }

            #[inline]
            fn set_valign(&mut self, valign: Align) {
                self.set_property("valign", valign.to_value())
            }

            #[inline]
            fn halign(&self) -> Align {
                self.#(#widget_path).*.halign()
            }

            #[inline]
            fn valign(&self) -> Align {
                self.#(#widget_path).*.valign()
            }

            #[inline]
            fn set_font(&mut self, font: Font) {
                self.#(#widget_path).*.set_font(font);
                self.font_changed()
            }

            #[inline]
            fn font(&self) -> Font {
                self.#(#widget_path).*.font()
            }

            #[inline]
            fn set_font_family(&mut self, family: String) {
                self.#(#widget_path).*.set_font_family(family)
            }

            #[inline]
            fn font_family(&self) -> &str {
                self.#(#widget_path).*.font_family()
            }

            #[inline]
            fn size(&self) -> Size {
                self.#(#widget_path).*.size()
            }

            #[inline]
            fn image_rect(&self) -> Rect {
                self.#(#widget_path).*.image_rect()
            }

            #[inline]
            fn origin_rect(&self, coord: Option<Coordinate>) -> Rect {
                self.#(#widget_path).*.origin_rect(coord)
            }

            #[inline]
            fn contents_rect(&self, coord: Option<Coordinate>) -> Rect {
                self.#(#widget_path).*.contents_rect(coord)
            }

            #[inline]
            fn background(&self) -> Color {
                self.#(#widget_path).*.background()
            }

            #[inline]
            fn set_background(&mut self, color: Color) {
                self.#(#widget_path).*.set_background(color)
            }

            #[inline]
            fn margins(&self) -> (i32, i32, i32, i32) {
                self.#(#widget_path).*.margins()
            }

            #[inline]
            fn margin_top(&self) -> i32 {
                self.#(#widget_path).*.margin_top()
            }

            #[inline]
            fn margin_right(&self) -> i32 {
                self.#(#widget_path).*.margin_right()
            }

            #[inline]
            fn margin_bottom(&self) -> i32 {
                self.#(#widget_path).*.margin_bottom()
            }

            #[inline]
            fn margin_left(&self) -> i32 {
                self.#(#widget_path).*.margin_left()
            }

            #[inline]
            fn set_margins(&mut self, top: i32, right: i32, bottom: i32, left: i32) {
                self.#(#widget_path).*.set_margins(top, right, bottom, left)
            }

            #[inline]
            fn set_margin_top(&mut self, val: i32) {
                self.#(#widget_path).*.set_margin_top(val)
            }

            #[inline]
            fn set_margin_right(&mut self, val: i32) {
                self.#(#widget_path).*.set_margin_right(val)
            }

            #[inline]
            fn set_margin_bottom(&mut self, val: i32) {
                self.#(#widget_path).*.set_margin_bottom(val)
            }

            #[inline]
            fn set_margin_left(&mut self, val: i32) {
                self.#(#widget_path).*.set_margin_left(val)
            }

            #[inline]
            fn paddings(&self) -> (i32, i32, i32, i32) {
                self.#(#widget_path).*.paddings()
            }

            #[inline]
            fn padding_top(&self) -> i32 {
                self.#(#widget_path).*.padding_top()
            }

            #[inline]
            fn padding_right(&self) -> i32 {
                self.#(#widget_path).*.padding_right()
            }

            #[inline]
            fn padding_bottom(&self) -> i32 {
                self.#(#widget_path).*.padding_bottom()
            }

            #[inline]
            fn padding_left(&self) -> i32 {
                self.#(#widget_path).*.padding_left()
            }

            #[inline]
            fn set_paddings(&mut self, top: i32, right: i32, bottom: i32, left: i32) {
                self.#(#widget_path).*.set_paddings(top, right, bottom, left)
            }

            #[inline]
            fn set_padding_top(&mut self, val: i32) {
                self.#(#widget_path).*.set_padding_top(val)
            }

            #[inline]
            fn set_padding_right(&mut self, val: i32) {
                self.#(#widget_path).*.set_padding_right(val)
            }

            #[inline]
            fn set_padding_bottom(&mut self, val: i32) {
                self.#(#widget_path).*.set_padding_bottom(val)
            }

            #[inline]
            fn set_padding_left(&mut self, val: i32) {
                self.#(#widget_path).*.set_padding_left(val)
            }

            #[inline]
            fn set_borders(&mut self, top: f32, right: f32, bottom: f32, left: f32) {
                self.#(#widget_path).*.set_borders(top, right, bottom, left)
            }

            #[inline]
            fn set_border_style(&mut self, style: BorderStyle) {
                self.#(#widget_path).*.set_border_style(style)
            }

            #[inline]
            fn set_border_color(&mut self, color: Color) {
                self.#(#widget_path).*.set_border_color(color)
            }

            #[inline]
            fn borders(&self) -> [f32; 4] {
                self.#(#widget_path).*.borders()
            }

            #[inline]
            fn border_style(&self) -> BorderStyle {
                self.#(#widget_path).*.border_style()
            }

            #[inline]
            fn border_color(&self) -> Color {
                self.#(#widget_path).*.border_color()
            }

            #[inline]
            fn set_cursor_shape(&mut self, cursor: SystemCursorShape) {
                self.#(#widget_path).*.set_cursor_shape(cursor)
            }
        }

        impl WidgetImplExt for #name {
            #[inline]
            fn child<T: WidgetImpl + ElementImpl + IsA<Widget>>(&mut self, child: T) {
                if self.parent_type().is_a(Container::static_type()) {
                    panic!("function `child()` was invalid in `Container`")
                }
                self.#(#widget_path).*.child_internal(child)
            }
        }

        impl IsA<Widget> for #name {}
    ))
}