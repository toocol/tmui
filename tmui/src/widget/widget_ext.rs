use super::{
    EventBubble, Font, ReflectSpacingCapable, SizeHint, Transparency, Widget, WidgetImpl,
    WindowAcquire,
};
use crate::{
    application_window::ApplicationWindow,
    font::FontTypeface,
    graphics::{
        border::Border,
        element::{ElementExt, ElementImpl},
    },
    primitive::Message,
    widget::WidgetSignals,
};
use std::ptr::NonNull;
use tlib::{
    figure::{Color, CoordRect, FPoint, FRect, Point, Rect, Size},
    namespace::{Align, BorderStyle, Coordinate, SystemCursorShape},
    object::{ObjectId, ObjectOperation},
    prelude::*,
    ptr_mut,
};

////////////////////////////////////// WidgetExt //////////////////////////////////////
/// The extended actions of [`Widget`], impl by proc-macro [`extends_widget`] automaticly.
pub trait WidgetExt {
    /// Get the ref of widget model.
    ///
    /// Go to[`Function defination`](WidgetExt::widget_model) (Defined in [`WidgetExt`])
    fn widget_props(&self) -> &Widget;

    /// Get the mutable ref of widget model.
    ///
    /// Go to[`Function defination`](WidgetExt::widget_model) (Defined in [`WidgetExt`])
    fn widget_props_mut(&mut self) -> &mut Widget;

    /// Go to[`Function defination`](WidgetExt::name) (Defined in [`WidgetExt`])
    fn name(&self) -> String;

    /// Go to[`Function defination`](WidgetExt::initialized) (Defined in [`WidgetExt`])
    fn initialized(&self) -> bool;

    /// Go to[`Function defination`](WidgetExt::set_initialized) (Defined in [`WidgetExt`])
    fn set_initialized(&mut self, initialized: bool);

    /// Go to[`Function defination`](WidgetExt::as_element) (Defined in [`WidgetExt`])
    fn as_element(&mut self) -> &mut dyn ElementImpl;

    /// Go to[`Function defination`](WidgetExt::first_rendered) (Defined in [`WidgetExt`])
    fn first_rendered(&self) -> bool;

    /// Go to[`Function defination`](WidgetExt::set_first_rendered) (Defined in [`WidgetExt`])
    fn set_first_rendered(&mut self);

    /// Go to[`Function defination`](WidgetExt::rerender_styles) (Defined in [`WidgetExt`])
    fn rerender_styles(&self) -> bool;

    /// Go to[`Function defination`](WidgetExt::set_rerender_styles) (Defined in [`WidgetExt`])
    fn set_rerender_styles(&mut self, rerender: bool);

    /// Go to[`Function defination`](WidgetExt::rerender_difference) (Defined in [`WidgetExt`])
    fn rerender_difference(&self) -> bool;

    /// Go to[`Function defination`](WidgetExt::set_rerender_difference) (Defined in [`WidgetExt`])
    fn set_rerender_difference(&mut self, rerender_difference: bool);

    /// ## Do not invoke this function directly.
    ///
    /// Go to[`Function defination`](WidgetExt::set_parent) (Defined in [`WidgetExt`])
    fn set_parent(&mut self, parent: *mut dyn WidgetImpl);

    /// Get the raw pointer of child.
    ///
    /// Go to[`Function defination`](WidgetExt::get_raw_child) (Defined in [`WidgetExt`])
    fn get_raw_child(&self) -> Option<*const dyn WidgetImpl>;

    /// Get the raw mut pointer of child.
    ///
    /// Go to[`Function defination`](WidgetExt::get_raw_child_mut) (Defined in [`WidgetExt`])
    fn get_raw_child_mut(&mut self) -> Option<*mut dyn WidgetImpl>;

    /// Get the ref of child.
    ///
    /// Go to[`Function defination`](WidgetExt::get_child_ref) (Defined in [`WidgetExt`])
    fn get_child_ref(&self) -> Option<&dyn WidgetImpl>;

    /// Get the ref mut of child.
    ///
    /// Go to[`Function defination`](WidgetExt::get_child_mut) (Defined in [`WidgetExt`])
    fn get_child_mut(&mut self) -> Option<&mut dyn WidgetImpl>;

    /// Get the raw pointer of parent.
    ///
    /// Go to[`Function defination`](WidgetExt::get_raw_parent) (Defined in [`WidgetExt`])
    fn get_raw_parent(&self) -> Option<*const dyn WidgetImpl>;

    /// Get the raw mut pointer of parent.
    ///
    /// Go to[`Function defination`](WidgetExt::get_raw_parent) (Defined in [`WidgetExt`])
    fn get_raw_parent_mut(&mut self) -> Option<*mut dyn WidgetImpl>;

    /// Get the ref of parent.
    ///
    /// Go to[`Function defination`](WidgetExt::get_child_ref) (Defined in [`WidgetExt`])
    fn get_parent_ref(&self) -> Option<&dyn WidgetImpl>;

    /// Get the ref mut of child.
    ///
    /// Go to[`Function defination`](WidgetExt::get_parent_mut) (Defined in [`WidgetExt`])
    fn get_parent_mut(&mut self) -> Option<&mut dyn WidgetImpl>;

    /// Hide the Widget.
    ///
    /// Go to[`Function defination`](WidgetExt::hide) (Defined in [`WidgetExt`])
    fn hide(&mut self);

    /// Show the Widget.
    ///
    /// Go to[`Function defination`](WidgetExt::show) (Defined in [`WidgetExt`])
    fn show(&mut self);

    /// Return true if widget is visble, otherwise, false is returned.
    ///
    /// Go to[`Function defination`](WidgetExt::visible) (Defined in [`WidgetExt`])
    fn visible(&self) -> bool;

    /// Setter of property `focus`. <br>
    /// Only effected after phase `run_after`.
    ///
    /// Go to[`Function defination`](WidgetExt::set_focus) (Defined in [`WidgetExt`])
    fn set_focus(&mut self, focus: bool);

    /// Getter of property `focus`.
    ///
    /// Go to[`Function defination`](WidgetExt::is_focus) (Defined in [`WidgetExt`])
    fn is_focus(&self) -> bool;

    /// Resize the widget. <br>
    /// `resize() will set fixed_width and fixed_height to false`, make widget flexible.
    ///
    /// Go to[`Function defination`](WidgetExt::resize) (Defined in [`WidgetExt`])
    fn resize(&mut self, width: Option<i32>, height: Option<i32>);

    /// Request the widget's width. <br>
    /// This function should be used in construct phase of the ui component,
    /// the function will not change the layout and will not trigger the signal `size_changed()`.
    ///
    /// Go to[`Function defination`](WidgetExt::width_request) (Defined in [`WidgetExt`])
    fn width_request(&mut self, width: i32);

    /// Request the widget's width. <br>
    /// This function should be used in construct phase of the ui component,
    /// the function will not change the layout and will not trigger the signal `size_changed()`.
    ///
    /// Go to[`Function defination`](WidgetExt::height_request) (Defined in [`WidgetExt`])
    fn height_request(&mut self, width: i32);

    /// Go to[`Function defination`](WidgetExt::get_width_request) (Defined in [`WidgetExt`])
    fn get_width_request(&self) -> i32;

    /// Go to[`Function defination`](WidgetExt::get_height_request) (Defined in [`WidgetExt`])
    fn get_height_request(&self) -> i32;

    /// Update widget's child image rect union.
    ///
    /// Go to[`Function defination`](WidgetExt::update_geometry) (Defined in [`WidgetExt`])
    fn update_geometry(&mut self);

    /// Widget's width was fixed or not,
    /// `true` when user invoke [`width_request`](WidgetExt::width_request)
    ///
    /// Go to[`Function defination`](WidgetExt::fixed_width) (Defined in [`WidgetExt`])
    fn fixed_width(&self) -> bool;

    /// Widget's height was fixed or not,
    /// `true` when user invoke [`height_request`](WidgetExt::height_request)
    ///
    /// Go to[`Function defination`](WidgetExt::fixed_height) (Defined in [`WidgetExt`])
    fn fixed_height(&self) -> bool;

    /// Used in conjunction with the function [`hexpand`],
    /// if widget was width fixed and hexpanded, `the width ration = width / parent_width`
    ///
    /// Go to[`Function defination`](WidgetExt::fixed_width_ration) (Defined in [`WidgetExt`])
    fn fixed_width_ration(&self) -> f32;

    /// Used in conjunction with the function [`vexpand`],
    /// if widget was height fixed and vexpanded, `the height ration = height / parent_height`
    ///
    /// Go to[`Function defination`](WidgetExt::fixed_height_ration) (Defined in [`WidgetExt`])
    fn fixed_height_ration(&self) -> f32;

    /// Set alignment on the horizontal direction.
    ///
    /// Go to[`Function defination`](WidgetExt::set_halign) (Defined in [`WidgetExt`])
    fn set_halign(&mut self, halign: Align);

    /// Set alignment on the vertical direction.
    ///
    /// Go to[`Function defination`](WidgetExt::set_valign) (Defined in [`WidgetExt`])
    fn set_valign(&mut self, valign: Align);

    /// Get alignment on the horizontal direction.
    ///
    /// Go to[`Function defination`](WidgetExt::halign) (Defined in [`WidgetExt`])
    fn halign(&self) -> Align;

    /// Get alignment on the vertical direction.
    ///
    /// Go to[`Function defination`](WidgetExt::valign) (Defined in [`WidgetExt`])
    fn valign(&self) -> Align;

    /// Set the font of widget.
    ///
    /// Go to[`Function defination`](WidgetExt::set_font) (Defined in [`WidgetExt`])
    fn set_font(&mut self, font: Font);

    /// Get the font of widget.
    ///
    /// Go to[`Function defination`](WidgetExt::font) (Defined in [`WidgetExt`])
    fn font(&self) -> &Font;

    /// Get the mutable reference of font of widget.
    /// 
    /// Go to[`Function defination`](WidgetExt::font_mut) (Defined in [`WidgetExt`])
    fn font_mut(&mut self) -> &mut Font;

    /// Set the font family of Widget.
    ///
    /// Go to[`Function defination`](WidgetExt::set_font_families) (Defined in [`WidgetExt`])
    fn set_font_families(&mut self, families: &[&str]);

    /// Get the rect of widget without borders.
    ///
    /// Go to[`Function defination`](WidgetExt::borderless_rect) (Defined in [`WidgetExt`])
    fn borderless_rect(&self) -> FRect;

    /// Get the size of widget. The size does not include the margins.
    ///
    /// Go to[`Function defination`](WidgetExt::size) (Defined in [`WidgetExt`])
    fn size(&self) -> Size;

    /// Get the area of widget's total image Rect with the margins. <br>
    /// The [`Coordinate`] was `World`.
    ///
    /// Go to[`Function defination`](WidgetExt::image_rect) (Defined in [`WidgetExt`])
    fn image_rect(&self) -> Rect;

    /// Get the area of widget's total image Rect with the margins. <br>
    /// The [`Coordinate`] was `World`.
    ///
    /// Go to[`Function defination`](WidgetExt::image_rect_f) (Defined in [`WidgetExt`])
    fn image_rect_f(&self) -> FRect;

    /// Get the area of widget's origin Rect. <br>
    /// The default [`Coordinate`] was `World`.
    ///
    /// Go to[`Function defination`](WidgetExt::origin_rect) (Defined in [`WidgetExt`])
    fn origin_rect(&self, coord: Option<Coordinate>) -> Rect;

    /// Get the area of widget's origin Rect. <br>
    /// The default [`Coordinate`] was `World`.
    ///
    /// Go to[`Function defination`](WidgetExt::origin_rect_f) (Defined in [`WidgetExt`])
    fn origin_rect_f(&self, coord: Option<Coordinate>) -> FRect;

    /// Get the area inside the widget's paddings. <br>
    /// The default [`Coordinate`] was `World`.
    ///
    /// Go to[`Function defination`](WidgetExt::contents_rect) (Defined in [`WidgetExt`])
    fn contents_rect(&self, coord: Option<Coordinate>) -> Rect;

    /// Get the area inside the widget's paddings. <br>
    /// The default [`Coordinate`] was `World`.
    ///
    /// Go to[`Function defination`](WidgetExt::contents_rect) (Defined in [`WidgetExt`])
    fn contents_rect_f(&self, coord: Option<Coordinate>) -> FRect;

    /// Get the widget's background color.
    ///
    /// Go to[`Function defination`](WidgetExt::background) (Defined in [`WidgetExt`])
    fn background(&self) -> Color;

    /// Set the widget's background color.
    ///
    /// Go to[`Function defination`](WidgetExt::set_background) (Defined in [`WidgetExt`])
    fn set_background(&mut self, color: Color);

    /// Get the margins of the Widget. (top, right, bottom, left)
    ///
    /// Go to[`Function defination`](WidgetExt::margins) (Defined in [`WidgetExt`])
    fn margins(&self) -> (i32, i32, i32, i32);

    /// Get the top margin of the Widget.
    ///
    /// Go to[`Function defination`](WidgetExt::margin_top) (Defined in [`WidgetExt`])
    fn margin_top(&self) -> i32;

    /// Get the right margin of the Widget.
    ///
    /// Go to[`Function defination`](WidgetExt::margin_right) (Defined in [`WidgetExt`])
    fn margin_right(&self) -> i32;

    /// Get the bottom margin of the Widget.
    ///
    /// Go to[`Function defination`](WidgetExt::margin_bottom) (Defined in [`WidgetExt`])
    fn margin_bottom(&self) -> i32;

    /// Get the left margin of the Widget.
    ///
    /// Go to[`Function defination`](WidgetExt::margin_left) (Defined in [`WidgetExt`])
    fn margin_left(&self) -> i32;

    /// Set the margins of the Widget.
    ///
    /// Go to[`Function defination`](WidgetExt::set_margins) (Defined in [`WidgetExt`])
    fn set_margins(&mut self, top: i32, right: i32, bottom: i32, left: i32);

    /// Set the top margin of the Widget.
    ///
    /// Go to[`Function defination`](WidgetExt::set_margin_top) (Defined in [`WidgetExt`])
    fn set_margin_top(&mut self, val: i32);

    /// Set the right margin of the Widget.
    ///
    /// Go to[`Function defination`](WidgetExt::set_margin_right) (Defined in [`WidgetExt`])
    fn set_margin_right(&mut self, val: i32);

    /// Set the bottom margin of the Widget.
    ///
    /// Go to[`Function defination`](WidgetExt::set_margin_bottom) (Defined in [`WidgetExt`])
    fn set_margin_bottom(&mut self, val: i32);

    /// Set the left margin of the Widget.
    ///
    /// Go to[`Function defination`](WidgetExt::set_margin_left) (Defined in [`WidgetExt`])
    fn set_margin_left(&mut self, val: i32);

    /// Get the paddins of the Widget. (top, right, bottom, left)
    ///
    /// Go to[`Function defination`](WidgetExt::paddings) (Defined in [`WidgetExt`])
    fn paddings(&self) -> (i32, i32, i32, i32);

    /// Get the top padding of the Widget.
    ///
    /// Go to[`Function defination`](WidgetExt::padding_top) (Defined in [`WidgetExt`])
    fn padding_top(&self) -> i32;

    /// Get the right padding of the Widget.
    ///
    /// Go to[`Function defination`](WidgetExt::padding_right) (Defined in [`WidgetExt`])
    fn padding_right(&self) -> i32;

    /// Get the bottom padding of the Widget.
    ///
    /// Go to[`Function defination`](WidgetExt::padding_bottom) (Defined in [`WidgetExt`])
    fn padding_bottom(&self) -> i32;

    /// Get the left padding of the Widget.
    ///
    /// Go to[`Function defination`](WidgetExt::padding_left) (Defined in [`WidgetExt`])
    fn padding_left(&self) -> i32;

    /// Set the paddings of the Widget.
    ///
    /// Go to[`Function defination`](WidgetExt::set_paddings) (Defined in [`WidgetExt`])
    fn set_paddings(&mut self, top: i32, right: i32, bottom: i32, left: i32);

    /// Set the top padding of the Widget.
    ///
    /// Go to[`Function defination`](WidgetExt::set_padding_top) (Defined in [`WidgetExt`])
    fn set_padding_top(&mut self, val: i32);

    /// Set the right padding of the Widget.
    ///
    /// Go to[`Function defination`](WidgetExt::set_padding_right) (Defined in [`WidgetExt`])
    fn set_padding_right(&mut self, val: i32);

    /// Set the bottom padding of the Widget.
    ///
    /// Go to[`Function defination`](WidgetExt::set_padding_bottom) (Defined in [`WidgetExt`])
    fn set_padding_bottom(&mut self, val: i32);

    /// Set the left padding of the Widget.
    ///
    /// Go to[`Function defination`](WidgetExt::set_padding_left) (Defined in [`WidgetExt`])
    fn set_padding_left(&mut self, val: i32);

    /// Get the refrence of [`Border`].
    ///
    /// Go to[`Function defination`](WidgetExt::border_ref) (Defined in [`WidgetExt`])
    fn border_ref(&self) -> &Border;

    /// Set the borders of the widget.
    ///
    /// Go to[`Function defination`](WidgetExt::set_borders) (Defined in [`WidgetExt`])
    fn set_borders(&mut self, top: f32, right: f32, bottom: f32, left: f32);

    /// Set the border style of the widget.
    ///
    /// Go to[`Function defination`](WidgetExt::set_border_style) (Defined in [`WidgetExt`])
    fn set_border_style(&mut self, style: BorderStyle);

    /// Set the border color(all directions) of the widget.
    ///
    /// Go to[`Function defination`](WidgetExt::set_border_color) (Defined in [`WidgetExt`])
    fn set_border_color(&mut self, color: Color);

    /// Set the top border color of the widget.
    ///
    /// Go to[`Function defination`](WidgetExt::set_border_top_color) (Defined in [`WidgetExt`])
    fn set_border_top_color(&mut self, color: Color);

    /// Set the right border color of the widget.
    ///
    /// Go to[`Function defination`](WidgetExt::set_border_right_color) (Defined in [`WidgetExt`])
    fn set_border_right_color(&mut self, color: Color);

    /// Set the bottom border color of the widget.
    ///
    /// Go to[`Function defination`](WidgetExt::set_border_bottom_color) (Defined in [`WidgetExt`])
    fn set_border_bottom_color(&mut self, color: Color);

    /// Set the left border color of the widget.
    ///
    /// Go to[`Function defination`](WidgetExt::set_border_left_color) (Defined in [`WidgetExt`])
    fn set_border_left_color(&mut self, color: Color);

    /// Get the borders of the widget. <br>
    /// @return (top, right, bottom, left)
    ///
    /// Go to[`Function defination`](WidgetExt::borders) (Defined in [`WidgetExt`])
    fn borders(&self) -> (f32, f32, f32, f32);

    /// Get the border style of the widget.
    ///
    /// Go to[`Function defination`](WidgetExt::border_style) (Defined in [`WidgetExt`])
    fn border_style(&self) -> BorderStyle;

    /// Get the border color of the widget.
    ///
    /// Go to[`Function defination`](WidgetExt::border_color) (Defined in [`WidgetExt`])
    fn border_color(&self) -> (Color, Color, Color, Color);

    /// Set the system cursor shape.
    ///
    /// Go to[`Function defination`](WidgetExt::set_cursor_shape) (Defined in [`WidgetExt`])
    fn set_cursor_shape(&mut self, cursor: SystemCursorShape);

    /// Map the given point to global coordinate.
    ///
    /// Go to[`Function defination`](WidgetExt::map_to_global) (Defined in [`WidgetExt`])
    fn map_to_global(&self, point: &Point) -> Point;

    /// Map the given point to widget coordinate.
    ///
    /// Go to[`Function defination`](WidgetExt::map_to_widget) (Defined in [`WidgetExt`])
    fn map_to_widget(&self, point: &Point) -> Point;

    /// Map the given point to global coordinate.
    ///
    /// Go to[`Function defination`](WidgetExt::map_to_global_f) (Defined in [`WidgetExt`])
    fn map_to_global_f(&self, point: &FPoint) -> FPoint;

    /// Map the given point to widget coordinate.
    ///
    /// Go to[`Function defination`](WidgetExt::map_to_widget_f) (Defined in [`WidgetExt`])
    fn map_to_widget_f(&self, point: &FPoint) -> FPoint;

    /// The widget tracking the `MouseMoveEvent` or not.
    ///
    /// Go to[`Function defination`](WidgetExt::mouse_tracking) (Defined in [`WidgetExt`])
    fn mouse_tracking(&self) -> bool;

    /// Set the `mouse_tracking` status of widget.
    ///
    /// when `ture`, widget will track the movement of mouse.
    ///
    /// Go to[`Function defination`](WidgetExt::set_mouse_tracking) (Defined in [`WidgetExt`])
    fn set_mouse_tracking(&mut self, is_tracking: bool);

    /// Parent run after function.
    ///
    /// Go to[`Function defination`](WidgetExt::parent_run_after) (Defined in [`WidgetExt`])
    fn parent_run_after(&mut self);

    /// Get `hexpand` of widget.
    ///
    /// `hexpand`: Horizontal scalability, if `true` can cause child widget to expand horizontally
    /// with changes in the width of the parent widget.
    ///
    /// Go to[`Function defination`](WidgetExt::hexpand) (Defined in [`WidgetExt`])
    fn hexpand(&self) -> bool;

    /// Set `hexpand` of widget.
    ///
    /// `hexpand`: Horizontal scalability, if `true` can cause child widget to expand horizontally
    /// with changes in the width of the parent widget.
    ///
    /// Go to[`Function defination`](WidgetExt::set_hexpand) (Defined in [`WidgetExt`])
    fn set_hexpand(&mut self, hexpand: bool);

    /// Get `vexpand` of widget.
    ///
    /// `vexpand`: Vertical scalability, if `true` can cause child widget to expand vertically
    /// height changes in the height of the parent widget.
    ///
    /// Go to[`Function defination`](WidgetExt::vexpand) (Defined in [`WidgetExt`])
    fn vexpand(&self) -> bool;

    /// Set `vexpand` of widget.
    ///
    /// `vexpand`: Vertical scalability, if `true` can cause child widget to expand vertically
    /// height changes in the height of the parent widget.
    ///
    /// Go to[`Function defination`](WidgetExt::set_vexpand) (Defined in [`WidgetExt`])
    fn set_vexpand(&mut self, vexpand: bool);

    /// The scale factor on horizontal, ratio of child width to parent component,
    /// only effective when widget's `hexpand was true` and `fixed_width was false`.
    ///
    /// ### when parent was widget:
    /// `width ration = hsclae / 1`
    ///
    /// ### when parent was coontainer:
    /// `width ration = hscale / parent_children_total_hscales`
    ///
    /// Go to[`Function defination`](WidgetExt::hscale) (Defined in [`WidgetExt`])
    fn hscale(&self) -> f32;

    /// See [`hscale`](WidgetExt::hscale)
    ///
    /// Go to[`Function defination`](WidgetExt::set_hscale) (Defined in [`WidgetExt`])
    fn set_hscale(&mut self, hscale: f32);

    /// The scale factor on vertical, ratio of child height to parent component,
    /// only effective when widget's hexpand was true.
    ///
    /// ### when parent was widget:
    /// `height ration = vsclae / 1`
    ///
    /// ### when parent was coontainer:
    /// `height ration = vscale / parent_children_total_vscales`
    ///
    /// Go to[`Function defination`](WidgetExt::vscale) (Defined in [`WidgetExt`])
    fn vscale(&self) -> f32;

    /// See [`vscale`](WidgetExt::vscale)
    ///
    /// Go to[`Function defination`](WidgetExt::set_vscale) (Defined in [`WidgetExt`])
    fn set_vscale(&mut self, vscale: f32);

    /// Go to[`Function defination`](WidgetExt::child_image_rect_union) (Defined in [`WidgetExt`])
    fn child_image_rect_union(&self) -> &Rect;

    /// Go to[`Function defination`](WidgetExt::child_image_rect_union_mut) (Defined in [`WidgetExt`])
    fn child_image_rect_union_mut(&mut self) -> &mut Rect;

    /// Go to[`Function defination`](WidgetExt::need_update_geometry) (Defined in [`WidgetExt`])
    fn need_update_geometry(&self) -> bool;

    /// Go to[`Function defination`](WidgetExt::child_overflow_rect) (Defined in [`WidgetExt`])
    fn child_overflow_rect(&self) -> &Rect;

    /// Go to[`Function defination`](WidgetExt::child_overflow_rect_mut) (Defined in [`WidgetExt`])
    fn child_overflow_rect_mut(&mut self) -> &mut Rect;

    /// Go to[`Function defination`](WidgetExt::image_rect_record) (Defined in [`WidgetExt`])
    fn image_rect_record(&self) -> Rect;

    /// Go to[`Function defination`](WidgetExt::set_image_rect_record) (Defined in [`WidgetExt`])
    fn set_image_rect_record(&mut self, image_rect: Rect);

    /// Go to[`Function defination`](WidgetExt::minimized) (Defined in [`WidgetExt`])
    fn minimized(&self) -> bool;

    /// Go to[`Function defination`](WidgetExt::set_minimized) (Defined in [`WidgetExt`])
    fn set_minimized(&mut self, minimized: bool);

    /// Go to[`Function defination`](WidgetExt::repaint_when_resize) (Defined in [`WidgetExt`])
    fn repaint_when_resize(&self) -> bool;

    /// Go to[`Function defination`](WidgetExt::set_repaint_when_resize) (Defined in [`WidgetExt`])
    fn set_repaint_when_resize(&mut self, repaint: bool);

    /// Go to[`Function defination`](WidgetExt::is_pressed) (Defined in [`WidgetExt`])
    fn is_pressed(&self) -> bool;

    /// Invalidate this widget to update it, and also update the child widget..
    ///
    /// Go to[`Function defination`](WidgetExt::propagate_update) (Defined in [`WidgetExt`])
    fn propagate_update(&mut self);

    /// Invalidate this widget with dirty rect to update it, and also update the child widget..<br>
    /// This will result in clipping the drawing area of the widget.(after styles render)
    ///
    /// Go to[`Function defination`](WidgetExt::propagate_update_rect) (Defined in [`WidgetExt`])
    fn propagate_update_rect(&mut self, rect: CoordRect);

    /// Invalidate this widget with dirty styles rect to update it, and also update the child widget..<br>
    /// This will result in clipping the drawing area of the widget.(before styles render)
    ///
    /// Go to[`Function defination`](WidgetExt::propagate_update_rect) (Defined in [`WidgetExt`])
    fn propagate_update_styles_rect(&mut self, rect: CoordRect);

    /// Check if the widget is a descendant of the widget represented by the specified id.
    ///
    /// Go to[`Function defination`](WidgetExt::descendant_of) (Defined in [`WidgetExt`])
    fn descendant_of(&self, id: ObjectId) -> bool;

    /// Propagate setting the property `animation_progressing`
    ///
    /// Go to[`Function defination`](WidgetExt::propagate_animation_progressing) (Defined in [`WidgetExt`])
    fn propagate_animation_progressing(&mut self, is: bool);

    /// Go to[`Function defination`](WidgetExt::is_animation_progressing) (Defined in [`WidgetExt`])
    fn is_animation_progressing(&self) -> bool;

    /// Getting the transparency of widget.
    ///
    /// Go to[`Function defination`](WidgetExt::transparency) (Defined in [`WidgetExt`])
    fn transparency(&self) -> Transparency;

    /// Setting the transparency of widget.
    ///
    /// Go to[`Function defination`](WidgetExt::set_transparency) (Defined in [`WidgetExt`])
    fn set_transparency(&mut self, transparency: Transparency);

    /// Propagate setting the transparency of widget.
    ///
    /// Go to[`Function defination`](WidgetExt::propagate_set_transparency) (Defined in [`WidgetExt`])
    fn propagate_set_transparency(&mut self, transparency: Transparency);

    /// Get the size hint of widget.
    ///
    /// For specific information about size_hint, please refer to [`size_hint`](crate::widget::Widget::size_hint)
    ///
    /// Go to[`Function defination`](WidgetExt::size_hint) (Defined in [`WidgetExt`])
    fn size_hint(&self) -> SizeHint;

    /// Set the size hint of widget.
    ///
    /// For specific information about size_hint, please refer to [`size_hint`](crate::widget::Widget::size_hint)
    ///
    /// Go to[`Function defination`](WidgetExt::set_size_hint) (Defined in [`WidgetExt`])
    fn set_size_hint(&mut self, size_hint: SizeHint);

    /// Whether the event will be bubbled or not.
    ///
    /// Go to[`Function defination`](WidgetExt::event_bubbled) (Defined in [`WidgetExt`])
    fn is_event_bubbled(&self, event_bubble: EventBubble) -> bool;

    /// Enable the event bubble.
    ///
    /// Go to[`Function defination`](WidgetExt::enable_bubble) (Defined in [`WidgetExt`])
    fn enable_bubble(&mut self, event_bubble: EventBubble);

    /// Disable the event bubble.
    ///
    /// Go to[`Function defination`](WidgetExt::disable_bubble) (Defined in [`WidgetExt`])
    fn disable_bubble(&mut self, event_bubble: EventBubble);

    /// Get the value of [`propagate_event_bubble`](Widget::propagate_event_bubble).
    ///
    /// Go to[`Function defination`](WidgetExt::is_propagate_event_bubble) (Defined in [`WidgetExt`])
    fn is_propagate_event_bubble(&self) -> bool;

    /// Set the value of [`propagate_event_bubble`](Widget::propagate_event_bubble).
    ///
    /// Go to[`Function defination`](WidgetExt::set_propagate_event_bubble) (Defined in [`WidgetExt`])
    fn set_propagate_event_bubble(&mut self, is: bool);

    /// Get the value of [`propagate_mouse_tracking`](Widget::propagate_mouse_tracking).
    ///
    /// Go to[`Function defination`](WidgetExt::is_propagate_mouse_tracking) (Defined in [`WidgetExt`])
    fn is_propagate_mouse_tracking(&self) -> bool;

    /// Set the value of [`propagate_event_bubble`](Widget::propagate_event_bubble).
    ///
    /// Go to[`Function defination`](WidgetExt::set_propagate_mouse_tracking) (Defined in [`WidgetExt`])
    fn set_propagate_mouse_tracking(&mut self, is: bool);

    /// Get the value of [`strict_clip_widget`](Widget::strict_clip_widget).
    ///
    /// Go to[`Function defination`](WidgetExt::is_strict_clip_widget) (Defined in [`WidgetExt`])
    fn is_strict_clip_widget(&self) -> bool;

    /// Set the value of [`strict_clip_widget`](Widget::strict_clip_widget).
    ///
    /// Go to[`Function defination`](WidgetExt::set_strict_clip_widget) (Defined in [`WidgetExt`])
    fn set_strict_clip_widget(&mut self, strict_clip_widget: bool);

    /// Get the value of [`strict_clip_widget`](Widget::resize_redraw).
    ///
    /// Go to[`Function defination`](WidgetExt::is_resize_redraw) (Defined in [`WidgetExt`])
    fn is_resize_redraw(&self) -> bool;
}

impl WidgetExt for Widget {
    #[inline]
    fn widget_props(&self) -> &Widget {
        self
    }

    #[inline]
    fn widget_props_mut(&mut self) -> &mut Widget {
        self
    }

    #[inline]
    fn name(&self) -> String {
        self.get_property("name").unwrap().get::<String>()
    }

    #[inline]
    fn initialized(&self) -> bool {
        self.initialized
    }

    #[inline]
    fn set_initialized(&mut self, initialized: bool) {
        self.initialized = initialized
    }

    #[inline]
    fn as_element(&mut self) -> &mut dyn ElementImpl {
        self
    }

    #[inline]
    fn first_rendered(&self) -> bool {
        self.first_rendered
    }

    #[inline]
    fn set_first_rendered(&mut self) {
        self.first_rendered = true
    }

    #[inline]
    fn rerender_styles(&self) -> bool {
        match self.get_property("rerender_styles") {
            Some(val) => val.get::<bool>(),
            None => false,
        }
    }

    #[inline]
    fn set_rerender_styles(&mut self, rerender: bool) {
        self.set_property("rerender_styles", rerender.to_value())
    }

    #[inline]
    fn rerender_difference(&self) -> bool {
        self.rerender_difference
    }

    #[inline]
    fn set_rerender_difference(&mut self, rerender_difference: bool) {
        self.rerender_difference = rerender_difference
    }

    #[inline]
    fn set_parent(&mut self, parent: *mut dyn WidgetImpl) {
        self.parent = NonNull::new(parent)
    }

    #[inline]
    fn get_raw_child(&self) -> Option<*const dyn WidgetImpl> {
        let mut child = self.child.as_ref().map(|c| c.as_ref().as_ptr());

        if child.is_none() {
            unsafe {
                child = match self.child_ref {
                    Some(ref c) => Some(c.as_ref().as_ptr()),
                    None => None,
                }
            }
        }

        child
    }

    #[inline]
    fn get_raw_child_mut(&mut self) -> Option<*mut dyn WidgetImpl> {
        let mut child = self.child.as_mut().map(|c| c.as_mut().as_ptr_mut());

        if child.is_none() {
            unsafe {
                child = match self.child_ref {
                    Some(ref mut c) => Some(c.as_mut().as_ptr_mut()),
                    None => None,
                }
            }
        }

        child
    }

    #[inline]
    fn get_child_ref(&self) -> Option<&dyn WidgetImpl> {
        let mut child = self.child.as_ref().map(|c| c.as_ref());

        if child.is_none() {
            unsafe {
                child = match self.child_ref {
                    Some(ref c) => Some(c.as_ref()),
                    None => None,
                }
            }
        }

        child
    }

    #[inline]
    fn get_child_mut(&mut self) -> Option<&mut dyn WidgetImpl> {
        let mut child = self.child.as_mut().map(|c| c.as_mut());

        if child.is_none() {
            unsafe {
                child = match self.child_ref {
                    Some(ref mut c) => Some(c.as_mut()),
                    None => None,
                }
            }
        }

        child
    }

    #[inline]
    fn get_raw_parent(&self) -> Option<*const dyn WidgetImpl> {
        match self.parent.as_ref() {
            Some(parent) => Some(unsafe { parent.as_ref() }),
            None => None,
        }
    }

    #[inline]
    fn get_raw_parent_mut(&mut self) -> Option<*mut dyn WidgetImpl> {
        match self.parent.as_mut() {
            Some(parent) => Some(unsafe { parent.as_mut() }),
            None => None,
        }
    }

    #[inline]
    fn get_parent_ref(&self) -> Option<&dyn WidgetImpl> {
        match self.parent {
            Some(ref parent) => unsafe { Some(parent.as_ref()) },
            None => None,
        }
    }

    #[inline]
    fn get_parent_mut(&mut self) -> Option<&mut dyn WidgetImpl> {
        match self.parent {
            Some(ref mut parent) => unsafe { Some(parent.as_mut()) },
            None => None,
        }
    }

    #[inline]
    fn hide(&mut self) {
        self.set_property("visible", false.to_value());

        if !self.is_animation_progressing() {
            self.window()
                .invalid_effected_widgets(self.image_rect(), self.id());
        }
    }

    #[inline]
    fn show(&mut self) {
        self.set_property("visible", true.to_value());
        self.set_rerender_styles(true);
        self.update();
    }

    #[inline]
    fn visible(&self) -> bool {
        self.get_property("visible").unwrap().get::<bool>()
    }

    #[inline]
    fn set_focus(&mut self, focus: bool) {
        let id = if focus {
            if self.is_focus() {
                return;
            }

            self.id()
        } else {
            0
        };

        ApplicationWindow::window_of(self.window_id()).set_focused_widget(id)
    }

    #[inline]
    fn is_focus(&self) -> bool {
        ApplicationWindow::window_of(self.window_id()).focused_widget() == self.id()
    }

    #[inline]
    fn resize(&mut self, width: Option<i32>, height: Option<i32>) {
        if let Some(width) = width {
            self.set_property("width", width.to_value());
        }
        if let Some(height) = height {
            self.set_property("height", height.to_value());
        }
        if self.id() != self.window_id() {
            if !self.window().initialized() {
                return;
            }

            self.window().layout_change(self);
        }
    }

    #[inline]
    fn width_request(&mut self, width: i32) {
        let size_hint = self.size_hint();
        if let Some(min_width) = size_hint.min_width() {
            if width < min_width {
                return;
            }
        }
        if let Some(max_width) = size_hint.max_width() {
            if width > max_width {
                return;
            }
        }
        self.set_property("width", width.to_value());
        self.fixed_width = true;
        self.width_request = width;
        if let Some(parent) = self.get_parent_ref() {
            let parent_size = if let Some(s) = cast!(parent as SpacingCapable) {
                let mut size = parent.size();
                s.spacing_size().remove_spacing_from(&mut size);
                size
            } else {
                parent.size()
            };
            if parent_size.width() == 0 {
                return;
            }

            self.fixed_width_ration = width as f32 / parent_size.width() as f32;
        }
    }

    #[inline]
    fn height_request(&mut self, height: i32) {
        let size_hint = self.size_hint();
        if let Some(min_height) = size_hint.min_height() {
            if height < min_height {
                return;
            }
        }
        if let Some(max_height) = size_hint.max_height() {
            if height > max_height {
                return;
            }
        }
        self.set_property("height", height.to_value());
        self.fixed_height = true;
        self.height_request = height;
        if let Some(parent) = self.get_parent_ref() {
            let parent_size = if let Some(s) = cast!(parent as SpacingCapable) {
                let mut size = parent.size();
                s.spacing_size().remove_spacing_from(&mut size);
                size
            } else {
                parent.size()
            };
            if parent_size.height() == 0 {
                return;
            }

            self.fixed_height_ration = height as f32 / parent_size.height() as f32;
        }
    }

    #[inline]
    fn get_width_request(&self) -> i32 {
        self.width_request
    }

    #[inline]
    fn get_height_request(&self) -> i32 {
        self.height_request
    }

    #[inline]
    fn update_geometry(&mut self) {
        let mut widget = self as &mut dyn WidgetImpl as *mut dyn WidgetImpl;
        let mut parent = self.get_parent_mut();

        while let Some(parent_mut) = parent {
            let w = ptr_mut!(widget);

            parent_mut.child_image_rect_union_mut().or(&w.image_rect());

            widget = parent_mut;
            parent = w.get_parent_mut();
        }
    }

    #[inline]
    fn fixed_width(&self) -> bool {
        self.fixed_width
    }

    #[inline]
    fn fixed_height(&self) -> bool {
        self.fixed_height
    }

    #[inline]
    fn fixed_width_ration(&self) -> f32 {
        self.fixed_width_ration
    }

    #[inline]
    fn fixed_height_ration(&self) -> f32 {
        self.fixed_height_ration
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
        self.get_property("halign").unwrap().get::<Align>()
    }

    #[inline]
    fn valign(&self) -> Align {
        self.get_property("valign").unwrap().get::<Align>()
    }

    #[inline]
    fn set_font(&mut self, font: Font) {
        self.font = font;
    }

    #[inline]
    fn font(&self) -> &Font {
        &self.font
    }


    #[inline]
    fn font_mut(&mut self) -> &mut Font {
        &mut self.font
    }

    #[inline]
    fn set_font_families(&mut self, families: &[&str]) {
        let mut typefaces = vec![];
        for f in families {
            let typeface = FontTypeface::new(f);
            typefaces.push(typeface);
        }
        self.font.set_typefaces(typefaces);
        self.update()
    }

    #[inline]
    fn borderless_rect(&self) -> FRect {
        let mut rect: FRect = self.rect().into();
        let (top, right, bottom, left) = self.borders();

        rect.set_x(rect.x() + left);
        rect.set_y(rect.y() + top);
        rect.set_width(rect.width() - (left + right));
        rect.set_height(rect.height() - (top + bottom));

        rect
    }

    #[inline]
    fn size(&self) -> Size {
        let rect = self.rect();
        Size::new(rect.width(), rect.height())
    }

    #[inline]
    fn image_rect(&self) -> Rect {
        let mut rect = self.rect();

        let h_factor = if rect.width() == 0 { 0 } else { 1 };
        let v_factor = if rect.height() == 0 { 0 } else { 1 };

        // Rect add the margins.
        let (top, right, bottom, left) = self.margins();
        rect.set_x(rect.x() - left);
        rect.set_y(rect.y() - top);
        if rect.width() != 0 {
            rect.set_width((rect.width() + left + right) * h_factor);
        }
        if rect.height() != 0 {
            rect.set_height((rect.height() + top + bottom) * v_factor);
        }

        rect.or(self.child_image_rect_union());
        rect.or(self.child_overflow_rect());
        rect
    }

    #[inline]
    fn image_rect_f(&self) -> FRect {
        self.image_rect().into()
    }

    #[inline]
    fn origin_rect(&self, coord: Option<Coordinate>) -> Rect {
        let mut rect = self.rect();

        if let Some(coord) = coord {
            if coord == Coordinate::Widget {
                rect.set_x(0);
                rect.set_y(0);
            }
        }

        rect
    }

    #[inline]
    fn origin_rect_f(&self, coord: Option<Coordinate>) -> FRect {
        self.origin_rect(coord).into()
    }

    #[inline]
    fn contents_rect(&self, coord: Option<Coordinate>) -> Rect {
        let mut rect = self.rect();

        // Rect add the paddings.
        let (top, right, bottom, left) = self.paddings();
        rect.set_x(rect.x() + left);
        rect.set_y(rect.y() + top);
        rect.set_width(rect.width() - left - right);
        rect.set_height(rect.height() - top - bottom);

        if let Some(coord) = coord {
            if coord == Coordinate::Widget {
                rect.set_x(left);
                rect.set_y(top);
            }
        }

        rect
    }

    #[inline]
    fn contents_rect_f(&self, coord: Option<Coordinate>) -> FRect {
        self.contents_rect(coord).into()
    }

    #[inline]
    fn background(&self) -> Color {
        self.background
    }

    #[inline]
    fn set_background(&mut self, color: Color) {
        self.set_rerender_styles(true);
        self.background = color;
        emit!(Widget::set_background => self.background_changed(), color);
    }

    #[inline]
    fn margins(&self) -> (i32, i32, i32, i32) {
        (
            self.margins[0],
            self.margins[1],
            self.margins[2],
            self.margins[3],
        )
    }

    #[inline]
    fn margin_top(&self) -> i32 {
        self.margins[0]
    }

    #[inline]
    fn margin_right(&self) -> i32 {
        self.margins[1]
    }

    #[inline]
    fn margin_bottom(&self) -> i32 {
        self.margins[2]
    }

    #[inline]
    fn margin_left(&self) -> i32 {
        self.margins[3]
    }

    #[inline]
    fn set_margins(&mut self, top: i32, right: i32, bottom: i32, left: i32) {
        self.margins[0] = top;
        self.margins[1] = right;
        self.margins[2] = bottom;
        self.margins[3] = left;

        self.need_update_geometry = top != 0 || right != 0 || bottom != 0 || left != 0;
    }

    #[inline]
    fn set_margin_top(&mut self, val: i32) {
        self.margins[0] = val;

        self.need_update_geometry = val != 0;
    }

    #[inline]
    fn set_margin_right(&mut self, val: i32) {
        self.margins[1] = val;

        self.need_update_geometry = val != 0;
    }

    #[inline]
    fn set_margin_bottom(&mut self, val: i32) {
        self.margins[2] = val;

        self.need_update_geometry = val != 0;
    }

    #[inline]
    fn set_margin_left(&mut self, val: i32) {
        self.margins[3] = val;

        self.need_update_geometry = val != 0;
    }

    #[inline]
    fn paddings(&self) -> (i32, i32, i32, i32) {
        (
            self.paddings[0],
            self.paddings[1],
            self.paddings[2],
            self.paddings[3],
        )
    }

    #[inline]
    fn padding_top(&self) -> i32 {
        self.paddings[0]
    }

    #[inline]
    fn padding_right(&self) -> i32 {
        self.paddings[1]
    }

    #[inline]
    fn padding_bottom(&self) -> i32 {
        self.paddings[2]
    }

    #[inline]
    fn padding_left(&self) -> i32 {
        self.paddings[3]
    }

    #[inline]
    fn set_paddings(&mut self, mut top: i32, mut right: i32, mut bottom: i32, mut left: i32) {
        if top < 0 {
            top = 0;
        }
        if right < 0 {
            right = 0;
        }
        if bottom < 0 {
            bottom = 0;
        }
        if left < 0 {
            left = 0;
        }

        self.paddings[0] = top;
        self.paddings[1] = right;
        self.paddings[2] = bottom;
        self.paddings[3] = left;
    }

    #[inline]
    fn set_padding_top(&mut self, mut val: i32) {
        if val < 0 {
            val = 0;
        }
        self.paddings[0] = val;
    }

    #[inline]
    fn set_padding_right(&mut self, mut val: i32) {
        if val < 0 {
            val = 0;
        }
        self.paddings[1] = val;
    }

    #[inline]
    fn set_padding_bottom(&mut self, mut val: i32) {
        if val < 0 {
            val = 0;
        }
        self.paddings[2] = val;
    }

    #[inline]
    fn set_padding_left(&mut self, mut val: i32) {
        if val < 0 {
            val = 0;
        }
        self.paddings[3] = val;
    }

    #[inline]
    fn border_ref(&self) -> &Border {
        &self.border
    }

    #[inline]
    fn set_borders(&mut self, mut top: f32, mut right: f32, mut bottom: f32, mut left: f32) {
        if top < 0. {
            top = 0.;
        }
        if right < 0. {
            right = 0.;
        }
        if bottom < 0. {
            bottom = 0.;
        }
        if left < 0. {
            left = 0.;
        }
        self.border.width.0 = top;
        self.border.width.1 = right;
        self.border.width.2 = bottom;
        self.border.width.3 = left;
    }

    #[inline]
    fn set_border_style(&mut self, style: BorderStyle) {
        self.border.style = style;
    }

    #[inline]
    fn set_border_color(&mut self, color: Color) {
        self.border.border_color = (color, color, color, color);
    }

    #[inline]
    fn set_border_top_color(&mut self, color: Color) {
        self.border.border_color.0 = color;
    }

    #[inline]
    fn set_border_right_color(&mut self, color: Color) {
        self.border.border_color.1 = color;
    }

    #[inline]
    fn set_border_bottom_color(&mut self, color: Color) {
        self.border.border_color.2 = color;
    }

    #[inline]
    fn set_border_left_color(&mut self, color: Color) {
        self.border.border_color.3 = color;
    }

    #[inline]
    fn borders(&self) -> (f32, f32, f32, f32) {
        self.border.width
    }

    #[inline]
    fn border_style(&self) -> BorderStyle {
        self.border.style
    }

    #[inline]
    fn border_color(&self) -> (Color, Color, Color, Color) {
        self.border.border_color
    }

    #[inline]
    fn set_cursor_shape(&mut self, cursor: SystemCursorShape) {
        ApplicationWindow::send_message_with_id(self.window_id(), Message::SetCursorShape(cursor))
    }

    #[inline]
    fn map_to_global(&self, point: &Point) -> Point {
        let rect = self.rect();
        Point::new(point.x() + rect.x(), point.y() + rect.y())
    }

    #[inline]
    fn map_to_widget(&self, point: &Point) -> Point {
        let rect = self.rect();
        Point::new(point.x() - rect.x(), point.y() - rect.y())
    }

    #[inline]
    fn map_to_global_f(&self, point: &FPoint) -> FPoint {
        let rect = self.rect();
        FPoint::new(point.x() + rect.x() as f32, point.y() + rect.y() as f32)
    }

    #[inline]
    fn map_to_widget_f(&self, point: &FPoint) -> FPoint {
        let rect = self.rect();
        FPoint::new(point.x() - rect.x() as f32, point.y() - rect.y() as f32)
    }

    #[inline]
    fn mouse_tracking(&self) -> bool {
        if let Some(val) = self.get_property("mouse_tracking") {
            val.get::<bool>()
        } else {
            false
        }
    }

    #[inline]
    fn set_mouse_tracking(&mut self, is_tracking: bool) {
        self.set_property("mouse_tracking", is_tracking.to_value());
    }

    #[inline]
    fn parent_run_after(&mut self) {}

    #[inline]
    fn hexpand(&self) -> bool {
        self.hexpand
    }

    #[inline]
    fn set_hexpand(&mut self, hexpand: bool) {
        self.hexpand = hexpand
    }

    #[inline]
    fn vexpand(&self) -> bool {
        self.vexpand
    }

    #[inline]
    fn set_vexpand(&mut self, vexpand: bool) {
        self.vexpand = vexpand
    }

    #[inline]
    fn hscale(&self) -> f32 {
        if self.fixed_width {
            return 0.;
        }
        self.hscale
    }

    #[inline]
    fn set_hscale(&mut self, hscale: f32) {
        self.hscale = hscale
    }

    #[inline]
    fn vscale(&self) -> f32 {
        if self.fixed_height {
            return 0.;
        }
        self.vscale
    }

    #[inline]
    fn set_vscale(&mut self, vscale: f32) {
        self.vscale = vscale
    }

    #[inline]
    fn child_image_rect_union(&self) -> &Rect {
        &self.child_image_rect_union
    }

    #[inline]
    fn child_image_rect_union_mut(&mut self) -> &mut Rect {
        &mut self.child_image_rect_union
    }

    #[inline]
    fn need_update_geometry(&self) -> bool {
        self.need_update_geometry
    }

    #[inline]
    fn child_overflow_rect(&self) -> &Rect {
        &self.child_overflow_rect
    }

    #[inline]
    fn child_overflow_rect_mut(&mut self) -> &mut Rect {
        &mut self.child_overflow_rect
    }

    #[inline]
    fn image_rect_record(&self) -> Rect {
        self.old_image_rect
    }

    #[inline]
    fn set_image_rect_record(&mut self, image_rect: Rect) {
        self.old_image_rect = image_rect
    }

    #[inline]
    fn minimized(&self) -> bool {
        match self.get_property("minimized") {
            Some(val) => val.get::<bool>(),
            None => false,
        }
    }

    #[inline]
    fn set_minimized(&mut self, minimized: bool) {
        self.set_rect_record((0, 0, 0, 0).into());
        self.set_image_rect_record((0, 0, 0, 0).into());

        self.set_property("minimized", minimized.to_value());
    }

    #[inline]
    fn repaint_when_resize(&self) -> bool {
        self.repaint_when_resize
    }

    #[inline]
    fn set_repaint_when_resize(&mut self, repaint: bool) {
        self.repaint_when_resize = repaint
    }

    #[inline]
    fn is_pressed(&self) -> bool {
        self.id() == self.window().pressed_widget()
    }

    #[inline]
    fn propagate_update(&mut self) {
        self.update();

        self.set_property("propagate_update", true.to_value());
    }

    #[inline]
    fn propagate_update_rect(&mut self, rect: CoordRect) {
        self.update_rect(rect);

        self.set_property("propagate_update_rect", rect.to_value());
    }

    #[inline]
    fn propagate_update_styles_rect(&mut self, rect: CoordRect) {
        self.update_styles_rect(rect);

        self.set_property("propagate_update_styles_rect", rect.to_value());
    }

    #[inline]
    fn descendant_of(&self, id: ObjectId) -> bool {
        let mut parent = self.get_parent_ref();
        while let Some(p) = parent {
            if p.id() == id {
                return true;
            }

            parent = p.get_parent_ref();
        }
        false
    }

    #[inline]
    fn propagate_animation_progressing(&mut self, is: bool) {
        self.set_property("animation_progressing", is.to_value())
    }

    #[inline]
    fn is_animation_progressing(&self) -> bool {
        match self.get_property("animation_progressing") {
            Some(p) => p.get::<bool>(),
            None => false,
        }
    }

    #[inline]
    fn transparency(&self) -> Transparency {
        match self.get_property("transparency") {
            Some(t) => t.get::<Transparency>(),
            None => 255,
        }
    }

    #[inline]
    fn set_transparency(&mut self, transparency: Transparency) {
        self.set_property("transparency", transparency.to_value())
    }

    #[inline]
    fn propagate_set_transparency(&mut self, transparency: Transparency) {
        self.set_transparency(transparency);

        self.set_property("propagate_transparency", transparency.to_value())
    }

    #[inline]
    fn size_hint(&self) -> SizeHint {
        self.size_hint
    }

    #[inline]
    fn set_size_hint(&mut self, size_hint: SizeHint) {
        if let (Some(min), Some(max)) = size_hint.all_width() {
            if min > max {
                panic!("`Minimum size hint can not be larger than maximum size hint.")
            }
        }
        if let (Some(min), Some(max)) = size_hint.all_height() {
            if min > max {
                panic!("`Minimum size hint can not be larger than maximum size hint.")
            }
        }
        self.size_hint = size_hint
    }

    #[inline]
    fn is_event_bubbled(&self, event_bubble: EventBubble) -> bool {
        self.event_bubble.contains(event_bubble)
    }

    #[inline]
    fn enable_bubble(&mut self, event_bubble: EventBubble) {
        self.event_bubble.insert(event_bubble)
    }

    #[inline]
    fn disable_bubble(&mut self, event_bubble: EventBubble) {
        self.event_bubble.remove(event_bubble)
    }

    #[inline]
    fn is_propagate_event_bubble(&self) -> bool {
        self.propagate_event_bubble
    }

    #[inline]
    fn set_propagate_event_bubble(&mut self, is: bool) {
        self.propagate_event_bubble = is
    }

    #[inline]
    fn is_propagate_mouse_tracking(&self) -> bool {
        self.propagate_mouse_tracking
    }

    #[inline]
    fn set_propagate_mouse_tracking(&mut self, is: bool) {
        self.propagate_mouse_tracking = is
    }

    #[inline]
    fn is_strict_clip_widget(&self) -> bool {
        self.strict_clip_widget
    }

    #[inline]
    fn set_strict_clip_widget(&mut self, strict_clip_widget: bool) {
        self.strict_clip_widget = strict_clip_widget
    }

    #[inline]
    fn is_resize_redraw(&self) -> bool {
        self.resize_redraw
    }
}
