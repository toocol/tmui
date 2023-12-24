use crate::{
    childable::Childable, extend_element, extend_object, general_attr::GeneralAttr, layout,
};
use proc_macro2::Ident;
use quote::quote;
use syn::{parse::Parser, DeriveInput, Meta};

pub(crate) fn expand(ast: &mut DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &ast.ident;

    let general_attr = GeneralAttr::parse(ast)?;

    let run_after_clause = &general_attr.run_after_clause;

    let animation_clause = &general_attr.animation_clause;
    let animation_reflect = &general_attr.animation_reflect;
    let animation_state_holder_field = &general_attr.animation_state_holder_field;
    let animation_state_holder_impl = &general_attr.animation_state_holder_impl;
    let animation_state_holder_reflect = &general_attr.animation_state_holder_reflect;

    let async_task_clause = &general_attr.async_task_impl_clause;
    let async_method_clause = &general_attr.async_task_method_clause;

    let popupable_impl_clause = &general_attr.popupable_impl_clause;
    let popupable_reflect_clause = &general_attr.popupable_reflect_clause;

    match &mut ast.data {
        syn::Data::Struct(ref mut struct_data) => {
            let mut childable = Childable::new();

            match &mut struct_data.fields {
                syn::Fields::Named(fields) => {
                    fields.named.push(syn::Field::parse_named.parse2(quote! {
                        pub widget: Widget
                    })?);

                    if general_attr.is_animation {
                        let default = general_attr.animation.as_ref().unwrap().parse_default()?;
                        let field = &general_attr.animation_field;
                        fields.named.push(syn::Field::parse_named.parse2(quote! {
                            #default
                            #field
                        })?);
                        fields.named.push(syn::Field::parse_named.parse2(quote! {
                            #animation_state_holder_field
                        })?);
                    }

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

                    childable.parse_childable(fields)?;
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        ast,
                        "`extends(Widget)` should defined on named fields struct.",
                    ))
                }
            }

            let object_trait_impl_clause = extend_object::gen_object_trait_impl_clause(
                name,
                "widget",
                vec!["widget", "element", "object"],
                false,
            )?;

            let element_trait_impl_clause =
                extend_element::gen_element_trait_impl_clause(name, vec!["widget", "element"])?;

            let widget_trait_impl_clause =
                gen_widget_trait_impl_clause(name, Some("widget"), vec!["widget"])?;

            let child_ref_clause = childable.get_child_ref();

            Ok(quote! {
                #[derive(Derivative)]
                #[derivative(Default)]
                #ast

                #object_trait_impl_clause

                #element_trait_impl_clause

                #widget_trait_impl_clause

                #animation_clause
                #animation_state_holder_impl

                #async_task_clause

                #popupable_impl_clause

                impl WidgetAcquire for #name {}

                impl SuperType for #name {
                    #[inline]
                    fn super_type(&self) -> Type {
                        Widget::static_type()
                    }
                }

                impl InnerInitializer for #name {
                    #[inline]
                    fn inner_type_register(&self, type_registry: &mut TypeRegistry) {
                        type_registry.register::<#name, ReflectWidgetImpl>();
                        #popupable_reflect_clause
                        #animation_reflect
                        #animation_state_holder_reflect
                    }

                    #[inline]
                    fn inner_initialize(&mut self) {
                        #run_after_clause
                    }

                    #[inline]
                    fn pretreat_construct(&mut self) {
                        #child_ref_clause
                    }
                }

                impl PointEffective for #name {
                    #[inline]
                    fn point_effective(&self, point: &Point) -> bool {
                        self.widget.point_effective(point)
                    }
                }

                impl ChildRegionAcquirer for #name {
                    #[inline]
                    fn child_region(&self) -> tlib::skia_safe::Region {
                        self.widget.child_region()
                    }
                }

                impl #name {
                    #async_method_clause
                }
            })
        }
        _ => Err(syn::Error::new_spanned(
            ast,
            "`extends(Widget)` has to be used with structs ",
        )),
    }
}

pub(crate) fn expand_with_layout(
    ast: &mut DeriveInput,
    layout_meta: &Meta,
    layout: &str,
    internal: bool,
) -> syn::Result<proc_macro2::TokenStream> {
    layout::expand(ast, layout_meta, layout, internal)
}

pub(crate) fn gen_widget_trait_impl_clause(
    name: &Ident,
    super_field: Option<&'static str>,
    widget_path: Vec<&'static str>,
) -> syn::Result<proc_macro2::TokenStream> {
    let widget_path: Vec<_> = widget_path
        .iter()
        .map(|s| Ident::new(s, name.span()))
        .collect();

    let parent_run_after = match super_field {
        Some(super_field) => {
            let super_field = Ident::new(super_field, name.span());
            quote!(
                self.#super_field.run_after()
            )
        }
        None => proc_macro2::TokenStream::new(),
    };

    Ok(quote!(
        impl WidgetExt for #name {
            #[inline]
            fn name(&self) -> String {
                self.#(#widget_path).*.name()
            }

            #[inline]
            fn initialized(&self) -> bool {
                self.#(#widget_path).*.initialized()
            }

            #[inline]
            fn set_initialized(&mut self, initialized: bool) {
                self.#(#widget_path).*.set_initialized(initialized)
            }

            #[inline]
            fn as_element(&mut self) -> &mut dyn ElementImpl {
                self
            }

            #[inline]
            fn first_rendered(&self) -> bool {
                self.#(#widget_path).*.first_rendered()
            }

            #[inline]
            fn set_first_rendered(&mut self) {
                self.#(#widget_path).*.set_first_rendered()
            }

            #[inline]
            fn rerender_styles(&self) -> bool {
                self.#(#widget_path).*.rerender_styles()
            }

            #[inline]
            fn set_rerender_styles(&mut self, rerender: bool) {
                self.#(#widget_path).*.set_rerender_styles(rerender)
            }

            #[inline]
            fn rerender_difference(&self) -> bool {
                self.#(#widget_path).*.rerender_difference()
            }

            #[inline]
            fn set_rerender_difference(&mut self, rerender_difference: bool) {
                self.#(#widget_path).*.set_rerender_difference(rerender_difference)
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
            fn get_child_ref(&self) -> Option<&dyn WidgetImpl> {
                self.#(#widget_path).*.get_child_ref()
            }

            #[inline]
            fn get_child_mut(&mut self) -> Option<&mut dyn WidgetImpl> {
                self.#(#widget_path).*.get_child_mut()
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
            fn get_parent_ref(&self) -> Option<&dyn WidgetImpl> {
                self.#(#widget_path).*.get_parent_ref()
            }

            #[inline]
            fn get_parent_mut(&mut self) -> Option<&mut dyn WidgetImpl> {
                self.#(#widget_path).*.get_parent_mut()
            }

            #[inline]
            fn hide(&mut self) {
                if let Some(snapshot) = cast_mut!(self as Snapshot) {
                    snapshot.start(false);
                }

                self.#(#widget_path).*.hide();
            }

            #[inline]
            fn show(&mut self) {
                if let Some(snapshot) = cast_mut!(self as Snapshot) {
                    snapshot.start(true);
                }

                self.#(#widget_path).*.show();
            }

            #[inline]
            fn visible(&self) -> bool {
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
            fn resize(&mut self, width: Option<i32>, height: Option<i32>) {
                self.#(#widget_path).*.resize(width, height);
                emit!(#name::resize => self.size_changed(), self.size());
            }

            #[inline]
            fn width_request(&mut self, width: i32) {
                self.#(#widget_path).*.width_request(width)
            }

            #[inline]
            fn height_request(&mut self, height: i32) {
                self.#(#widget_path).*.height_request(height)
            }

            #[inline]
            fn get_width_request(&self) -> i32 {
                self.#(#widget_path).*.get_width_request()
            }

            #[inline]
            fn get_height_request(&self) -> i32 {
                self.#(#widget_path).*.get_height_request()
            }

            #[inline]
            fn update_geometry(&mut self) {
                self.#(#widget_path).*.update_geometry()
            }

            #[inline]
            fn fixed_width(&self) -> bool {
                self.#(#widget_path).*.fixed_width()
            }

            #[inline]
            fn fixed_height(&self) -> bool {
                self.#(#widget_path).*.fixed_height()
            }

            #[inline]
            fn fixed_width_ration(&self) -> f32 {
                self.#(#widget_path).*.fixed_width_ration()
            }

            #[inline]
            fn fixed_height_ration(&self) -> f32 {
                self.#(#widget_path).*.fixed_height_ration()
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
            fn font(&self) -> &Font {
                self.#(#widget_path).*.font()
            }

            #[inline]
            fn font_mut(&mut self) -> &mut Font {
                self.#(#widget_path).*.font_mut()
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
            fn image_rect_f(&self) -> FRect {
                self.#(#widget_path).*.image_rect_f()
            }

            #[inline]
            fn origin_rect(&self, coord: Option<Coordinate>) -> Rect {
                self.#(#widget_path).*.origin_rect(coord)
            }

            #[inline]
            fn origin_rect_f(&self, coord: Option<Coordinate>) -> FRect {
                self.#(#widget_path).*.origin_rect_f(coord)
            }

            #[inline]
            fn contents_rect(&self, coord: Option<Coordinate>) -> Rect {
                self.#(#widget_path).*.contents_rect(coord)
            }

            #[inline]
            fn contents_rect_f(&self, coord: Option<Coordinate>) -> FRect {
                self.#(#widget_path).*.contents_rect_f(coord)
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
            fn border_ref(&self) -> &Border {
                self.#(#widget_path).*.border_ref()
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
            fn set_border_top_color(&mut self, color: Color) {
                self.#(#widget_path).*.set_border_top_color(color)
            }

            #[inline]
            fn set_border_right_color(&mut self, color: Color) {
                self.#(#widget_path).*.set_border_right_color(color)
            }

            #[inline]
            fn set_border_bottom_color(&mut self, color: Color) {
                self.#(#widget_path).*.set_border_bottom_color(color)
            }

            #[inline]
            fn set_border_left_color(&mut self, color: Color) {
                self.#(#widget_path).*.set_border_left_color(color)
            }

            #[inline]
            fn borders(&self) -> (f32, f32, f32, f32) {
                self.#(#widget_path).*.borders()
            }

            #[inline]
            fn border_style(&self) -> BorderStyle {
                self.#(#widget_path).*.border_style()
            }

            #[inline]
            fn border_color(&self) -> (Color, Color, Color, Color) {
                self.#(#widget_path).*.border_color()
            }

            #[inline]
            fn set_cursor_shape(&mut self, cursor: SystemCursorShape) {
                self.#(#widget_path).*.set_cursor_shape(cursor)
            }

            #[inline]
            fn map_to_global(&self, point: &Point) -> Point {
                self.#(#widget_path).*.map_to_global(point)
            }

            #[inline]
            fn map_to_widget(&self, point: &Point) -> Point {
                self.#(#widget_path).*.map_to_widget(point)
            }

            #[inline]
            fn map_to_global_f(&self, point: &FPoint) -> FPoint {
                self.#(#widget_path).*.map_to_global_f(point)
            }

            #[inline]
            fn map_to_widget_f(&self, point: &FPoint) -> FPoint {
                self.#(#widget_path).*.map_to_widget_f(point)
            }

            #[inline]
            fn mouse_tracking(&self) -> bool {
                self.#(#widget_path).*.mouse_tracking()
            }

            #[inline]
            fn set_mouse_tracking(&mut self, is_tracking: bool) {
                self.#(#widget_path).*.set_mouse_tracking(is_tracking)
            }

            #[inline]
            fn parent_run_after(&mut self) {
                #parent_run_after
            }

            #[inline]
            fn hexpand(&self) -> bool {
                self.#(#widget_path).*.hexpand()
            }

            #[inline]
            fn set_hexpand(&mut self, hexpand: bool) {
                self.#(#widget_path).*.set_hexpand(hexpand)
            }

            #[inline]
            fn vexpand(&self) -> bool {
                self.#(#widget_path).*.vexpand()
            }

            #[inline]
            fn set_vexpand(&mut self, vexpand: bool) {
                self.#(#widget_path).*.set_vexpand(vexpand)
            }

            #[inline]
            fn hscale(&self) -> f32 {
                self.#(#widget_path).*.hscale()
            }

            #[inline]
            fn set_hscale(&mut self, hscale: f32) {
                self.#(#widget_path).*.set_hscale(hscale)
            }

            #[inline]
            fn vscale(&self) -> f32 {
                self.#(#widget_path).*.vscale()
            }

            #[inline]
            fn set_vscale(&mut self, vscale: f32) {
                self.#(#widget_path).*.set_vscale(vscale)
            }

            #[inline]
            fn child_image_rect_union(&self) -> &Rect {
                self.#(#widget_path).*.child_image_rect_union()
            }

            #[inline]
            fn child_image_rect_union_mut(&mut self) -> &mut Rect {
                self.#(#widget_path).*.child_image_rect_union_mut()
            }

            #[inline]
            fn need_update_geometry(&self) -> bool {
                self.#(#widget_path).*.need_update_geometry()
            }

            #[inline]
            fn child_overflow_rect(&self) -> &Rect {
                self.#(#widget_path).*.child_overflow_rect()
            }

            #[inline]
            fn child_overflow_rect_mut(&mut self) -> &mut Rect {
                self.#(#widget_path).*.child_overflow_rect_mut()
            }

            #[inline]
            fn image_rect_record(&self) -> Rect {
                self.#(#widget_path).*.image_rect_record()
            }

            #[inline]
            fn set_image_rect_record(&mut self, image_rect: Rect) {
                self.#(#widget_path).*.set_image_rect_record(image_rect)
            }

            #[inline]
            fn minimized(&self) -> bool {
                self.#(#widget_path).*.minimized()
            }

            #[inline]
            fn set_minimized(&mut self, minimized: bool) {
                self.#(#widget_path).*.set_minimized(minimized)
            }

            #[inline]
            fn repaint_when_resize(&self) -> bool {
                self.#(#widget_path).*.repaint_when_resize()
            }

            #[inline]
            fn set_repaint_when_resize(&mut self, repaint: bool) {
                self.#(#widget_path).*.set_repaint_when_resize(repaint)
            }

            #[inline]
            fn is_pressed(&self) -> bool {
                self.#(#widget_path).*.is_pressed()
            }

            #[inline]
            fn propagate_update(&mut self) {
                self.#(#widget_path).*.propagate_update()
            }

            #[inline]
            fn propagate_update_rect(&mut self, rect: Rect) {
                self.#(#widget_path).*.propagate_update_rect(rect)
            }

            #[inline]
            fn propagate_update_rect_f(&mut self, rect: FRect) {
                self.#(#widget_path).*.propagate_update_rect_f(rect)
            }

            #[inline]
            fn propagate_update_global_rect(&mut self, rect: Rect) {
                self.#(#widget_path).*.propagate_update_global_rect(rect)
            }

            #[inline]
            fn propagate_update_global_rect_f(&mut self, rect: FRect) {
                self.#(#widget_path).*.propagate_update_global_rect_f(rect)
            }

            #[inline]
            fn descendant_of(&self, id: ObjectId) -> bool {
                self.#(#widget_path).*.descendant_of(id)
            }

            #[inline]
            fn propagate_animation_progressing(&mut self, is: bool) {
                self.#(#widget_path).*.propagate_animation_progressing(is)
            }

            #[inline]
            fn is_animation_progressing(&self) -> bool {
                self.#(#widget_path).*.is_animation_progressing()
            }

            #[inline]
            fn transparency(&self) -> Transparency {
                self.#(#widget_path).*.transparency()
            }

            #[inline]
            fn set_transparency(&mut self, transparency: Transparency) {
                self.#(#widget_path).*.set_transparency(transparency)
            }

            #[inline]
            fn propagate_set_transparency(&mut self, transparency: Transparency) {
                self.#(#widget_path).*.propagate_set_transparency(transparency)
            }
        }

        impl WidgetImplExt for #name {
            #[inline]
            fn child<T: WidgetImpl>(&mut self, child: Box<T>) {
                if self.super_type().is_a(Container::static_type()) {
                    panic!("function `child()` was invalid in `Container`")
                }
                self.#(#widget_path).*.child_internal(child)
            }

            #[inline]
            fn _child_ref(&mut self, child: *mut dyn WidgetImpl) {
                if self.super_type().is_a(Container::static_type()) {
                    panic!("function `child()` was invalid in `Container`")
                }
                self.#(#widget_path).*.child_ref_internal(child)
            }
        }

        impl IsA<Widget> for #name {}
    ))
}
