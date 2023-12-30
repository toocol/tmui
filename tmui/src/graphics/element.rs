use crate::{application_window::current_window_id, widget::WidgetImpl};
use super::{board::Board, drawing_context::DrawingContext};
use tlib::{
    figure::{Rect, CoordRegion, CoordRect},
    object::{ObjectImpl, ObjectSubclass},
    prelude::*,
};

/// Basic drawing element super type for basic graphics such as triangle, rectangle....
#[extends(Object)]
pub struct Element {
    window_id: ObjectId,
    old_rect: Rect,
    rect: Rect,
    redraw_region: CoordRegion,
    styles_redraw_region: CoordRegion,
}

impl ObjectSubclass for Element {
    const NAME: &'static str = "Element";
}

impl ObjectImpl for Element {
    fn construct(&mut self) {
        self.parent_construct();

        self.update();

        let try_window_id = current_window_id();
        if try_window_id != 0 {
            self.window_id = try_window_id
        }
    }
}

/// Elentment extend operation, impl this trait by proc-marcos `extends_element` automaticly.
pub trait ElementExt: 'static {
    /// Set the application window id which the element belongs to.
    ///
    /// Go to[`Function defination`](ElementExt::window_id) (Defined in [`ElementExt`])
    fn set_window_id(&mut self, id: ObjectId);

    /// Get the application window id which the element belongs to.
    ///
    /// Go to[`Function defination`](ElementExt::window_id) (Defined in [`ElementExt`])
    fn window_id(&self) -> ObjectId;

    /// Mark element's invalidate field to true, and element will be redrawed in next frame.
    ///
    /// Go to[`Function defination`](ElementExt::update) (Defined in [`ElementExt`])
    fn update(&mut self);

    /// Mark element's invalidate field to true, and element will be redrawed immediately.
    ///
    /// Go to[`Function defination`](ElementExt::force_update) (Defined in [`ElementExt`])
    fn force_update(&mut self);

    /// Specified the rects to redraw.
    /// This will result in clipping the drawing area of the widget.(after styles render)
    ///
    /// Go to[`Function defination`](ElementExt::update_rect) (Defined in [`ElementExt`])
    fn update_rect(&mut self, rect: CoordRect);

    /// Specified the styles rects to redraw.
    /// This will result in clipping the drawing area of the widget.(before styles render)
    ///
    /// Go to[`Function defination`](ElementExt::update_rect) (Defined in [`ElementExt`])
    fn update_styles_rect(&mut self, rect: CoordRect);

    /// Specified the region to redraw.
    ///
    /// Go to[`Function defination`](ElementExt::update_region) (Defined in [`ElementExt`])
    fn update_region(&mut self, region: &CoordRegion);

    /// Cleaer the redraw region.
    ///
    /// Go to[`Function defination`](ElementExt::clear_regions) (Defined in [`ElementExt`])
    fn clear_regions(&mut self);

    /// Get the redraw region. <br>
    ///
    /// Go to[`Function defination`](ElementExt::redraw_region) (Defined in [`ElementExt`])
    fn redraw_region(&self) -> &CoordRegion;

    /// Get the styles redraw region. <br>
    ///
    /// Go to[`Function defination`](ElementExt::redraw_region) (Defined in [`ElementExt`])
    fn styles_redraw_region(&self) -> &CoordRegion;

    /// Go to[`Function defination`](ElementExt::rect) (Defined in [`ElementExt`])
    fn rect(&self) -> Rect;

    /// Go to[`Function defination`](ElementExt::set_fixed_width) (Defined in [`ElementExt`])
    fn set_fixed_width(&mut self, width: i32);

    /// Go to[`Function defination`](ElementExt::set_fixed_height) (Defined in [`ElementExt`])
    fn set_fixed_height(&mut self, height: i32);

    /// Go to[`Function defination`](ElementExt::set_fixed_x) (Defined in [`ElementExt`])
    fn set_fixed_x(&mut self, x: i32);

    /// Go to[`Function defination`](ElementExt::set_fixed_y) (Defined in [`ElementExt`])
    fn set_fixed_y(&mut self, y: i32);

    /// Go to[`Function defination`](ElementExt::invalidate) (Defined in [`ElementExt`])
    fn invalidate(&self) -> bool;

    /// Go to[`Function defination`](ElementExt::validate) (Defined in [`ElementExt`])
    fn validate(&mut self);

    /// Go to[`Function defination`](ElementExt::rect_record) (Defined in [`ElementExt`])
    fn rect_record(&self) -> Rect;

    /// Go to[`Function defination`](ElementExt::set_rect_record) (Defined in [`ElementExt`])
    fn set_rect_record(&mut self, rect: Rect);
}

impl ElementExt for Element {
    #[inline]
    fn set_window_id(&mut self, id: ObjectId) {
        self.window_id = id
    }

    #[inline]
    fn window_id(&self) -> ObjectId {
        self.window_id
    }

    #[inline]
    fn update(&mut self) {
        self.set_property("invalidate", true.to_value());
        Board::notify_update()
    }

    #[inline]
    fn force_update(&mut self) {
        self.update();
        Board::force_update();
    }

    #[inline]
    fn update_rect(&mut self, rect: CoordRect) {
        self.redraw_region.add_rect(rect)
    }

    #[inline]
    fn update_styles_rect(&mut self, rect: CoordRect) {
        self.styles_redraw_region.add_rect(rect)
    }

    #[inline]
    fn update_region(&mut self, region: &CoordRegion) {
        if region.is_empty() {
            return
        }
        self.redraw_region.add_region(region)
    }

    #[inline]
    fn clear_regions(&mut self) {
        self.redraw_region.clear();
        self.styles_redraw_region.clear();
    }

    #[inline]
    fn redraw_region(&self) -> &CoordRegion {
        &self.redraw_region
    }

    #[inline]
    fn styles_redraw_region(&self) -> &CoordRegion {
        &self.styles_redraw_region
    }

    #[inline]
    fn rect(&self) -> Rect {
        self.rect
    }

    #[inline]
    fn set_fixed_width(&mut self, width: i32) {
        self.rect.set_width(width)
    }

    #[inline]
    fn set_fixed_height(&mut self, height: i32) {
        self.rect.set_height(height)
    }

    #[inline]
    fn set_fixed_x(&mut self, x: i32) {
        self.rect.set_x(x)
    }

    #[inline]
    fn set_fixed_y(&mut self, y: i32) {
        self.rect.set_y(y)
    }

    #[inline]
    fn invalidate(&self) -> bool {
        match self.get_property("invalidate") {
            Some(val) => val.get(),
            None => true,
        }
    }

    #[inline]
    fn validate(&mut self) {
        self.set_property("invalidate", false.to_value())
    }

    #[inline]
    fn rect_record(&self) -> Rect {
        self.old_rect
    }

    #[inline]
    fn set_rect_record(&mut self, rect: Rect) {
        self.old_rect = rect
    }
}

/// Every Element's subclass should impl this trait manually, and implements `on_renderer` function. <br>
/// Each subclass which impl [`WidgetImpl`] will impl this trait automatically.
#[reflect_trait]
pub trait ElementImpl: ElementExt + ObjectImpl + ObjectOperation + SuperType + 'static {
    fn on_renderer(&mut self, cr: &DrawingContext);
}

pub trait ElementAcquire: ElementImpl + Default {}

/// The hierarchy of widget on the z-axis, the higher the numerical value,
/// the higher the widget position
pub(crate) const TOP_Z_INDEX: u32 = 100000;
pub(crate) trait HierachyZ {
    fn z_index(&self) -> u32;

    fn set_z_index(&mut self, z_index: u32);
}
macro_rules! hierarchy_z_impl {
    () => {
        #[inline]
        fn z_index(&self) -> u32 {
            match self.get_property("z_index") {
                Some(val) => val.get(),
                None => 0,
            }
        }

        #[inline]
        fn set_z_index(&mut self, z_index: u32) {
            self.set_property("z_index", z_index.to_value())
        }
    };
}
impl<T: ElementImpl> HierachyZ for T {
    hierarchy_z_impl!();
}
impl HierachyZ for dyn ElementImpl {
    hierarchy_z_impl!();
}
impl HierachyZ for dyn WidgetImpl {
    hierarchy_z_impl!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element() {
        let mut element = Object::new::<Element>(&[("prop1", &&12), ("prop2", &"12")]);
        element.update();
        assert_eq!(12, element.get_property("prop1").unwrap().get::<i32>());
        assert_eq!("12", element.get_property("prop2").unwrap().get::<String>());
        test_is_a(element);
    }

    fn test_is_a<T: IsA<Element>>(obj: Box<T>) {
        let element = obj.downcast_ref::<Element>().unwrap();
        assert_eq!(12, element.get_property("prop1").unwrap().get::<i32>());
        assert_eq!("12", element.get_property("prop2").unwrap().get::<String>());

        let element = obj.downcast_ref::<Element>().unwrap();
        assert!(element.invalidate());
    }
}
