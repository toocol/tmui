use crate::{
    application_window::ApplicationWindow,
    graphics::{
        border::Border,
        drawing_context::DrawingContext,
        element::{ElementImpl, HierachyZ},
        painter::Painter,
        render_difference::RenderDiffence,
    },
    layout::LayoutManager,
    prelude::*,
    primitive::Message,
};
use derivative::Derivative;
use log::error;
use std::ptr::NonNull;
use tlib::{
    emit,
    events::{InputMethodEvent, KeyEvent, MouseEvent, ReceiveCharacterEvent},
    figure::{Color, FPoint, FontTypeface, Size},
    namespace::{Align, BorderStyle, Coordinate, SystemCursorShape},
    object::{ObjectImpl, ObjectSubclass},
    ptr_mut, signals, skia_safe::region::RegionOp,
};

/// Size hint for widget:
/// 0: minimum size hint
/// 1: normal size hint
/// 2: maximum size hint
pub type SizeHint = (Option<Size>, Option<Size>, Option<Size>);

#[extends(Element)]
pub struct Widget {
    parent: Option<NonNull<dyn WidgetImpl>>,
    child: Option<Box<dyn WidgetImpl>>,
    child_ref: Option<NonNull<dyn WidgetImpl>>,

    old_image_rect: Rect,
    child_image_rect_union: Rect,
    child_overflow_rect: Rect,
    need_update_geometry: bool,

    #[derivative(Default(value = "true"))]
    repaint_when_resize: bool,

    initialized: bool,
    first_rendered: bool,
    rerender_difference: bool,

    #[derivative(Default(value = "Color::WHITE"))]
    background: Color,
    font: Font,
    font_family: String,
    margins: [i32; 4],
    paddings: [i32; 4],
    border: Border,

    width_request: i32,
    height_request: i32,

    /// Widget's width was fixed or not,
    /// `true` when user invoke [`width_request`](WidgetExt::width_request)
    fixed_width: bool,
    /// Widget's height was fixed or not,
    /// `true` when user invoke [`height_request`](WidgetExt::height_request)
    fixed_height: bool,
    /// Used in conjunction with the function [`hexpand`],
    /// if widget was width fixed and hexpanded, `the width ration = width / parent_width`
    fixed_width_ration: f32,
    /// Used in conjunction with the function [`vexpand`],
    /// if widget was height fixed and vexpanded, `the height ration = height / parent_height`
    fixed_height_ration: f32,

    /// Horizontal scalability, if `true` can cause child widget to expand horizontally
    /// with changes in the width of the parent widget.
    hexpand: bool,
    /// The scale factor on horizontal, ratio of child width to parent component,
    /// only effective when widget's `hexpand was true` and `fixed_width was false`.
    ///
    /// ### when parent was widget:
    /// `width ration = hscale / 1`
    ///
    /// ### when parent was coontainer:
    /// `width ration = hscale / parent_children_total_hscales`
    #[derivative(Default(value = "1."))]
    hscale: f32,
    /// Vertical scalability, if `true` can cause child widget to expand vertically
    /// height changes in the height of the parent widget.
    vexpand: bool,
    /// The scale factor on vertical, ratio of child height to parent component,
    /// only effective when widget's hexpand was true.
    ///
    /// ### when parent was widget:
    /// `height ration = vsclae / 1`
    ///
    /// ### when parent was coontainer:
    /// `height ration = vscale / parent_children_total_vscales`
    #[derivative(Default(value = "1."))]
    vscale: f32,

    /// Let the widget track the `MouseMoveEvent`, the default value was false.
    mouse_tracking: bool,
}

////////////////////////////////////// Widget Signals //////////////////////////////////////
pub trait WidgetSignals: ActionExt {
    signals! {
        WidgetSignals:

        /// Emit when widget's size changed.
        ///
        /// @param [`Size`]
        size_changed();

        /// Emit when widget's geometry(size or position) changed.
        ///
        /// @param [`Rect`]
        geometry_changed();

        /// Emit when widget's receive mouse pressed event.
        ///
        /// @param [`MouseEvent`]
        mouse_pressed();

        /// Emit when widget's receive mouse released event.
        ///
        /// @param [`MouseEvent`]
        mouse_released();

        /// Emit when widget's receive mouse double click event.
        ///
        /// @param [`MouseEvent`]
        mouse_double_click();

        /// Emit when widget's receive mouse move event.
        ///
        /// @param [`MouseEvent`]
        mouse_move();

        /// Emit when widget's receive mouse wheel event.
        ///
        /// @param [`MouseEvent`]
        mouse_wheel();

        /// Emit when widget's receive mouse enter event.
        ///
        /// @param [`MouseEvent`]
        mouse_enter();

        /// Emit when widget's receive mouse leave event.
        ///
        /// @param [`MouseEvent`]
        mouse_leave();

        /// Emit when widget's receive key pressed event.
        ///
        /// @param [`KeyEvent`]
        key_pressed();

        /// Emit when widget's receive key released event.
        ///
        /// @param [`KeyEvent`]
        key_released();

        /// Emit when widget's receive character event.
        ///
        /// @param [`ReceiveCharacterEvent`]
        receive_character();

        /// Emit when widget's background changed.
        ///
        /// @param [`Color`]
        background_changed();
    }
}
impl<T: WidgetImpl + ActionExt> WidgetSignals for T {}
impl WidgetSignals for dyn WidgetImpl {}

////////////////////////////////////// Widget Implements //////////////////////////////////////
impl Widget {
    #[inline]
    pub fn child_internal<T>(&mut self, mut child: Box<T>)
    where
        T: WidgetImpl,
    {
        child.set_parent(self);

        ApplicationWindow::initialize_dynamic_component(child.as_mut());

        self.child = Some(child);
        self.child_ref = None;
    }

    #[inline]
    pub fn child_ref_internal(&mut self, child: *mut dyn WidgetImpl) {
        let child_mut = ptr_mut!(child);
        child_mut.set_parent(self);

        ApplicationWindow::initialize_dynamic_component(child_mut);

        self.child = None;
        self.child_ref = NonNull::new(child);
    }

    /// Notify all the child widget to invalidate.
    #[inline]
    fn notify_invalidate(&mut self) {
        if let Some(child) = self.get_child_mut() {
            child.update()
        }
    }

    /// Notify the child to change the visibility.
    #[inline]
    fn notify_visible(&mut self, visible: bool) {
        if let Some(child) = self.get_child_mut() {
            if visible {
                child.show()
            } else {
                child.hide()
            }
        }
    }

    /// Notify the child to change the zindex.
    #[inline]
    fn notify_zindex(&mut self, offset: u32) {
        if let Some(child) = self.get_child_mut() {
            child.set_z_index(child.z_index() + offset);
        }
    }

    /// Notify the child to rerender styles.
    #[inline]
    fn notify_rerender_styles(&mut self) {
        if let Some(child) = self.get_child_mut() {
            child.set_rerender_styles(true)
        }
    }

    #[inline]
    fn notify_minimized(&mut self) {
        if let Some(child) = self.get_child_mut() {
            child.set_minimized(true)
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
            "z_index" => {
                if !ApplicationWindow::window_of(self.window_id()).initialized() {
                    return;
                }
                let new_z_index = value.get::<u32>();
                self.notify_zindex(new_z_index - self.z_index());
            }
            "rerender_styles" => {
                let rerender = value.get::<bool>();
                if rerender {
                    self.notify_rerender_styles()
                }
            }
            "minimized" => {
                let minimized = value.get::<bool>();
                if minimized {
                    self.notify_minimized();
                }
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

        let mut geometry = self.rect();
        if geometry.width() == 0 || geometry.height() == 0 {
            return;
        }
        geometry.set_point(&(0, 0).into());

        let is_shared_widget = self.super_type().is_a(SharedWidget::static_type());

        if !self.first_rendered() || self.rerender_styles() {
            if !is_shared_widget {
                // Draw the background color of the Widget.
                if self.rerender_difference() && self.first_rendered() && !self.window().minimized()
                {
                    let mut border_rect: FRect = self.rect_record().into();
                    border_rect.set_point(&(0, 0).into());
                    self.border_ref().clear_border(
                        &mut painter,
                        border_rect,
                        self.background(),
                    );

                    self.render_difference(&mut painter);
                } else {
                    painter.fill_rect(geometry, self.background());
                }
            }

            self.set_rect_record(self.rect());
            self.set_image_rect_record(self.image_rect());

            // Draw the border of the Widget.
            self.border_ref().render(&mut painter, geometry.into());

            self.set_first_rendered();
            self.set_rerender_styles(false);
        }

        painter.reset();
        painter.set_font(self.font().to_skia_font());
        self.paint(&mut painter);
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

    /// Setter of property `focus`.
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

    /// Update widget's geometry: size, layout...
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

    /// Get the borders of the widget.
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
        let mut child = match self.child {
            Some(ref c) => Some(c.as_ref().as_ptr()),
            None => None,
        };

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
        let mut child = match self.child {
            Some(ref mut c) => Some(c.as_mut().as_ptr_mut()),
            None => None,
        };

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
        let mut child = match self.child {
            Some(ref c) => Some(c.as_ref()),
            None => None,
        };

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
        let mut child = match self.child {
            Some(ref mut c) => Some(c.as_mut()),
            None => None,
        };

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
        self.update();
    }

    #[inline]
    fn show(&mut self) {
        self.set_property("visible", true.to_value());
        self.update();
    }

    #[inline]
    fn visible(&self) -> bool {
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
    fn resize(&mut self, width: Option<i32>, height: Option<i32>) {
        if let Some(width) = width {
            self.set_property("width", width.to_value());
        }
        if let Some(height) = height {
            self.set_property("height", height.to_value());
        }
        if self.id() != self.window_id() {
            if ApplicationWindow::is_initialize_phase() {
                return;
            }
            self.window().layout_change(self);
            self.update();
        }
    }

    #[inline]
    fn width_request(&mut self, width: i32) {
        self.set_property("width", width.to_value());
        self.fixed_width = true;
        self.width_request = width;
        if let Some(parent) = self.get_parent_ref() {
            self.fixed_width_ration = width as f32 / parent.size().width() as f32;
        }
    }

    #[inline]
    fn height_request(&mut self, height: i32) {
        self.set_property("height", height.to_value());
        self.fixed_height = true;
        self.height_request = height;
        if let Some(parent) = self.get_parent_ref() {
            self.fixed_height_ration = height as f32 / parent.size().height() as f32;
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
        if rect.width() != 0 {
            rect.set_width(rect.width() + left + right);
        }
        if rect.height() != 0 {
            rect.set_height(rect.height() + top + bottom);
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

        if top != 0 || right != 0 || bottom != 0 || left != 0 {
            self.need_update_geometry = true;
        } else {
            self.need_update_geometry = false;
        }
    }

    #[inline]
    fn set_margin_top(&mut self, val: i32) {
        self.margins[0] = val;

        if val != 0 {
            self.need_update_geometry = true;
        } else {
            self.need_update_geometry = false;
        }
    }

    #[inline]
    fn set_margin_right(&mut self, val: i32) {
        self.margins[1] = val;

        if val != 0 {
            self.need_update_geometry = true;
        } else {
            self.need_update_geometry = false;
        }
    }

    #[inline]
    fn set_margin_bottom(&mut self, val: i32) {
        self.margins[2] = val;

        if val != 0 {
            self.need_update_geometry = true;
        } else {
            self.need_update_geometry = false;
        }
    }

    #[inline]
    fn set_margin_left(&mut self, val: i32) {
        self.margins[3] = val;

        if val != 0 {
            self.need_update_geometry = true;
        } else {
            self.need_update_geometry = false;
        }
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
        let contents_rect = self.contents_rect(None);
        Point::new(point.x() + contents_rect.x(), point.y() + contents_rect.y())
    }

    #[inline]
    fn map_to_widget(&self, point: &Point) -> Point {
        let rect = self.rect();
        Point::new(point.x() - rect.x(), point.y() - rect.y())
    }

    #[inline]
    fn map_to_global_f(&self, point: &FPoint) -> FPoint {
        let rect = self.rect();
        FPoint::new(
            point.x() + rect.x() as f32,
            point.y() + rect.y() as f32,
        )
    }

    #[inline]
    fn map_to_widget_f(&self, point: &FPoint) -> FPoint {
        let contents_rect = self.contents_rect(None);
        FPoint::new(
            point.x() - contents_rect.x() as f32,
            point.y() - contents_rect.y() as f32,
        )
    }

    #[inline]
    fn mouse_tracking(&self) -> bool {
        self.mouse_tracking
    }

    #[inline]
    fn set_mouse_tracking(&mut self, is_tracking: bool) {
        self.mouse_tracking = is_tracking
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
        self.hscale
    }

    #[inline]
    fn set_hscale(&mut self, hscale: f32) {
        self.hscale = hscale
    }

    #[inline]
    fn vscale(&self) -> f32 {
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
}

////////////////////////////////////// WidgetGenericExt //////////////////////////////////////
/// The trait provide some functions include the generic types.
pub trait WidgetGenericExt {
    /// Go to[`Function defination`](WidgetGenericExt::get_parent) (Defined in [`WidgetGenericExt`])
    fn parent_ref<T: IsA<Widget> + ObjectType>(&self) -> Option<&T>;

    /// Go to[`Function defination`](WidgetGenericExt::get_child) (Defined in [`WidgetGenericExt`])
    fn child_ref<T: IsA<Widget> + ObjectType>(&self) -> Option<&T>;

    /// Go to[`Function defination`](WidgetGenericExt::get_parent) (Defined in [`WidgetGenericExt`])
    fn parent_mut<T: IsA<Widget> + ObjectType>(&mut self) -> Option<&mut T>;

    /// Go to[`Function defination`](WidgetGenericExt::get_child) (Defined in [`WidgetGenericExt`])
    fn child_mut<T: IsA<Widget> + ObjectType>(&mut self) -> Option<&mut T>;
}

impl<T: WidgetImpl> WidgetGenericExt for T {
    fn parent_ref<R: IsA<Widget> + ObjectType>(&self) -> Option<&R> {
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

    fn child_ref<R: IsA<Widget> + ObjectType>(&self) -> Option<&R> {
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

    fn parent_mut<R: IsA<Widget> + ObjectType>(&mut self) -> Option<&mut R> {
        let raw_parent = self.get_raw_parent_mut();
        match raw_parent {
            Some(parent) => unsafe {
                if parent.as_ref().is_none() {
                    return None;
                }
                if parent
                    .as_mut()
                    .unwrap()
                    .object_type()
                    .is_a(R::static_type())
                {
                    (parent as *mut R).as_mut()
                } else {
                    None
                }
            },
            None => None,
        }
    }

    fn child_mut<R: IsA<Widget> + ObjectType>(&mut self) -> Option<&mut R> {
        let raw_child = self.get_raw_child_mut();
        match raw_child {
            Some(child) => unsafe {
                if child.as_ref().is_none() {
                    return None;
                }
                if child.as_ref().unwrap().object_type().is_a(R::static_type()) {
                    (child as *mut R).as_mut()
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
        if !self_rect.contains(point) {
            return false;
        }

        if let Some(child) = self.get_child_ref() {
            if !child.visible() {
                return true;
            }
            return !child.rect().contains(point);
        }
        true
    }
}

////////////////////////////////////// ChildRegionAcquirer //////////////////////////////////////
pub trait ChildRegionAcquirer {
    fn child_region(&self) -> tlib::skia_safe::Region;
}
impl ChildRegionAcquirer for Widget {
    fn child_region(&self) -> tlib::skia_safe::Region {
        let mut region = tlib::skia_safe::Region::new();
        if let Some(child) = self.get_child_ref() {
            let rect: tlib::skia_safe::IRect = child.rect().into();
            region.op_rect(rect, RegionOp::Intersect);
        }
        region
    }
}

////////////////////////////////////// InnerEventProcess //////////////////////////////////////
pub trait InnerEventProcess {
    /// Invoke when widget's receive mouse pressed event.
    fn inner_mouse_pressed(&mut self, event: &MouseEvent);

    /// Invoke when widget's receive mouse released event.
    fn inner_mouse_released(&mut self, event: &MouseEvent);

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
        self.window().set_pressed_widget(self.id());
        emit!(Widget::inner_mouse_pressed => self.mouse_pressed(), event);
    }

    #[inline]
    fn inner_mouse_released(&mut self, event: &MouseEvent) {
        self.window().set_pressed_widget(0);
        emit!(Widget::inner_mouse_released => self.mouse_released(), event);
    }

    #[inline]
    fn inner_mouse_move(&mut self, event: &MouseEvent) {
        emit!(Widget::inner_mouse_move => self.mouse_move(), event);
    }

    #[inline]
    fn inner_mouse_wheel(&mut self, event: &MouseEvent) {
        emit!(Widget::inner_mouse_wheel => self.mouse_wheel(), event);
    }

    #[inline]
    fn inner_mouse_enter(&mut self, event: &MouseEvent) {
        emit!(Widget::inner_mouse_enter => self.mouse_enter(), event);
    }

    #[inline]
    fn inner_mouse_leave(&mut self, event: &MouseEvent) {
        emit!(Widget::inner_mouse_leave => self.mouse_leave(), event);
    }

    #[inline]
    fn inner_key_pressed(&mut self, event: &KeyEvent) {
        emit!(Widget::inner_key_pressed => self.key_pressed(), event);
    }

    #[inline]
    fn inner_key_released(&mut self, event: &KeyEvent) {
        emit!(Widget::inner_key_released => self.key_released(), event);
    }

    #[inline]
    fn inner_receive_character(&mut self, event: &ReceiveCharacterEvent) {
        emit!(Widget::inner_receive_character => self.receive_character(), event);
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
    + SuperType
    + Layout
    + InnerEventProcess
    + PointEffective
    + ChildRegionAcquirer
    + ActionExt
{
    /// Invoke this function when widget's size change.
    fn size_hint(&mut self) -> SizeHint {
        (None, None, None)
    }

    /// The widget can be focused or not, default value was false.
    fn enable_focus(&self) -> bool {
        false
    }

    /// Invoke this function when renderering.
    fn paint(&mut self, painter: &mut Painter) {}

    /// Invoke when widget's font was changed.
    fn font_changed(&mut self) {}

    /// `run_after()` will be invoked when application was started. <br>
    ///
    /// ### Should annotated macro `[run_after]` to enable this function.
    ///
    /// ### Should call `self.parent_run_after()` mannually if override this function.
    fn run_after(&mut self) {
        self.parent_run_after();
    }

    /// Invoke when widget's receive mouse pressed event.
    fn on_mouse_pressed(&mut self, event: &MouseEvent) {}

    /// Invoke when widget's receive mouse released event.
    fn on_mouse_released(&mut self, event: &MouseEvent) {}

    /// Invoke when widget's receive mouse move event.
    ///
    /// The widget does not track mouse movement by default. If need, call function [`set_mouse_tracking`](WidgetExt::set_mouse_tracking)
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
    pub fn as_ptr(&self) -> *const Self {
        self
    }

    #[inline]
    pub fn as_ptr_mut(&mut self) -> *mut Self {
        self
    }

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

    /// @see [`Widget::child_ref_internal`](Widget) <br>
    /// Go to[`Function defination`](WidgetImplExt::child_ref) (Defined in [`WidgetImplExt`])
    fn _child_ref(&mut self, child: *mut dyn WidgetImpl);
}

////////////////////////////////////// Widget Layouts impl //////////////////////////////////////
impl<T: WidgetAcquire> Layout for T {
    fn composition(&self) -> crate::layout::Composition {
        crate::layout::Composition::Default
    }

    fn position_layout(
        &mut self,
        previous: Option<&dyn WidgetImpl>,
        parent: Option<&dyn WidgetImpl>,
        manage_by_container: bool,
    ) {
        LayoutManager::base_widget_position_layout(self, previous, parent, manage_by_container)
    }
}

impl Layout for Widget {
    fn composition(&self) -> crate::layout::Composition {
        crate::layout::Composition::Default
    }

    fn position_layout(&mut self, _: Option<&dyn WidgetImpl>, _: Option<&dyn WidgetImpl>, _: bool) {
    }
}

////////////////////////////////////// ZInddexStep //////////////////////////////////////
pub(crate) trait ZIndexStep {
    /// Get current widget's z-index step, starts from 1, `auto-increacement`.
    fn z_index_step(&mut self) -> u32;
}
macro_rules! z_index_step_impl {
    () => {
        #[inline]
        fn z_index_step(&mut self) -> u32 {
            let step = match self.get_property("z_index_step") {
                Some(val) => val.get(),
                None => 1,
            };
            self.set_property("z_index_step", (step + 1).to_value());
            step
        }
    };
}
impl<T: WidgetImpl> ZIndexStep for T {
    z_index_step_impl!();
}
impl ZIndexStep for dyn WidgetImpl {
    z_index_step_impl!();
}

////////////////////////////////////// ScaleCalculate //////////////////////////////////////
pub(crate) trait ScaleCalculate {
    #[inline]
    fn hscale_calculate(&self) -> f32 {
        1.
    }

    #[inline]
    fn vscale_calculate(&self) -> f32 {
        1.
    }
}

impl ScaleCalculate for dyn WidgetImpl {}

////////////////////////////////////// WindowAcquire //////////////////////////////////////
pub trait WindowAcquire {
    fn window(&self) -> &'static mut ApplicationWindow;
}
impl<T: WidgetImpl> WindowAcquire for T {
    #[inline]
    fn window(&self) -> &'static mut ApplicationWindow {
        ApplicationWindow::window_of(self.window_id())
    }
}
impl WindowAcquire for dyn WidgetImpl {
    #[inline]
    fn window(&self) -> &'static mut ApplicationWindow {
        ApplicationWindow::window_of(self.window_id())
    }
}

#[cfg(test)]
mod tests {
    use super::WidgetImpl;
    use crate::{prelude::*, skia_safe, widget::WidgetGenericExt};
    use tlib::{
        object::{ObjectImpl, ObjectSubclass},
        skia_safe::region::RegionOp,
    };

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

        let child_ref = widget.child_ref::<ChildWidget>().unwrap();
        assert_eq!(child_ref.id(), child_id);
        assert_eq!(120, child_ref.get_property("width").unwrap().get::<i32>());
        assert_eq!(80, child_ref.get_property("height").unwrap().get::<i32>());
    }

    #[test]
    fn test_skia_region() {
        let mut region = skia_safe::Region::new();
        let rect1 = skia_safe::IRect::new(0, 0, 100, 100);
        let rect2 = skia_safe::IRect::new(0, 0, 400, 400);
        region.op_rect(rect1, RegionOp::Union);
        region.op_rect(rect2, RegionOp::Difference);
    }
}
