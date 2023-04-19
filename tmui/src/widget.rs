use crate::{
    application::Application,
    graphics::{
        drawing_context::DrawingContext,
        element::ElementImpl,
        figure::{Color, Size},
        painter::Painter,
    },
    platform::Message,
    prelude::*, util::skia_font_clone,
};
use log::debug;
use skia_safe::Font;
use tlib::{
    namespace::{Align, BorderStyle, Coordinate, SystemCursorShape},
    object::{IsSubclassable, ObjectImpl, ObjectSubclass},
    signals,
};

#[extends_element]
pub struct Widget {
    parent: Option<*const dyn WidgetImpl>,
    child: Option<Box<dyn WidgetImpl>>,

    background: Color,
    font: Font,
    font_family: String,
    margins: [i32; 4],
    paddings: [i32; 4],
    borders: [f32; 4],
    border_style: BorderStyle,
    border_color: Color,
}

////////////////////////////////////// Widget Signals //////////////////////////////////////
pub trait WidgetSignals: ActionExt {
    signals! {
        /// Emit when widget's size changed.
        size_changed();
    }
}
impl<T: WidgetImpl + ActionExt> WidgetSignals for T {}

////////////////////////////////////// Widget Implements //////////////////////////////////////
impl Default for Widget {
    fn default() -> Self {
        Self {
            parent: Default::default(),
            child: Default::default(),
            background: Color::WHITE,
            font: Default::default(),
            font_family: Default::default(),
            margins: Default::default(),
            paddings: Default::default(),
            element: Default::default(),
            borders: Default::default(),
            border_style: Default::default(),
            border_color: Color::BLACK,
        }
    }
}

impl Widget {
    pub fn child_internal<T>(&mut self, child: T)
    where
        T: WidgetImpl + ElementImpl + IsA<Widget>,
    {
        let child = Box::new(child);
        self.child = Some(child);
    }

    /// Notify all the child widget to invalidate.
    fn notify_invalidate(&mut self) {
        if let Some(child) = self.get_raw_child_mut() {
            unsafe { child.as_mut().unwrap().update() }
        }
    }

    /// Notify the child to change the visibility.
    fn notify_visible(&mut self, visible: bool) {
        if let Some(child) = self.get_raw_child_mut() {
            let child = unsafe { child.as_mut().unwrap() };
            if visible {
                child.show()
            } else {
                child.hide()
            }
        }
    }
}

impl ObjectSubclass for Widget {
    const NAME: &'static str = "Widget";

    type Type = Widget;
    type ParentType = Object;
}

impl IsSubclassable for Widget {}

impl ObjectImpl for Widget {
    fn construct(&mut self) {
        self.parent_construct();

        self.set_halign(Align::default());
        self.set_valign(Align::default());

        self.show();
        self.set_focus(false);

        debug!("`Widget` construct")
    }

    fn on_property_set(&mut self, name: &str, value: &Value) {
        debug!("`Widget` on set property, name = {}", name);

        match name {
            "width" => {
                let width = value.get::<i32>();
                self.set_fixed_width(width);
            }
            "height" => {
                let height = value.get::<i32>();
                self.set_fixed_height(height);
            }
            "invalidate" => {
                let invalidate = value.get::<bool>();
                if invalidate {
                    // Notify all the child widget to invalidate, preparing rerenderer after.
                    self.notify_invalidate();
                }
            }
            "visible" => {
                let visible = value.get::<bool>();
                self.notify_visible(visible)
            }
            "focus" => {}
            _ => {}
        }
    }
}

impl WidgetImpl for Widget {}

impl<T: WidgetImpl + WidgetExt> ElementImpl for T {
    fn on_renderer(&mut self, cr: &DrawingContext) {
        if !self.visible() {
            return
        }

        let mut painter = Painter::new(cr.canvas(), self);

        let origin_rect = self.origin_rect(Some(Coordinate::Widget));

        // Draw the background color of the Widget.
        painter.fill_rect(origin_rect, self.background());

        // Draw the border of the Widget.
        let borders = self.borders();
        let _style = self.border_style();
        painter.set_color(self.border_color());
        if borders[0] > 0. {
            painter.set_line_width(borders[0]);
        }
        if borders[1] > 0. {
            painter.set_line_width(borders[1]);
        }
        if borders[2] > 0. {
            painter.set_line_width(borders[2]);
        }
        if borders[3] > 0. {
            painter.set_line_width(borders[3]);
        }

        self.paint(painter)
    }
}

pub trait WidgetAcquire: WidgetImpl {}

////////////////////////////////////// WidgetExt //////////////////////////////////////
/// The extended actions of [`Widget`], impl by proc-macro [`extends_widget`] automaticly.
pub trait WidgetExt {
    /// Go to[`Function defination`](WidgetExt::as_element) (Defined in [`WidgetExt`])
    fn as_element(&mut self) -> *mut dyn ElementImpl;

    /// ## Do not invoke this function directly.
    /// 
    /// Go to[`Function defination`](WidgetExt::set_parent) (Defined in [`WidgetExt`])
    fn set_parent(&mut self, parent: *mut dyn WidgetImpl);

    /// Get the raw pointer of child.
    /// Use [`WidgetGenericExt::get_child()`](WidgetGenericExt::get_child) instead.
    /// 
    /// Go to[`Function defination`](WidgetExt::get_raw_child) (Defined in [`WidgetExt`])
    fn get_raw_child(&self) -> Option<*const dyn WidgetImpl>;

    /// Get the raw pointer of child.
    /// Use [`WidgetGenericExt::get_child()`](WidgetGenericExt::get_child) instead.
    /// 
    /// Go to[`Function defination`](WidgetExt::get_raw_child_mut) (Defined in [`WidgetExt`])
    fn get_raw_child_mut(&mut self) -> Option<*mut dyn WidgetImpl>;

    /// Get the raw pointer of parent.
    /// Use [`get_parent()`](WidgetGenericExt::get_parent) instead.
    /// 
    /// Go to[`Function defination`](WidgetExt::get_raw_parent) (Defined in [`WidgetExt`])
    fn get_raw_parent(&self) -> Option<*const dyn WidgetImpl>;

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
    fn visible(&mut self) -> bool;

    /// Setter of property `focus`.
    /// 
    /// Go to[`Function defination`](WidgetExt::set_focus) (Defined in [`WidgetExt`])
    fn set_focus(&mut self, focus: bool);

    /// Getter of property `focus`.
    /// 
    /// Go to[`Function defination`](WidgetExt::is_focus) (Defined in [`WidgetExt`])
    fn is_focus(&self) -> bool;

    /// Resize the widget.
    /// 
    /// Go to[`Function defination`](WidgetExt::resize) (Defined in [`WidgetExt`])
    fn resize(&mut self, width: i32, height: i32);

    /// Request the widget's maximum width.
    /// 
    /// Go to[`Function defination`](WidgetExt::width_request) (Defined in [`WidgetExt`])
    fn width_request(&mut self, width: i32);

    /// Request the widget's maximum width.
    /// 
    /// Go to[`Function defination`](WidgetExt::height_request) (Defined in [`WidgetExt`])
    fn height_request(&mut self, width: i32);

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
    fn font(&self) -> Font;

    /// Set the font family of Widget.
    /// 
    /// Go to[`Function defination`](WidgetExt::set_font_family) (Defined in [`WidgetExt`])
    fn set_font_family(&mut self, family: String);

    /// Get the font family of Widget.
    /// 
    /// Go to[`Function defination`](WidgetExt::font_family) (Defined in [`WidgetExt`])
    fn font_family(&self) -> &str;

    /// Get the size of widget.
    /// 
    /// Go to[`Function defination`](WidgetExt::size) (Defined in [`WidgetExt`])
    fn size(&self) -> Size;

    /// Get the area of widget's total image Rect with the margins. <br>
    /// The [`Coordinate`] was `World`.
    /// 
    /// Go to[`Function defination`](WidgetExt::image_rect) (Defined in [`WidgetExt`])
    fn image_rect(&self) -> Rect;

    /// Get the area of widget's origin Rect. <br>
    /// The default [`Coordinate`] was `World`.
    /// 
    /// Go to[`Function defination`](WidgetExt::origin_rect) (Defined in [`WidgetExt`])
    fn origin_rect(&self, coord: Option<Coordinate>) -> Rect;

    /// Get the area inside the widget's paddings. <br>
    /// The default [`Coordinate`] was `World`.
    /// 
    /// Go to[`Function defination`](WidgetExt::contents_rect) (Defined in [`WidgetExt`])
    fn contents_rect(&self, coord: Option<Coordinate>) -> Rect;

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

    /// Set the borders of the widget.
    /// 
    /// Go to[`Function defination`](WidgetExt::set_borders) (Defined in [`WidgetExt`])
    fn set_borders(&mut self, top: f32, right: f32, bottom: f32, left: f32);

    /// Set the border style of the widget.
    /// 
    /// Go to[`Function defination`](WidgetExt::set_border_style) (Defined in [`WidgetExt`])
    fn set_border_style(&mut self, style: BorderStyle);

    /// Set the border color of the widget.
    /// 
    /// Go to[`Function defination`](WidgetExt::set_border_color) (Defined in [`WidgetExt`])
    fn set_border_color(&mut self, color: Color);

    /// Get the borders of the widget.
    /// 
    /// Go to[`Function defination`](WidgetExt::borders) (Defined in [`WidgetExt`])
    fn borders(&self) -> [f32; 4];

    /// Get the border style of the widget.
    /// 
    /// Go to[`Function defination`](WidgetExt::border_style) (Defined in [`WidgetExt`])
    fn border_style(&self) -> BorderStyle;

    /// Get the border color of the widget.
    /// 
    /// Go to[`Function defination`](WidgetExt::border_color) (Defined in [`WidgetExt`])
    fn border_color(&self) -> Color;

    /// Set the system cursor shape.
    /// 
    /// Go to[`Function defination`](WidgetExt::set_cursor_shape) (Defined in [`WidgetExt`])
    fn set_cursor_shape(&mut self, cursor: SystemCursorShape);
}

impl WidgetExt for Widget {
    fn as_element(&mut self) -> *mut dyn ElementImpl {
        self as *mut Self as *mut dyn ElementImpl
    }

    fn set_parent(&mut self, parent: *mut dyn WidgetImpl) {
        self.parent = Some(parent)
    }

    fn get_raw_child(&self) -> Option<*const dyn WidgetImpl> {
        match self.child.as_ref() {
            Some(child) => Some(child.as_ref() as *const dyn WidgetImpl),
            None => None,
        }
    }

    fn get_raw_child_mut(&mut self) -> Option<*mut dyn WidgetImpl> {
        match self.child.as_mut() {
            Some(child) => Some(child.as_mut() as *mut dyn WidgetImpl),
            None => None,
        }
    }

    fn get_raw_parent(&self) -> Option<*const dyn WidgetImpl> {
        match self.parent.as_ref() {
            Some(parent) => Some(*parent),
            None => None,
        }
    }

    fn hide(&mut self) {
        self.set_property("visible", false.to_value())
    }

    fn show(&mut self) {
        self.set_property("visible", true.to_value())
    }

    fn visible(&mut self) -> bool {
        self.get_property("visible").unwrap().get::<bool>()
    }

    fn set_focus(&mut self, focus: bool) {
        self.set_property("focus", focus.to_value())
    }

    fn is_focus(&self) -> bool {
        self.get_property("focus").unwrap().get::<bool>()
    }

    fn resize(&mut self, width: i32, height: i32) {
        self.set_property("width", width.to_value());
        self.set_property("height", height.to_value());
    }

    fn width_request(&mut self, width: i32) {
        self.set_property("width", width.to_value());
        self.set_property("width-request", width.to_value());
    }

    fn height_request(&mut self, height: i32) {
        self.set_property("height", height.to_value());
        self.set_property("height-request", height.to_value());
    }

    fn set_halign(&mut self, halign: Align) {
        self.set_property("halign", halign.to_value())
    }

    fn set_valign(&mut self, valign: Align) {
        self.set_property("valign", valign.to_value())
    }

    fn halign(&self) -> Align {
        self.get_property("halign").unwrap().get::<Align>()
    }

    fn valign(&self) -> Align {
        self.get_property("valign").unwrap().get::<Align>()
    }

    fn set_font(&mut self, font: Font) {
        self.font = font;
        self.font_changed();
    }

    fn font(&self) -> Font {
        skia_font_clone(&self.font)
    }

    fn set_font_family(&mut self, family: String) {
        self.font_family = family
    }

    fn font_family(&self) -> &str {
        &self.font_family
    }

    fn size(&self) -> Size {
        let rect = self.rect();
        Size::new(rect.width(), rect.height())
    }

    fn image_rect(&self) -> Rect {
        let mut rect = self.rect();

        let (top, right, bottom, left) = self.margins();
        rect.set_x(rect.x() - left);
        rect.set_y(rect.y() - top);
        rect.set_width(rect.width() + left + right);
        rect.set_height(rect.height() + top + bottom);

        rect
    }

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

    fn background(&self) -> Color {
        self.background
    }

    fn set_background(&mut self, color: Color) {
        self.background = color
    }

    fn margins(&self) -> (i32, i32, i32, i32) {
        (
            self.margins[0],
            self.margins[1],
            self.margins[2],
            self.margins[3],
        )
    }

    fn margin_top(&self) -> i32 {
        self.margins[0]
    }

    fn margin_right(&self) -> i32 {
        self.margins[1]
    }

    fn margin_bottom(&self) -> i32 {
        self.margins[2]
    }

    fn margin_left(&self) -> i32 {
        self.margins[3]
    }

    fn set_margins(&mut self, top: i32, right: i32, bottom: i32, left: i32) {
        self.margins[0] = top;
        self.margins[1] = right;
        self.margins[2] = bottom;
        self.margins[3] = left;
    }

    fn set_margin_top(&mut self, val: i32) {
        self.margins[0] = val;
    }

    fn set_margin_right(&mut self, val: i32) {
        self.margins[1] = val;
    }

    fn set_margin_bottom(&mut self, val: i32) {
        self.margins[2] = val;
    }

    fn set_margin_left(&mut self, val: i32) {
        self.margins[3] = val;
    }

    fn paddings(&self) -> (i32, i32, i32, i32) {
        (
            self.paddings[0],
            self.paddings[1],
            self.paddings[2],
            self.paddings[3],
        )
    }

    fn padding_top(&self) -> i32 {
        self.paddings[0]
    }

    fn padding_right(&self) -> i32 {
        self.paddings[1]
    }

    fn padding_bottom(&self) -> i32 {
        self.paddings[2]
    }

    fn padding_left(&self) -> i32 {
        self.paddings[3]
    }

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
        let size = self.size();
        self.width_request(size.width() as i32 + left + right);
        self.height_request(size.height() as i32 + top + bottom);
    }

    fn set_padding_top(&mut self, mut val: i32) {
        if val < 0 {
            val = 0;
        }
        self.paddings[0] = val;
        let size = self.size();
        self.height_request(size.height() as i32 + val);
    }

    fn set_padding_right(&mut self, mut val: i32) {
        if val < 0 {
            val = 0;
        }
        self.paddings[1] = val;
        let size = self.size();
        self.width_request(size.width() as i32 + val);
    }

    fn set_padding_bottom(&mut self, mut val: i32) {
        if val < 0 {
            val = 0;
        }
        self.paddings[2] = val;
        let size = self.size();
        self.height_request(size.height() as i32 + val);
    }

    fn set_padding_left(&mut self, mut val: i32) {
        if val < 0 {
            val = 0;
        }
        self.paddings[3] = val;
        let size = self.size();
        self.width_request(size.width() as i32 + val);
    }

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
        self.borders[0] = top;
        self.borders[1] = right;
        self.borders[2] = bottom;
        self.borders[3] = left;
    }

    fn set_border_style(&mut self, style: BorderStyle) {
        self.border_style = style;
    }

    fn set_border_color(&mut self, color: Color) {
        self.border_color = color;
    }

    fn borders(&self) -> [f32; 4] {
        self.borders
    }

    fn border_style(&self) -> BorderStyle {
        self.border_style
    }

    fn border_color(&self) -> Color {
        self.border_color
    }

    fn set_cursor_shape(&mut self, cursor: SystemCursorShape) {
        Application::send_message(Message::message_set_cursor_shape(cursor))
    }
}

////////////////////////////////////// WidgetGenericExt //////////////////////////////////////
/// The trait provide some functions include the generic types.
pub trait WidgetGenericExt {
    /// Go to[`Function defination`](WidgetGenericExt::get_parent) (Defined in [`WidgetGenericExt`])
    fn get_parent<T: IsA<Widget> + ObjectType>(&self) -> Option<&T>;

    /// Go to[`Function defination`](WidgetGenericExt::get_child) (Defined in [`WidgetGenericExt`])
    fn get_child<T: IsA<Widget> + ObjectType>(&self) -> Option<&T>;
}

impl<T: WidgetImpl> WidgetGenericExt for T {
    fn get_parent<R: IsA<Widget> + ObjectType>(&self) -> Option<&R> {
        let raw_parent = self.get_raw_parent();
        match raw_parent {
            Some(parent) => unsafe {
                if parent.as_ref().is_none() {
                    return None;
                }
                if parent
                    .as_ref()
                    .unwrap()
                    .object_type()
                    .is_a(R::static_type())
                {
                    (parent as *const R).as_ref()
                } else {
                    None
                }
            },
            None => None,
        }
    }

    fn get_child<R: IsA<Widget> + ObjectType>(&self) -> Option<&R> {
        let raw_child = self.get_raw_child();
        match raw_child {
            Some(child) => unsafe {
                if child.as_ref().is_none() {
                    return None;
                }
                if child.as_ref().unwrap().object_type().is_a(R::static_type()) {
                    (child as *const R).as_ref()
                } else {
                    None
                }
            },
            None => None,
        }
    }
}

////////////////////////////////////// WidgetImpl //////////////////////////////////////
/// Every struct modified by proc-macro [`extends_widget`] should impl this trait manually.
/// WidgetImpl's `paint()` function Will be proxy executated by [`ElementImpl::on_renderer`] method .
#[allow(unused_variables)]
#[allow(unused_mut)]
pub trait WidgetImpl: WidgetExt + ElementImpl + ElementExt + ObjectOperation + ObjectType + ObjectImpl + ParentType {
    /// Invoke this function when widget's size change.
    fn size_hint(&mut self) -> Size {
        let width = self.get_property("width-request").unwrap().get::<i32>();
        let height = self.get_property("height-request").unwrap().get::<i32>();
        Size::new(width, height)
    }

    /// Invoke this function when renderering.
    fn paint(&mut self, mut painter: Painter) {}

    /// Invoke when widget's font was changed.
    fn font_changed(&mut self) {}
}

pub trait WidgetImplExt: WidgetImpl {
    /// @see [`Widget::child_internal`](Widget) <br>
    /// Go to[`Function defination`](WidgetImplExt::child) (Defined in [`WidgetImplExt`])
    fn child<T: WidgetImpl + ElementImpl + IsA<Widget>>(&mut self, child: T);
}

#[cfg(test)]
mod tests {
    use super::WidgetImpl;
    use crate::{prelude::*, widget::WidgetGenericExt};
    use tlib::object::{ObjectImpl, ObjectSubclass};

    #[extends_widget]
    #[derive(Default)]
    struct SubWidget {}

    impl ObjectSubclass for SubWidget {
        const NAME: &'static str = "SubWidget";

        type Type = SubWidget;
        type ParentType = Widget;
    }

    impl ObjectImpl for SubWidget {}

    impl WidgetImpl for SubWidget {}

    #[extends_widget]
    #[derive(Default)]
    struct ChildWidget {}

    impl ObjectSubclass for ChildWidget {
        const NAME: &'static str = "ChildWidget";

        type Type = SubWidget;
        type ParentType = Widget;
    }

    impl ObjectImpl for ChildWidget {}

    impl WidgetImpl for ChildWidget {}

    #[test]
    fn test_sub_widget() {
        let mut widget: SubWidget = Object::new(&[("width", &&120), ("height", &&80)]);
        assert_eq!(120, widget.get_property("width").unwrap().get::<i32>());
        assert_eq!(80, widget.get_property("height").unwrap().get::<i32>());

        let child: ChildWidget = Object::new(&[("width", &&120), ("height", &&80)]);
        let child_id = child.id();

        widget.child(child);

        let child_ref = widget.get_child::<ChildWidget>().unwrap();
        assert_eq!(child_ref.id(), child_id);
        assert_eq!(120, child_ref.get_property("width").unwrap().get::<i32>());
        assert_eq!(80, child_ref.get_property("height").unwrap().get::<i32>());
    }
}
