use super::{
    callbacks::Callbacks, widget_inner::WidgetInnerExt, EventBubble, Font, ReflectSpacingCapable,
    SizeHint, Transparency, WidgetImpl,
};
use crate::{
    animation::snapshot::ReflectSnapshot,
    application_window::ApplicationWindow,
    font::FontTypeface,
    graphics::{border::Border, element::ElementImpl},
    primitive::Message,
    widget::WidgetSignals,
};
use std::ptr::NonNull;
use tlib::{
    figure::{Color, CoordRect, FPoint, FRect, Point, Rect, Size},
    namespace::{Align, BorderStyle, Coordinate, SystemCursorShape},
    object::ObjectId,
    prelude::*,
    ptr_mut,
};

////////////////////////////////////// WidgetExt //////////////////////////////////////
/// The extended actions of [`Widget`], impl by proc-macro [`extends_widget`] automaticly.
pub trait WidgetExt {
    /// Get the name of widget.
    fn name(&self) -> String;

    /// The widget was initialized or not.
    fn initialized(&self) -> bool;

    /// Upcast widget to `&mut dyn ElementImpl`
    fn as_element(&mut self) -> &mut dyn ElementImpl;

    /// Widget need re-render styles in next frame or not.
    fn render_styles(&self) -> bool;

    /// Set the widget need re-render styles in next frame or not.
    fn set_render_styles(&mut self, rerender: bool);

    /// Is the widget only render the difference after size changed.
    fn is_render_difference(&self) -> bool;

    /// Set the widget is only render the difference after size changed.
    fn set_render_difference(&mut self, rerender_difference: bool);

    /// ## Do not invoke this parent directly.
    fn set_parent(&mut self, parent: *mut dyn WidgetImpl);

    /// Get the raw pointer of child.
    fn get_raw_child(&self) -> Option<*const dyn WidgetImpl>;

    /// Get the raw mut pointer of child.
    fn get_raw_child_mut(&mut self) -> Option<*mut dyn WidgetImpl>;

    /// Get the ref of child.
    fn get_child_ref(&self) -> Option<&dyn WidgetImpl>;

    /// Get the ref mut of child.
    fn get_child_mut(&mut self) -> Option<&mut dyn WidgetImpl>;

    /// Get the raw pointer of parent.
    fn get_raw_parent(&self) -> Option<*const dyn WidgetImpl>;

    /// Get the raw mut pointer of parent.
    fn get_raw_parent_mut(&mut self) -> Option<*mut dyn WidgetImpl>;

    /// Get the ref of parent.
    fn get_parent_ref(&self) -> Option<&dyn WidgetImpl>;

    /// Get the ref mut of child.
    fn get_parent_mut(&mut self) -> Option<&mut dyn WidgetImpl>;

    /// Hide the Widget.
    fn hide(&mut self);

    /// Show the Widget.
    fn show(&mut self);

    /// Return true if widget is visble, otherwise, false is returned.
    fn visible(&self) -> bool;

    /// Setter of property `focus`. <br>
    /// Only effected after phase `run_after`.
    fn set_focus(&mut self, focus: bool);

    /// Getter of property `focus`.
    fn is_focus(&self) -> bool;

    /// Resize the widget. <br>
    /// `resize() will set fixed_width and fixed_height to false`, make widget flexible.
    fn resize(&mut self, width: Option<i32>, height: Option<i32>);

    /// Request the widget's width. <br>
    /// This function should be used in construct phase of the ui component,
    /// the function will not change the layout and will not trigger the signal `size_changed()`.
    fn width_request(&mut self, width: i32);

    /// Request the widget's width. <br>
    /// This function should be used in construct phase of the ui component,
    /// the function will not change the layout and will not trigger the signal `size_changed()`.
    fn height_request(&mut self, width: i32);

    /// Get the width request of widget.
    fn get_width_request(&self) -> i32;

    /// Get the height request of widget.
    fn get_height_request(&self) -> i32;

    /// Update widget's child image rect union.
    fn update_geometry(&mut self);

    /// Widget's width was fixed or not,
    /// `true` when user invoke [`width_request`](WidgetExt::width_request)
    fn fixed_width(&self) -> bool;

    /// Widget's height was fixed or not,
    /// `true` when user invoke [`height_request`](WidgetExt::height_request)
    fn fixed_height(&self) -> bool;

    /// Used in conjunction with the function [`hexpand`],
    /// if widget was width fixed and hexpanded, `the width ration = width / parent_width`
    fn fixed_width_ration(&self) -> f32;

    /// Used in conjunction with the function [`vexpand`],
    /// if widget was height fixed and vexpanded, `the height ration = height / parent_height`
    fn fixed_height_ration(&self) -> f32;

    /// Set alignment on the horizontal direction.
    fn set_halign(&mut self, halign: Align);

    /// Set alignment on the vertical direction.
    fn set_valign(&mut self, valign: Align);

    /// Get alignment on the horizontal direction.
    fn halign(&self) -> Align;

    /// Get alignment on the vertical direction.
    fn valign(&self) -> Align;

    /// Set the font of widget.
    fn set_font(&mut self, font: Font);

    /// Get the font of widget.
    fn font(&self) -> &Font;

    /// Get the mutable reference of font of widget.
    fn font_mut(&mut self) -> &mut Font;

    /// Set the font family of Widget.
    fn set_font_families(&mut self, families: &[&str]);

    /// Get the rect of widget without borders.
    fn borderless_rect(&self) -> FRect;

    /// Get the size of widget. The size does not include the margins.
    fn size(&self) -> Size;

    /// Get the area of widget's total image Rect with the margins. <br>
    /// The [`Coordinate`] was `World`.
    fn image_rect(&self) -> Rect;

    /// Get the area of widget's total image Rect with the margins. <br>
    /// The [`Coordinate`] was `World`.
    fn image_rect_f(&self) -> FRect;

    /// Get the area of widget's origin Rect. <br>
    /// The default [`Coordinate`] was `World`.
    fn origin_rect(&self, coord: Option<Coordinate>) -> Rect;

    /// Get the area of widget's origin Rect. <br>
    /// The default [`Coordinate`] was `World`.
    fn origin_rect_f(&self, coord: Option<Coordinate>) -> FRect;

    /// Get the area inside the widget's paddings. <br>
    /// The default [`Coordinate`] was `World`.
    fn contents_rect(&self, coord: Option<Coordinate>) -> Rect;

    /// Get the area inside the widget's paddings. <br>
    /// The default [`Coordinate`] was `World`.
    fn contents_rect_f(&self, coord: Option<Coordinate>) -> FRect;

    /// Get the widget's background color.
    fn background(&self) -> Color;

    /// Set the widget's background color.
    fn set_background(&mut self, color: Color);

    /// Get the margins of the Widget. (top, right, bottom, left)
    fn margins(&self) -> (i32, i32, i32, i32);

    /// Get the top margin of the Widget.
    fn margin_top(&self) -> i32;

    /// Get the right margin of the Widget.
    fn margin_right(&self) -> i32;

    /// Get the bottom margin of the Widget.
    fn margin_bottom(&self) -> i32;

    /// Get the left margin of the Widget.
    fn margin_left(&self) -> i32;

    /// Set the margins of the Widget.
    fn set_margins(&mut self, top: i32, right: i32, bottom: i32, left: i32);

    /// Set the top margin of the Widget.
    fn set_margin_top(&mut self, val: i32);

    /// Set the right margin of the Widget.
    fn set_margin_right(&mut self, val: i32);

    /// Set the bottom margin of the Widget.
    fn set_margin_bottom(&mut self, val: i32);

    /// Set the left margin of the Widget.
    fn set_margin_left(&mut self, val: i32);

    /// Get the paddins of the Widget. (top, right, bottom, left)
    fn paddings(&self) -> (i32, i32, i32, i32);

    /// Get the top padding of the Widget.
    fn padding_top(&self) -> i32;

    /// Get the right padding of the Widget.
    fn padding_right(&self) -> i32;

    /// Get the bottom padding of the Widget.
    fn padding_bottom(&self) -> i32;

    /// Get the left padding of the Widget.
    fn padding_left(&self) -> i32;

    /// Set the paddings of the Widget.
    fn set_paddings(&mut self, top: i32, right: i32, bottom: i32, left: i32);

    /// Set the top padding of the Widget.
    fn set_padding_top(&mut self, val: i32);

    /// Set the right padding of the Widget.
    fn set_padding_right(&mut self, val: i32);

    /// Set the bottom padding of the Widget.
    fn set_padding_bottom(&mut self, val: i32);

    /// Set the left padding of the Widget.
    fn set_padding_left(&mut self, val: i32);

    /// Get the refrence of [`Border`].
    fn border_ref(&self) -> &Border;

    /// Set the borders of the widget.
    fn set_borders(&mut self, top: f32, right: f32, bottom: f32, left: f32);

    /// Set the border radius of the widget.
    fn set_border_radius(&mut self, radius: f32);

    /// Set the border style of the widget.
    fn set_border_style(&mut self, style: BorderStyle);

    /// Set the border color(all directions) of the widget.
    fn set_border_color(&mut self, color: Color);

    /// Set the top border color of the widget.
    fn set_border_top_color(&mut self, color: Color);

    /// Set the right border color of the widget.
    fn set_border_right_color(&mut self, color: Color);

    /// Set the bottom border color of the widget.
    fn set_border_bottom_color(&mut self, color: Color);

    /// Set the left border color of the widget.
    fn set_border_left_color(&mut self, color: Color);

    /// Get the borders of the widget. <br>
    /// @return (top, right, bottom, left)
    fn borders(&self) -> (f32, f32, f32, f32);

    /// Get the border style of the widget.
    fn border_style(&self) -> BorderStyle;

    /// Get the border color of the widget.
    fn border_color(&self) -> (Color, Color, Color, Color);

    /// Set the system cursor shape.
    fn set_cursor_shape(&mut self, cursor: SystemCursorShape);

    /// Map the given point to global coordinate.
    fn map_to_global(&self, point: &Point) -> Point;

    /// Map the given point to widget coordinate.
    fn map_to_widget(&self, point: &Point) -> Point;

    /// Map the given point to global coordinate.
    fn map_to_global_f(&self, point: &FPoint) -> FPoint;

    /// Map the given point to widget coordinate.
    fn map_to_widget_f(&self, point: &FPoint) -> FPoint;

    /// The widget tracking the `MouseMoveEvent` or not.
    fn mouse_tracking(&self) -> bool;

    /// Set the `mouse_tracking` status of widget.
    ///
    /// when `ture`, widget will track the movement of mouse.
    fn set_mouse_tracking(&mut self, is_tracking: bool);

    /// Get `hexpand` of widget.
    ///
    /// `hexpand`: Horizontal scalability, if `true` can cause child widget to expand horizontally
    /// with changes in the width of the parent widget.
    fn hexpand(&self) -> bool;

    /// Set `hexpand` of widget.
    ///
    /// `hexpand`: Horizontal scalability, if `true` can cause child widget to expand horizontally
    /// with changes in the width of the parent widget.
    fn set_hexpand(&mut self, hexpand: bool);

    /// Get `vexpand` of widget.
    ///
    /// `vexpand`: Vertical scalability, if `true` can cause child widget to expand vertically
    /// height changes in the height of the parent widget.
    fn vexpand(&self) -> bool;

    /// Set `vexpand` of widget.
    ///
    /// `vexpand`: Vertical scalability, if `true` can cause child widget to expand vertically
    /// height changes in the height of the parent widget.
    fn set_vexpand(&mut self, vexpand: bool);

    /// The scale factor on horizontal, ratio of child width to parent component,
    /// only effective when widget's `hexpand was true` and `fixed_width was false`.
    ///
    /// ### when parent was widget:
    /// `width ration = hsclae / 1`
    ///
    /// ### when parent was coontainer:
    /// `width ration = hscale / parent_children_total_hscales`
    fn hscale(&self) -> f32;

    /// See [`hscale`](WidgetExt::hscale)
    fn set_hscale(&mut self, hscale: f32);

    /// The scale factor on vertical, ratio of child height to parent component,
    /// only effective when widget's hexpand was true.
    ///
    /// ### when parent was widget:
    /// `height ration = vsclae / 1`
    ///
    /// ### when parent was coontainer:
    /// `height ration = vscale / parent_children_total_vscales`
    fn vscale(&self) -> f32;

    /// See [`vscale`](WidgetExt::vscale)
    fn set_vscale(&mut self, vscale: f32);

    /// Is the widget's size was minimized or not.
    fn minimized(&self) -> bool;

    /// Set the widget's minimized status.
    fn set_minimized(&mut self, minimized: bool);

    /// Is the widget repaint when resize.
    fn repaint_when_resize(&self) -> bool;

    /// Set the widget is repaint when resize.
    fn set_repaint_when_resize(&mut self, repaint: bool);

    /// Is the widget under mouse pressed.
    fn is_pressed(&self) -> bool;

    /// Invalidate this widget to update it, and also update the child widget..
    fn propagate_update(&mut self);

    /// Invalidate this widget with dirty rect to update it, and also update the child widget..<br>
    /// This will result in clipping the drawing area of the widget.(after styles render)
    fn propagate_update_rect(&mut self, rect: CoordRect);

    /// Invalidate this widget with dirty styles rect to update it, and also update the child widget..<br>
    /// This will result in clipping the drawing area of the widget.(before styles render)
    fn propagate_update_styles_rect(&mut self, rect: CoordRect);

    /// Check if the widget is the ancestor of the widget represented by the specified id.
    fn ancestor_of(&self, id: ObjectId) -> bool;

    /// Check if the widget is a descendant of the widget represented by the specified id.
    fn descendant_of(&self, id: ObjectId) -> bool;

    /// Propagate setting the property `animation_progressing`
    fn propagate_animation_progressing(&mut self, is: bool);

    /// Is the widget under animation pregressing.
    fn is_animation_progressing(&self) -> bool;

    /// Getting the transparency of widget.
    fn transparency(&self) -> Transparency;

    /// Setting the transparency of widget.
    fn set_transparency(&mut self, transparency: Transparency);

    /// Propagate setting the transparency of widget.
    fn propagate_set_transparency(&mut self, transparency: Transparency);

    /// Get the size hint of widget.
    ///
    /// For specific information about size_hint, please refer to [`size_hint`](crate::widget::Widget::size_hint)
    fn size_hint(&self) -> SizeHint;

    /// Set the size hint of widget.
    ///
    /// For specific information about size_hint, please refer to [`size_hint`](crate::widget::Widget::size_hint)
    fn set_size_hint(&mut self, size_hint: SizeHint);

    /// Whether the event will be bubbled or not.
    fn is_event_bubbled(&self, event_bubble: EventBubble) -> bool;

    /// Enable the event bubble.
    fn enable_bubble(&mut self, event_bubble: EventBubble);

    /// Disable the event bubble.
    fn disable_bubble(&mut self, event_bubble: EventBubble);

    /// Get the value of [`propagate_event_bubble`](Widget::propagate_event_bubble).
    fn is_propagate_event_bubble(&self) -> bool;

    /// Set the value of [`propagate_event_bubble`](Widget::propagate_event_bubble).
    fn set_propagate_event_bubble(&mut self, is: bool);

    /// Get the value of [`propagate_mouse_tracking`](Widget::propagate_mouse_tracking).
    fn is_propagate_mouse_tracking(&self) -> bool;

    /// Set the value of [`propagate_event_bubble`](Widget::propagate_event_bubble).
    fn set_propagate_mouse_tracking(&mut self, is: bool);

    /// Get the value of [`strict_clip_widget`](Widget::strict_clip_widget).
    fn is_strict_clip_widget(&self) -> bool;

    /// Set the value of [`strict_clip_widget`](Widget::strict_clip_widget).
    fn set_strict_clip_widget(&mut self, strict_clip_widget: bool);

    /// Get the value of [`strict_clip_widget`](Widget::resize_redraw).
    fn is_resize_redraw(&self) -> bool;

    /// Get the reference of [`Callbacks`]
    fn callbacks(&self) -> &Callbacks;

    /// Get the mutable reference of [`Callbacks`]
    fn callbacks_mut(&mut self) -> &mut Callbacks;

    /// Whether the fixed widget occupy the parent widget's space.
    ///
    /// @see [`Widget::occupy_space`]
    fn is_occupy_space(&self) -> bool;

    /// Set the fixed widget occupy the parent widget's space or not.
    ///
    /// @see [`Widget::occupy_space`]
    fn set_occupy_space(&mut self, occupy_space: bool);
}

impl<T: WidgetImpl> WidgetExt for T {
    #[inline]
    fn name(&self) -> String {
        self.get_property("name").unwrap().get::<String>()
    }

    #[inline]
    fn initialized(&self) -> bool {
        self.widget_props().initialized
    }

    #[inline]
    fn as_element(&mut self) -> &mut dyn ElementImpl {
        self
    }

    #[inline]
    fn render_styles(&self) -> bool {
        match self.get_property("rerender_styles") {
            Some(val) => val.get::<bool>(),
            None => false,
        }
    }

    #[inline]
    fn set_render_styles(&mut self, rerender: bool) {
        self.set_property("rerender_styles", rerender.to_value())
    }

    #[inline]
    fn is_render_difference(&self) -> bool {
        self.widget_props().rerender_difference
    }

    #[inline]
    fn set_render_difference(&mut self, rerender_difference: bool) {
        self.widget_props_mut().rerender_difference = rerender_difference
    }

    #[inline]
    fn set_parent(&mut self, parent: *mut dyn WidgetImpl) {
        self.widget_props_mut().parent = NonNull::new(parent)
    }

    #[inline]
    fn get_raw_child(&self) -> Option<*const dyn WidgetImpl> {
        let mut child = self
            .widget_props()
            .child
            .as_ref()
            .map(|c| c.as_ref().as_ptr());

        if child.is_none() {
            unsafe {
                child = match self.widget_props().child_ref {
                    Some(ref c) => Some(c.as_ref().as_ptr()),
                    None => None,
                }
            }
        }

        child
    }

    #[inline]
    fn get_raw_child_mut(&mut self) -> Option<*mut dyn WidgetImpl> {
        let mut child = self
            .widget_props_mut()
            .child
            .as_mut()
            .map(|c| c.as_mut().as_ptr_mut());

        if child.is_none() {
            unsafe {
                child = match self.widget_props_mut().child_ref {
                    Some(ref mut c) => Some(c.as_mut().as_ptr_mut()),
                    None => None,
                }
            }
        }

        child
    }

    #[inline]
    fn get_child_ref(&self) -> Option<&dyn WidgetImpl> {
        let mut child = self.widget_props().child.as_ref().map(|c| c.as_ref());

        if child.is_none() {
            unsafe {
                child = match self.widget_props().child_ref {
                    Some(ref c) => Some(c.as_ref()),
                    None => None,
                }
            }
        }

        child
    }

    #[inline]
    fn get_child_mut(&mut self) -> Option<&mut dyn WidgetImpl> {
        let props = self.widget_props_mut();
        let mut child = props.child.as_mut().map(|c| c.as_mut());

        if child.is_none() {
            unsafe {
                child = match props.child_ref {
                    Some(ref mut c) => Some(c.as_mut()),
                    None => None,
                }
            }
        }

        child
    }

    #[inline]
    fn get_raw_parent(&self) -> Option<*const dyn WidgetImpl> {
        match self.widget_props().parent.as_ref() {
            Some(parent) => Some(unsafe { parent.as_ref() }),
            None => None,
        }
    }

    #[inline]
    fn get_raw_parent_mut(&mut self) -> Option<*mut dyn WidgetImpl> {
        match self.widget_props_mut().parent.as_mut() {
            Some(parent) => Some(unsafe { parent.as_mut() }),
            None => None,
        }
    }

    #[inline]
    fn get_parent_ref(&self) -> Option<&dyn WidgetImpl> {
        match self.widget_props().parent {
            Some(ref parent) => unsafe { Some(parent.as_ref()) },
            None => None,
        }
    }

    #[inline]
    fn get_parent_mut(&mut self) -> Option<&mut dyn WidgetImpl> {
        match self.widget_props_mut().parent {
            Some(ref mut parent) => unsafe { Some(parent.as_mut()) },
            None => None,
        }
    }

    #[inline]
    fn hide(&mut self) {
        if let Some(snapshot) = cast_mut!(self as Snapshot) {
            snapshot.start(false);
        }

        self.set_property("visible", false.to_value());

        if !self.is_animation_progressing() && self.window().initialized() {
            self.window()
                .invalid_effected_widgets(self.image_rect(), self.id());
        }
    }

    #[inline]
    fn show(&mut self) {
        if let Some(snapshot) = cast_mut!(self as Snapshot) {
            snapshot.start(true);
        }

        self.set_property("visible", true.to_value());
        self.set_render_styles(true);
        self.update();
    }

    #[inline]
    fn visible(&self) -> bool {
        self.get_property("visible").unwrap().get::<bool>()
    }

    #[inline]
    fn set_focus(&mut self, focus: bool) {
        if !self.enable_focus() {
            return;
        }

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
        let mut resized = false;

        if let Some(width) = width {
            self.set_fixed_width(width);
            resized = true;
        }
        if let Some(height) = height {
            self.set_fixed_height(height);
            resized = true;
        }

        if resized {
            emit!(self.size_changed(), self.size())
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
        self.set_fixed_width(width);
        self.widget_props_mut().fixed_width = true;
        self.widget_props_mut().width_request = width;
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

            self.widget_props_mut().fixed_width_ration = width as f32 / parent_size.width() as f32;
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
        self.set_fixed_height(height);
        self.widget_props_mut().fixed_height = true;
        self.widget_props_mut().height_request = height;
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

            self.widget_props_mut().fixed_height_ration =
                height as f32 / parent_size.height() as f32;
        }
    }

    #[inline]
    fn get_width_request(&self) -> i32 {
        self.widget_props().width_request
    }

    #[inline]
    fn get_height_request(&self) -> i32 {
        self.widget_props().height_request
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
        self.widget_props().fixed_width
    }

    #[inline]
    fn fixed_height(&self) -> bool {
        self.widget_props().fixed_height
    }

    #[inline]
    fn fixed_width_ration(&self) -> f32 {
        self.widget_props().fixed_width_ration
    }

    #[inline]
    fn fixed_height_ration(&self) -> f32 {
        self.widget_props().fixed_height_ration
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
        self.widget_props_mut().font = font;
        self.font_changed();
    }

    #[inline]
    fn font(&self) -> &Font {
        &self.widget_props().font
    }

    #[inline]
    fn font_mut(&mut self) -> &mut Font {
        &mut self.widget_props_mut().font
    }

    #[inline]
    fn set_font_families(&mut self, families: &[&str]) {
        let mut typefaces = vec![];
        for f in families {
            let typeface = FontTypeface::new(f);
            typefaces.push(typeface);
        }
        self.widget_props_mut().font.set_typefaces(typefaces);
        self.update();
        self.font_changed();
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
        self.widget_props().background
    }

    #[inline]
    fn set_background(&mut self, color: Color) {
        self.set_render_styles(true);
        self.widget_props_mut().background = color;
        emit!(Widget::set_background => self.background_changed(), color);

        self.update();
    }

    #[inline]
    fn margins(&self) -> (i32, i32, i32, i32) {
        let props = self.widget_props();
        (
            props.margins[0],
            props.margins[1],
            props.margins[2],
            props.margins[3],
        )
    }

    #[inline]
    fn margin_top(&self) -> i32 {
        self.widget_props().margins[0]
    }

    #[inline]
    fn margin_right(&self) -> i32 {
        self.widget_props().margins[1]
    }

    #[inline]
    fn margin_bottom(&self) -> i32 {
        self.widget_props().margins[2]
    }

    #[inline]
    fn margin_left(&self) -> i32 {
        self.widget_props().margins[3]
    }

    #[inline]
    fn set_margins(&mut self, top: i32, right: i32, bottom: i32, left: i32) {
        let props = self.widget_props_mut();
        props.margins[0] = top;
        props.margins[1] = right;
        props.margins[2] = bottom;
        props.margins[3] = left;

        props.need_update_geometry = top != 0 || right != 0 || bottom != 0 || left != 0;
    }

    #[inline]
    fn set_margin_top(&mut self, val: i32) {
        self.widget_props_mut().margins[0] = val;

        self.widget_props_mut().need_update_geometry = val != 0;
    }

    #[inline]
    fn set_margin_right(&mut self, val: i32) {
        self.widget_props_mut().margins[1] = val;

        self.widget_props_mut().need_update_geometry = val != 0;
    }

    #[inline]
    fn set_margin_bottom(&mut self, val: i32) {
        self.widget_props_mut().margins[2] = val;

        self.widget_props_mut().need_update_geometry = val != 0;
    }

    #[inline]
    fn set_margin_left(&mut self, val: i32) {
        self.widget_props_mut().margins[3] = val;

        self.widget_props_mut().need_update_geometry = val != 0;
    }

    #[inline]
    fn paddings(&self) -> (i32, i32, i32, i32) {
        let props = self.widget_props();
        (
            props.paddings[0],
            props.paddings[1],
            props.paddings[2],
            props.paddings[3],
        )
    }

    #[inline]
    fn padding_top(&self) -> i32 {
        self.widget_props().paddings[0]
    }

    #[inline]
    fn padding_right(&self) -> i32 {
        self.widget_props().paddings[1]
    }

    #[inline]
    fn padding_bottom(&self) -> i32 {
        self.widget_props().paddings[2]
    }

    #[inline]
    fn padding_left(&self) -> i32 {
        self.widget_props().paddings[3]
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

        let props = self.widget_props_mut();
        props.paddings[0] = top;
        props.paddings[1] = right;
        props.paddings[2] = bottom;
        props.paddings[3] = left;
    }

    #[inline]
    fn set_padding_top(&mut self, mut val: i32) {
        if val < 0 {
            val = 0;
        }
        self.widget_props_mut().paddings[0] = val;
    }

    #[inline]
    fn set_padding_right(&mut self, mut val: i32) {
        if val < 0 {
            val = 0;
        }
        self.widget_props_mut().paddings[1] = val;
    }

    #[inline]
    fn set_padding_bottom(&mut self, mut val: i32) {
        if val < 0 {
            val = 0;
        }
        self.widget_props_mut().paddings[2] = val;
    }

    #[inline]
    fn set_padding_left(&mut self, mut val: i32) {
        if val < 0 {
            val = 0;
        }
        self.widget_props_mut().paddings[3] = val;
    }

    #[inline]
    fn border_ref(&self) -> &Border {
        &self.widget_props().border
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
        let props = self.widget_props_mut();
        props.border.width.0 = top;
        props.border.width.1 = right;
        props.border.width.2 = bottom;
        props.border.width.3 = left;
    }

    #[inline]
    fn set_border_radius(&mut self, radius: f32) {
        if radius < 0. {
            return;
        }

        self.widget_props_mut().border.border_radius = radius;
    }

    #[inline]
    fn set_border_style(&mut self, style: BorderStyle) {
        self.widget_props_mut().border.style = style;
    }

    #[inline]
    fn set_border_color(&mut self, color: Color) {
        self.widget_props_mut().border.border_color = (color, color, color, color);
    }

    #[inline]
    fn set_border_top_color(&mut self, color: Color) {
        self.widget_props_mut().border.border_color.0 = color;
    }

    #[inline]
    fn set_border_right_color(&mut self, color: Color) {
        self.widget_props_mut().border.border_color.1 = color;
    }

    #[inline]
    fn set_border_bottom_color(&mut self, color: Color) {
        self.widget_props_mut().border.border_color.2 = color;
    }

    #[inline]
    fn set_border_left_color(&mut self, color: Color) {
        self.widget_props_mut().border.border_color.3 = color;
    }

    #[inline]
    fn borders(&self) -> (f32, f32, f32, f32) {
        self.widget_props().border.width
    }

    #[inline]
    fn border_style(&self) -> BorderStyle {
        self.widget_props().border.style
    }

    #[inline]
    fn border_color(&self) -> (Color, Color, Color, Color) {
        self.widget_props().border.border_color
    }

    #[inline]
    fn set_cursor_shape(&mut self, cursor: SystemCursorShape) {
        let window = self.window();
        window.send_message(Message::SetCursorShape(cursor, window.winit_id().unwrap()))
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
    fn hexpand(&self) -> bool {
        self.widget_props().hexpand
    }

    #[inline]
    fn set_hexpand(&mut self, hexpand: bool) {
        self.widget_props_mut().hexpand = hexpand
    }

    #[inline]
    fn vexpand(&self) -> bool {
        self.widget_props().vexpand
    }

    #[inline]
    fn set_vexpand(&mut self, vexpand: bool) {
        self.widget_props_mut().vexpand = vexpand
    }

    #[inline]
    fn hscale(&self) -> f32 {
        if self.widget_props().fixed_width {
            return 0.;
        }
        self.widget_props().hscale
    }

    #[inline]
    fn set_hscale(&mut self, hscale: f32) {
        self.widget_props_mut().hscale = hscale
    }

    #[inline]
    fn vscale(&self) -> f32 {
        if self.widget_props().fixed_height {
            return 0.;
        }
        self.widget_props().vscale
    }

    #[inline]
    fn set_vscale(&mut self, vscale: f32) {
        self.widget_props_mut().vscale = vscale
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
        self.widget_props().repaint_when_resize
    }

    #[inline]
    fn set_repaint_when_resize(&mut self, repaint: bool) {
        self.widget_props_mut().repaint_when_resize = repaint
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
    fn ancestor_of(&self, id: ObjectId) -> bool {
        self.children_index().contains(&id)
    }

    #[inline]
    fn descendant_of(&self, id: ObjectId) -> bool {
        if let Some(p) = self.window().finds_by_id(id) {
            p.children_index().contains(&self.id())
        } else {
            false
        }
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
        self.widget_props().size_hint
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
        self.widget_props_mut().size_hint = size_hint
    }

    #[inline]
    fn is_event_bubbled(&self, event_bubble: EventBubble) -> bool {
        self.widget_props().event_bubble.contains(event_bubble)
    }

    #[inline]
    fn enable_bubble(&mut self, event_bubble: EventBubble) {
        self.widget_props_mut().event_bubble.insert(event_bubble)
    }

    #[inline]
    fn disable_bubble(&mut self, event_bubble: EventBubble) {
        self.widget_props_mut().event_bubble.remove(event_bubble)
    }

    #[inline]
    fn is_propagate_event_bubble(&self) -> bool {
        self.widget_props().propagate_event_bubble
    }

    #[inline]
    fn set_propagate_event_bubble(&mut self, is: bool) {
        self.widget_props_mut().propagate_event_bubble = is
    }

    #[inline]
    fn is_propagate_mouse_tracking(&self) -> bool {
        self.widget_props().propagate_mouse_tracking
    }

    #[inline]
    fn set_propagate_mouse_tracking(&mut self, is: bool) {
        self.widget_props_mut().propagate_mouse_tracking = is
    }

    #[inline]
    fn is_strict_clip_widget(&self) -> bool {
        self.widget_props().strict_clip_widget
    }

    #[inline]
    fn set_strict_clip_widget(&mut self, strict_clip_widget: bool) {
        self.widget_props_mut().strict_clip_widget = strict_clip_widget
    }

    #[inline]
    fn is_resize_redraw(&self) -> bool {
        self.widget_props().resize_redraw
    }

    #[inline]
    fn callbacks(&self) -> &Callbacks {
        &self.widget_props().callbacks
    }

    #[inline]
    fn callbacks_mut(&mut self) -> &mut Callbacks {
        &mut self.widget_props_mut().callbacks
    }

    #[inline]
    fn is_occupy_space(&self) -> bool {
        self.widget_props().occupy_space
    }

    #[inline]
    fn set_occupy_space(&mut self, occupy_space: bool) {
        self.widget_props_mut().occupy_space = occupy_space;
    }
}
