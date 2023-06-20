use crate::{
    application_window::ApplicationWindow,
    graphics::{drawing_context::DrawingContext, element::ElementImpl, painter::Painter},
    layout::LayoutManager,
    platform::Message,
    prelude::*,
};
use derivative::Derivative;
use log::error;
use std::ptr::NonNull;
use tlib::{
    emit,
    events::{InputMethodEvent, KeyEvent, MouseEvent, ReceiveCharacterEvent},
    figure::{Color, FontTypeface, Size},
    namespace::{Align, BorderStyle, Coordinate, SystemCursorShape},
    object::{ObjectImpl, ObjectSubclass},
    signals,
};

/// Size hint for widget:
/// 0: minimum size hint
/// 1: normal size hint
/// 2: maximum size hint
pub type SizeHint = (Size, Size, Size);

#[extends(Element)]
pub struct Widget {
    parent: Option<NonNull<dyn WidgetImpl>>,
    child: Option<Box<dyn WidgetImpl>>,
    initialized: bool,

    #[derivative(Default(value = "Color::WHITE"))]
    background: Color,
    font: Font,
    font_family: String,
    margins: [i32; 4],
    paddings: [i32; 4],
    borders: [f32; 4],
    border_style: BorderStyle,
    #[derivative(Default(value = "Color::BLACK"))]
    border_color: Color,
}

////////////////////////////////////// Widget Signals //////////////////////////////////////
pub trait WidgetSignals: ActionExt {
    signals! {
        /// Emit when widget's size changed.
        size_changed();

        /// Emit when widget's receive mouse pressed event.
        mouse_pressed();

        /// Emit when widget's receive mouse released event.
        mouse_released();

        /// Emit when widget's receive mouse double click event.
        mouse_double_click();

        /// Emit when widget's receive mouse move event.
        mouse_move();

        /// Emit when widget's receive mouse wheel event.
        mouse_wheel();

        /// Emit when widget's receive mouse enter event.
        mouse_enter();

        /// Emit when widget's receive mouse leave event.
        mouse_leave();

        /// Emit when widget's receive key pressed event.
        key_pressed();

        /// Emit when widget's receive key released event.
        key_released();

        /// Emit when widget's receive character event.
        receive_character();

        /// Emit when widget's receive input method event.
        input_method();
    }
}
impl<T: WidgetImpl + ActionExt> WidgetSignals for T {}

////////////////////////////////////// Widget Implements //////////////////////////////////////
impl Widget {
    pub fn child_internal<T>(&mut self, mut child: Box<T>)
    where
        T: WidgetImpl,
    {
        ApplicationWindow::initialize_dynamic_component(self, child.as_mut());
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
}

impl ObjectImpl for Widget {
    fn construct(&mut self) {
        self.parent_construct();

        self.set_halign(Align::default());
        self.set_valign(Align::default());

        self.show();
    }

    fn on_property_set(&mut self, name: &str, value: &Value) {
        self.parent_on_property_set(name, value);

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
            _ => {}
        }
    }
}

impl WidgetImpl for Widget {}

/////////////////////////////////////////////////////////////////////////////////
/// Renderering function for Widget.
/////////////////////////////////////////////////////////////////////////////////
impl<T: WidgetImpl + WidgetExt> ElementImpl for T {
    fn on_renderer(&mut self, cr: &DrawingContext) {
        if !self.visible() {
            return;
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

pub trait WidgetAcquire: WidgetImpl + Default {}

////////////////////////////////////// WidgetExt //////////////////////////////////////
/// The extended actions of [`Widget`], impl by proc-macro [`extends_widget`] automaticly.
pub trait WidgetExt {
    /// Go to[`Function defination`](WidgetExt::name) (Defined in [`WidgetExt`])
    fn name(&self) -> String;

    /// Go to[`Function defination`](WidgetExt::initialized) (Defined in [`WidgetExt`])
    fn initialized(&self) -> bool;

    /// Go to[`Function defination`](WidgetExt::set_initialized) (Defined in [`WidgetExt`])
    fn set_initialized(&mut self, initialized: bool);

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

    /// Get the raw mut pointer of child.
    /// Use [`WidgetGenericExt::get_child()`](WidgetGenericExt::get_child) instead.
    ///
    /// Go to[`Function defination`](WidgetExt::get_raw_child_mut) (Defined in [`WidgetExt`])
    fn get_raw_child_mut(&mut self) -> Option<*mut dyn WidgetImpl>;

    /// Get the raw pointer of parent.
    /// Use [`get_parent()`](WidgetGenericExt::get_parent) instead.
    ///
    /// Go to[`Function defination`](WidgetExt::get_raw_parent) (Defined in [`WidgetExt`])
    fn get_raw_parent(&self) -> Option<*const dyn WidgetImpl>;

    /// Get the raw mut pointer of parent.
    /// Use [`get_parent()`](WidgetGenericExt::get_parent) instead.
    ///
    /// Go to[`Function defination`](WidgetExt::get_raw_parent) (Defined in [`WidgetExt`])
    fn get_raw_parent_mut(&mut self) -> Option<*mut dyn WidgetImpl>;

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

    /// Update widget's geometry: size, layout...
    ///
    /// Go to[`Function defination`](WidgetExt::update_geometry) (Defined in [`WidgetExt`])
    fn update_geometry(&mut self);

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

    /// Get the mut font of widget.
    ///
    /// Go to[`Function defination`](WidgetExt::font_mut) (Defined in [`WidgetExt`])
    fn font_mut(&mut self) -> &mut Font;

    /// Set the font family of Widget.
    ///
    /// Go to[`Function defination`](WidgetExt::set_font_family) (Defined in [`WidgetExt`])
    fn set_font_family(&mut self, family: String);

    /// Get the font family of Widget.
    ///
    /// Go to[`Function defination`](WidgetExt::font_family) (Defined in [`WidgetExt`])
    fn font_family(&self) -> &str;

    /// Get the size of widget. The size does not include the margins.
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

    /// Map the given point to global coordinate.
    ///
    /// Go to[`Function defination`](WidgetExt::map_to_global) (Defined in [`WidgetExt`])
    fn map_to_global(&self, point: &Point) -> Point;

    /// Map the given point to widget coordinate.
    ///
    /// Go to[`Function defination`](WidgetExt::map_to_widget) (Defined in [`WidgetExt`])
    fn map_to_widget(&self, point: &Point) -> Point;
}

impl WidgetExt for Widget {
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
    fn as_element(&mut self) -> *mut dyn ElementImpl {
        self as *mut Self as *mut dyn ElementImpl
    }

    #[inline]
    fn set_parent(&mut self, parent: *mut dyn WidgetImpl) {
        self.parent = NonNull::new(parent)
    }

    #[inline]
    fn get_raw_child(&self) -> Option<*const dyn WidgetImpl> {
        match self.child.as_ref() {
            Some(child) => Some(child.as_ref() as *const dyn WidgetImpl),
            None => None,
        }
    }

    #[inline]
    fn get_raw_child_mut(&mut self) -> Option<*mut dyn WidgetImpl> {
        match self.child.as_mut() {
            Some(child) => Some(child.as_mut() as *mut dyn WidgetImpl),
            None => None,
        }
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
    fn hide(&mut self) {
        self.set_property("visible", false.to_value());
        self.update();
    }

    #[inline]
    fn show(&mut self) {
        self.set_property("visible", true.to_value());
        self.update();
    }

    #[inline]
    fn visible(&mut self) -> bool {
        self.get_property("visible").unwrap().get::<bool>()
    }

    #[inline]
    fn set_focus(&mut self, focus: bool) {
        let id = if focus { self.id() } else { 0 };
        ApplicationWindow::window_of(self.window_id()).set_focused_widget(id)
    }

    #[inline]
    fn is_focus(&self) -> bool {
        ApplicationWindow::window_of(self.window_id()).focused_widget() == self.id()
    }

    #[inline]
    fn resize(&mut self, width: i32, height: i32) {
        self.set_property("width", width.to_value());
        self.set_property("height", height.to_value());
    }

    #[inline]
    fn width_request(&mut self, width: i32) {
        self.set_property("width", width.to_value());
        self.set_property("width-request", width.to_value());
    }

    #[inline]
    fn height_request(&mut self, height: i32) {
        self.set_property("height", height.to_value());
        self.set_property("height-request", height.to_value());
    }

    #[inline]
    fn update_geometry(&mut self) {
        ApplicationWindow::window_of(self.window_id()).layout_change(self);
        self.update();
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
        self.font_changed();
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
    fn set_font_family(&mut self, family: String) {
        let typeface = FontTypeface::builder().family(family.clone()).build();
        self.font_family = family;
        self.font.set_typeface(typeface);
        self.update()
    }

    #[inline]
    fn font_family(&self) -> &str {
        &self.font_family
    }

    #[inline]
    fn size(&self) -> Size {
        let rect = self.rect();
        Size::new(rect.width(), rect.height())
    }

    #[inline]
    fn image_rect(&self) -> Rect {
        let mut rect = self.rect();

        // Rect add the margins.
        let (top, right, bottom, left) = self.margins();
        rect.set_x(rect.x() - left);
        rect.set_y(rect.y() - top);
        rect.set_width(rect.width() + left + right);
        rect.set_height(rect.height() + top + bottom);

        rect
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
    fn background(&self) -> Color {
        self.background
    }

    #[inline]
    fn set_background(&mut self, color: Color) {
        self.background = color
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
    }

    #[inline]
    fn set_margin_top(&mut self, val: i32) {
        self.margins[0] = val;
    }

    #[inline]
    fn set_margin_right(&mut self, val: i32) {
        self.margins[1] = val;
    }

    #[inline]
    fn set_margin_bottom(&mut self, val: i32) {
        self.margins[2] = val;
    }

    #[inline]
    fn set_margin_left(&mut self, val: i32) {
        self.margins[3] = val;
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
        let size = self.size();
        self.height_request(size.height() as i32 + val);
    }

    #[inline]
    fn set_padding_right(&mut self, mut val: i32) {
        if val < 0 {
            val = 0;
        }
        self.paddings[1] = val;
        let size = self.size();
        self.width_request(size.width() as i32 + val);
    }

    #[inline]
    fn set_padding_bottom(&mut self, mut val: i32) {
        if val < 0 {
            val = 0;
        }
        self.paddings[2] = val;
        let size = self.size();
        self.height_request(size.height() as i32 + val);
    }

    #[inline]
    fn set_padding_left(&mut self, mut val: i32) {
        if val < 0 {
            val = 0;
        }
        self.paddings[3] = val;
        let size = self.size();
        self.width_request(size.width() as i32 + val);
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
        self.borders[0] = top;
        self.borders[1] = right;
        self.borders[2] = bottom;
        self.borders[3] = left;
    }

    #[inline]
    fn set_border_style(&mut self, style: BorderStyle) {
        self.border_style = style;
    }

    #[inline]
    fn set_border_color(&mut self, color: Color) {
        self.border_color = color;
    }

    #[inline]
    fn borders(&self) -> [f32; 4] {
        self.borders
    }

    #[inline]
    fn border_style(&self) -> BorderStyle {
        self.border_style
    }

    #[inline]
    fn border_color(&self) -> Color {
        self.border_color
    }

    #[inline]
    fn set_cursor_shape(&mut self, cursor: SystemCursorShape) {
        ApplicationWindow::send_message_with_id(self.window_id(), Message::SetCursorShape(cursor))
    }

    #[inline]
    fn map_to_global(&self, point: &Point) -> Point {
        let contents_rect = self.contents_rect(None);
        Point::new(point.x() + contents_rect.x(), point.y() + contents_rect.y())
    }

    #[inline]
    fn map_to_widget(&self, point: &Point) -> Point {
        let contents_rect = self.contents_rect(None);
        Point::new(point.x() - contents_rect.x(), point.y() - contents_rect.y())
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

////////////////////////////////////// PointEffective //////////////////////////////////////
pub trait PointEffective {
    /// Is the detection point effective.
    fn point_effective(&self, point: &Point) -> bool;
}
impl PointEffective for Widget {
    fn point_effective(&self, point: &Point) -> bool {
        let self_rect = self.rect();
        if let Some(child) = self.get_raw_child() {
            let child_rect = unsafe { child.as_ref().unwrap().rect() };
            return self_rect.contains(point) && !child_rect.contains(point);
        } else {
            return self_rect.contains(point);
        }
    }
}

////////////////////////////////////// InnerEventProcess //////////////////////////////////////
pub trait InnerEventProcess {
    /// Invoke when widget's receive mouse pressed event.
    fn inner_mouse_pressed(&mut self, event: &MouseEvent);

    /// Invoke when widget's receive mouse released event.
    fn inner_mouse_released(&mut self, event: &MouseEvent);

    /// Invoke when widget's receive mouse double click event.
    fn inner_mouse_double_click(&mut self, event: &MouseEvent);

    /// Invoke when widget's receive mouse move event.
    fn inner_mouse_move(&mut self, event: &MouseEvent);

    /// Invoke when widget's receive mouse wheel event.
    fn inner_mouse_wheel(&mut self, event: &MouseEvent);

    /// Invoke when widget's receive mouse enter event.
    fn inner_mouse_enter(&mut self, event: &MouseEvent);

    /// Invoke when widget's receive mouse leave event.
    fn inner_mouse_leave(&mut self, event: &MouseEvent);

    /// Invoke when widget's receive key pressed event.
    fn inner_key_pressed(&mut self, event: &KeyEvent);

    /// Invoke when widget's receive key released event.
    fn inner_key_released(&mut self, event: &KeyEvent);

    /// Invoke when widget's receive character event.
    fn inner_receive_character(&mut self, event: &ReceiveCharacterEvent);
}
impl<T: WidgetImpl + WidgetSignals> InnerEventProcess for T {
    #[inline]
    fn inner_mouse_pressed(&mut self, event: &MouseEvent) {
        if self.enable_focus() {
            self.set_focus(true)
        }
        emit!(self.mouse_pressed(), event);
    }

    #[inline]
    fn inner_mouse_released(&mut self, event: &MouseEvent) {
        emit!(self.mouse_released(), event);
    }

    #[inline]
    fn inner_mouse_double_click(&mut self, event: &MouseEvent) {
        emit!(self.mouse_double_click(), event);
    }

    #[inline]
    fn inner_mouse_move(&mut self, event: &MouseEvent) {
        emit!(self.mouse_move(), event);
    }

    #[inline]
    fn inner_mouse_wheel(&mut self, event: &MouseEvent) {
        emit!(self.mouse_wheel(), event);
    }

    #[inline]
    fn inner_mouse_enter(&mut self, event: &MouseEvent) {
        emit!(self.mouse_enter(), event);
    }

    #[inline]
    fn inner_mouse_leave(&mut self, event: &MouseEvent) {
        emit!(self.mouse_leave(), event);
    }

    #[inline]
    fn inner_key_pressed(&mut self, event: &KeyEvent) {
        emit!(self.key_pressed(), event);
    }

    #[inline]
    fn inner_key_released(&mut self, event: &KeyEvent) {
        emit!(self.key_released(), event);
    }

    #[inline]
    fn inner_receive_character(&mut self, event: &ReceiveCharacterEvent) {
        emit!(self.receive_character(), event);
    }
}

////////////////////////////////////// WidgetImpl //////////////////////////////////////
/// Every struct modified by proc-macro [`extends_widget`] should impl this trait manually.
/// WidgetImpl's `paint()` function Will be proxy executated by [`ElementImpl::on_renderer`] method .
#[allow(unused_variables)]
#[allow(unused_mut)]
#[reflect_trait]
pub trait WidgetImpl:
    WidgetExt
    + ElementImpl
    + ElementExt
    + ObjectOperation
    + ObjectType
    + ObjectImpl
    + ParentType
    + Layout
    + InnerEventProcess
    + PointEffective
{
    /// Invoke this function when widget's size change.
    fn size_hint(&mut self) -> Option<SizeHint> {
        None
    }

    /// The widget can be focused or not, default value was false.
    fn enable_focus(&self) -> bool {
        false
    }

    /// Invoke this function when renderering.
    fn paint(&mut self, mut painter: Painter) {}

    /// Invoke when widget's font was changed.
    fn font_changed(&mut self) {}

    /// Invoke when widget's receive mouse pressed event.
    fn on_mouse_pressed(&mut self, event: &MouseEvent) {}

    /// Invoke when widget's receive mouse released event.
    fn on_mouse_released(&mut self, event: &MouseEvent) {}

    /// Invoke when widget's receive mouse double click event.
    fn on_mouse_double_click(&mut self, event: &MouseEvent) {}

    /// Invoke when widget's receive mouse move event.
    fn on_mouse_move(&mut self, event: &MouseEvent) {}

    /// Invoke when widget's receive mouse wheel event.
    fn on_mouse_wheel(&mut self, event: &MouseEvent) {}

    /// Invoke when widget's receive mouse enter event.
    fn on_mouse_enter(&mut self, event: &MouseEvent) {}

    /// Invoke when widget's receive mouse leave event.
    fn on_mouse_leave(&mut self, event: &MouseEvent) {}

    /// Invoke when widget's receive key pressed event.
    fn on_key_pressed(&mut self, event: &KeyEvent) {}

    /// Invoke when widget's receive key released event.
    fn on_key_released(&mut self, event: &KeyEvent) {}

    /// Invoke when widget's receive character event.
    fn on_receive_character(&mut self, event: &ReceiveCharacterEvent) {}

    /// Invoke when widget's receive input method event.
    fn on_input_method(&mut self, input_method: &InputMethodEvent) {}
}

impl dyn WidgetImpl {
    #[inline]
    pub fn is<T: StaticType + 'static>(&self) -> bool {
        self.object_type().is_a(T::static_type()) && self.as_any().is::<T>()
    }

    #[inline]
    pub fn downcast_ref<T: StaticType + 'static>(&self) -> Option<&T> {
        if self.is::<T>() {
            self.as_any().downcast_ref::<T>()
        } else {
            error!(
                "Downcast widget type mismatched, require {}, get {}",
                self.object_type().name(),
                T::static_type().name()
            );
            None
        }
    }

    #[inline]
    pub fn downcast_mut<T: StaticType + 'static>(&mut self) -> Option<&mut T> {
        if self.is::<T>() {
            self.as_any_mut().downcast_mut::<T>()
        } else {
            error!(
                "Downcast widget type mismatched, require {}, get {}",
                self.object_type().name(),
                T::static_type().name()
            );
            None
        }
    }

    #[inline]
    pub fn downcast<T: StaticType + 'static>(self: Box<Self>) -> Option<Box<T>> {
        let require = self.object_type().name();
        match self.as_any_boxed().downcast::<T>() {
            Ok(v) => Some(v),
            _ => {
                error!(
                    "Downcast widget type mismatched, require {}, get {}",
                    require,
                    T::static_type().name()
                );
                None
            }
        }
    }
}

pub trait WidgetImplExt: WidgetImpl {
    /// @see [`Widget::child_internal`](Widget) <br>
    /// Go to[`Function defination`](WidgetImplExt::child) (Defined in [`WidgetImplExt`])
    fn child<T: WidgetImpl>(&mut self, child: Box<T>);
}

////////////////////////////////////// Widget Layouts impl //////////////////////////////////////
impl<T: WidgetAcquire> Layout for T {
    fn composition(&self) -> crate::layout::Composition {
        crate::layout::Composition::Default
    }

    fn position_layout(
        &mut self,
        previous: &dyn WidgetImpl,
        parent: &dyn WidgetImpl,
        manage_by_container: bool,
    ) {
        LayoutManager::base_widget_position_layout(self, previous, parent, manage_by_container)
    }
}

impl Layout for Widget {
    fn composition(&self) -> crate::layout::Composition {
        crate::layout::Composition::Default
    }

    fn position_layout(&mut self, _: &dyn WidgetImpl, _: &dyn WidgetImpl, _: bool) {}
}

#[cfg(test)]
mod tests {
    use super::WidgetImpl;
    use crate::{prelude::*, widget::WidgetGenericExt};
    use tlib::object::{ObjectImpl, ObjectSubclass};

    #[extends(Widget)]
    struct SubWidget {}

    impl ObjectSubclass for SubWidget {
        const NAME: &'static str = "SubWidget";
    }

    impl ObjectImpl for SubWidget {}

    impl WidgetImpl for SubWidget {}

    #[extends(Widget)]
    struct ChildWidget {}

    impl ObjectSubclass for ChildWidget {
        const NAME: &'static str = "ChildWidget";
    }

    impl ObjectImpl for ChildWidget {}

    impl WidgetImpl for ChildWidget {}

    #[test]
    fn test_sub_widget() {
        let mut widget: Box<SubWidget> = Object::new(&[("width", &&120), ("height", &&80)]);
        assert_eq!(120, widget.get_property("width").unwrap().get::<i32>());
        assert_eq!(80, widget.get_property("height").unwrap().get::<i32>());

        let child: Box<ChildWidget> = Object::new(&[("width", &&120), ("height", &&80)]);
        let child_id = child.id();

        widget.child(child);

        let child_ref = widget.get_child::<ChildWidget>().unwrap();
        assert_eq!(child_ref.id(), child_id);
        assert_eq!(120, child_ref.get_property("width").unwrap().get::<i32>());
        assert_eq!(80, child_ref.get_property("height").unwrap().get::<i32>());
    }
}
