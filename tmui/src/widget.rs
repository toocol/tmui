use crate::{
    graphics::{
        drawing_context::DrawingContext,
        element::ElementImpl,
        figure::{Color, Size},
        painter::Painter,
    },
    prelude::*,
};
use log::debug;
use skia_safe::Font;
use std::cell::RefCell;
use tlib::{
    namespace::{Align, Coordinate},
    object::{IsSubclassable, ObjectImpl, ObjectSubclass},
    signals,
};

#[extends_element]
pub struct Widget {
    parent: RefCell<Option<*const dyn WidgetImpl>>,
    child: RefCell<Option<Box<dyn WidgetImpl>>>,

    background: Color,
    font: Font,
    margins: [i32; 4],
    paddings: [i32; 4],
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
            margins: Default::default(),
            paddings: Default::default(),
            element: Default::default(),
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

        debug!("`Widget` construct")
    }

    fn on_property_set(&self, name: &str, value: &Value) {
        debug!("`Widget` on set property, name = {}", name);

        match name {
            "width" => {
                let width = value.get::<i32>();
                self.set_fixed_width(width);
            }
            "height" => {
                let height = value.get::<i32>();
                self.set_fixed_height(height)
            }
            "invalidate" => {
                let invalidate = value.get::<bool>();
                if invalidate {
                    // Notify all the child widget to invalidate, preparing rerenderer after.
                    self.notify_invalidate();
                }
            }
            _ => {}
        }
    }
}

impl WidgetImpl for Widget {}

impl<T: WidgetImpl + WidgetExt> ElementImpl for T {
    fn on_renderer(&mut self, cr: &DrawingContext) {
        let mut painter = Painter::new(cr.canvas(), self);

        debug!("background color: {:?}", self.background());
        // Draw the background color of the Widget.
        painter.fill_rect(
            self.origin_rect(Some(Coordinate::Widget)),
            self.background(),
        );

        self.paint(painter)
    }
}

impl Widget {
    pub fn child_internal<T>(&self, child: T)
    where
        T: WidgetImpl + ElementImpl + IsA<Widget>,
    {
        let child = Box::new(child);
        *self.child.borrow_mut() = Some(child);
    }
}

pub trait WidgetAcquire: WidgetImpl {}

////////////////////////////////////// WidgetExt //////////////////////////////////////
/// The extended actions of [`Widget`], impl by proc-macro [`extends_widget`] automaticly.
pub trait WidgetExt {
    fn as_element(&mut self) -> *mut dyn ElementImpl;

    /// ## Do not invoke this function directly.
    fn set_parent(&self, parent: *const dyn WidgetImpl);

    /// Get the raw pointer of child.
    /// Please use `get_child()` function in [`WidgetGenericExt`]
    fn get_raw_child(&self) -> Option<*const dyn WidgetImpl>;

    /// Get the raw pointer of child.
    /// Please use `get_child()` function in [`WidgetGenericExt`]
    fn get_raw_child_mut(&self) -> Option<*mut dyn WidgetImpl>;

    /// Get the raw pointer of parent.
    /// Please use `get_parent()` function in [`WidgetGenericExt`]
    fn get_raw_parent(&self) -> Option<*const dyn WidgetImpl>;

    /// Request the widget's maximum width.
    fn width_request(&self, width: i32);

    /// Request the widget's maximum width.
    fn height_request(&self, width: i32);

    /// Notify all the child widget to invalidate.
    fn notify_invalidate(&self);

    /// Set alignment on the horizontal direction.
    fn set_halign(&self, halign: Align);

    /// Set alignment on the vertical direction.
    fn set_valign(&self, valign: Align);

    /// Get alignment on the horizontal direction.
    fn halign(&self) -> Align;

    /// Get alignment on the vertical direction.
    fn valign(&self) -> Align;

    /// Set the font of widget.
    fn set_font(&mut self, font: Font);

    /// Get the font of widget.
    fn font(&self) -> Font;

    /// Get the size of widget.
    fn size(&self) -> Size;

    /// Get the area of widget's total image Rect with the margins. <br>
    /// The [`Coordinate`] was `World`.
    fn image_rect(&self) -> Rect;

    /// Get the area of widget's origin Rect. <br>
    /// The default [`Coordinate`] was `World`.
    fn origin_rect(&self, coord: Option<Coordinate>) -> Rect;

    /// Get the area inside the widget's paddings. <br>
    /// The default [`Coordinate`] was `World`.
    fn contents_rect(&self, coord: Option<Coordinate>) -> Rect;

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
}

impl WidgetExt for Widget {
    fn as_element(&mut self) -> *mut dyn ElementImpl {
        self as *mut Self as *mut dyn ElementImpl
    }

    fn set_parent(&self, parent: *const dyn WidgetImpl) {
        *self.parent.borrow_mut() = Some(parent)
    }

    fn get_raw_child(&self) -> Option<*const dyn WidgetImpl> {
        match self.child.borrow().as_ref() {
            Some(child) => Some(child.as_ref() as *const dyn WidgetImpl),
            None => None,
        }
    }

    fn get_raw_child_mut(&self) -> Option<*mut dyn WidgetImpl> {
        match self.child.borrow_mut().as_mut() {
            Some(child) => Some(child.as_mut() as *mut dyn WidgetImpl),
            None => None,
        }
    }

    fn get_raw_parent(&self) -> Option<*const dyn WidgetImpl> {
        match self.parent.borrow().as_ref() {
            Some(parent) => Some(*parent),
            None => None,
        }
    }

    fn width_request(&self, width: i32) {
        self.set_property("width", width.to_value());
        self.set_property("width-request", width.to_value());
    }

    fn height_request(&self, height: i32) {
        self.set_property("height", height.to_value());
        self.set_property("height-request", height.to_value());
    }

    fn notify_invalidate(&self) {
        if let Some(child) = self.get_raw_child() {
            unsafe { child.as_ref().unwrap().update() }
        }
    }

    fn set_halign(&self, halign: Align) {
        self.set_property("halign", halign.to_value())
    }

    fn set_valign(&self, valign: Align) {
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
        let mut font = Font::default();
        font.set_force_auto_hinting(self.font.is_force_auto_hinting());
        font.set_embedded_bitmaps(self.font.is_embedded_bitmaps());
        font.set_subpixel(self.font.is_subpixel());
        font.set_linear_metrics(self.font.is_linear_metrics());
        font.set_embolden(self.font.is_embolden());
        font.set_baseline_snap(self.font.is_baseline_snap());
        font.set_edging(self.font.edging());
        font.set_hinting(self.font.hinting());
        if let Some(typeface) = self.font.typeface() {
            font.set_typeface(typeface);
        }
        font.set_size(self.font.size());
        font.set_scale_x(self.font.scale_x());
        font.set_skew_x(self.font.skew_x());
        font
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
        self.width_request(size.width() + left + right);
        self.height_request(size.height() + top + bottom);
    }

    fn set_padding_top(&mut self, mut val: i32) {
        if val < 0 {
            val = 0;
        }
        self.paddings[0] = val;
        let size = self.size();
        self.height_request(size.height() + val);
    }

    fn set_padding_right(&mut self, mut val: i32) {
        if val < 0 {
            val = 0;
        }
        self.paddings[1] = val;
        let size = self.size();
        self.width_request(size.width() + val);
    }

    fn set_padding_bottom(&mut self, mut val: i32) {
        if val < 0 {
            val = 0;
        }
        self.paddings[2] = val;
        let size = self.size();
        self.height_request(size.height() + val);
    }

    fn set_padding_left(&mut self, mut val: i32) {
        if val < 0 {
            val = 0;
        }
        self.paddings[3] = val;
        let size = self.size();
        self.width_request(size.width() + val);
    }
}

////////////////////////////////////// WidgetGenericExt //////////////////////////////////////
/// The trait provide some functions include the generic types.
pub trait WidgetGenericExt {
    fn get_parent<T: IsA<Widget> + ObjectType>(&self) -> Option<&T>;

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
/// WidgetImpl's `paint()` function Will be proxy executated by ElementImpl `on_renderer` method .
#[allow(unused_variables)]
#[allow(unused_mut)]
pub trait WidgetImpl: WidgetExt + ElementExt + ObjectOperation + ObjectType + ObjectImpl {
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
    fn child<T: WidgetImpl + ElementImpl + IsA<Widget>>(&self, child: T);
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
        let widget: SubWidget = Object::new(&[("width", &&120), ("height", &&80)]);
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
