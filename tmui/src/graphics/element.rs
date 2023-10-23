use crate::{application_window::current_window_id, widget::WidgetImpl};

use super::{board::Board, drawing_context::DrawingContext};
use tlib::{
    figure::{Rect, Region, FRect, FRegion},
    object::{ObjectImpl, ObjectSubclass},
    prelude::*,
};

/// Basic drawing element super type for basic graphics such as triangle, rectangle....
#[extends(Object)]
pub struct Element {
    window_id: u16,
    rect: Rect,
    fixed_size: bool,
    /// The region rect of element's coordinate was `Widget`.
    redraw_region: Region,
    redraw_region_f: FRegion,
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
    fn set_window_id(&mut self, id: u16);

    /// Get the application window id which the element belongs to.
    ///
    /// Go to[`Function defination`](ElementExt::window_id) (Defined in [`ElementExt`])
    fn window_id(&self) -> u16;

    /// Mark element's invalidate field to true, and element will be redrawed in next frame.
    ///
    /// Go to[`Function defination`](ElementExt::update) (Defined in [`ElementExt`])
    fn update(&mut self);

    /// Mark element's invalidate field to true, and element will be redrawed immediately.
    ///
    /// Go to[`Function defination`](ElementExt::force_update) (Defined in [`ElementExt`])
    fn force_update(&mut self);

    /// Specified the region to redraw.
    ///
    /// Go to[`Function defination`](ElementExt::update_region) (Defined in [`ElementExt`])
    fn update_region(&mut self, rect: Rect);

    /// Specified the region to redraw.
    ///
    /// Go to[`Function defination`](ElementExt::update_region_f) (Defined in [`ElementExt`])
    fn update_region_f(&mut self, rect: FRect);

    /// Cleaer the redraw region.
    ///
    /// Go to[`Function defination`](ElementExt::clear_region) (Defined in [`ElementExt`])
    fn clear_region(&mut self);

    /// Cleaer the redraw region.
    ///
    /// Go to[`Function defination`](ElementExt::clear_region_f) (Defined in [`ElementExt`])
    fn clear_region_f(&mut self);

    /// Get the redraw region. <br>
    /// The region rect of element's coordinate was `Widget`.
    ///
    /// Go to[`Function defination`](ElementExt::redraw_region) (Defined in [`ElementExt`])
    fn redraw_region(&self) -> &Region;

    /// Get the redraw region. <br>
    /// The region rect of element's coordinate was `Widget`.
    ///
    /// Go to[`Function defination`](ElementExt::redraw_region) (Defined in [`ElementExt`])
    fn redraw_region_f(&self) -> &FRegion;

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

    /// Go to[`Function defination`](ElementExt::is_fixed_size) (Defined in [`ElementExt`])
    fn is_fixed_size(&self) -> bool;

    /// Go to[`Function defination`](ElementExt::unfixed_size) (Defined in [`ElementExt`])
    fn unfixed_size(&mut self);
}

impl ElementExt for Element {
    #[inline]
    fn set_window_id(&mut self, id: u16) {
        self.window_id = id
    }

    #[inline]
    fn window_id(&self) -> u16 {
        self.window_id
    }

    #[inline]
    fn update(&mut self) {
        self.set_property("invalidate", true.to_value());
        Board::notify_update()
    }

    #[inline]
    fn force_update(&mut self) {
        self.set_property("invalidate", true.to_value());
        // TODO: invoke `Board`'s `invalidate_visual` obligatory.
    }

    #[inline]
    fn update_region(&mut self, rect: Rect) {
        if rect.width() == 0 || rect.height() == 0 {
            return;
        }
        self.redraw_region.add_rect(rect)
    }

    #[inline]
    fn update_region_f(&mut self, rect: FRect) {
        if rect.width() == 0. || rect.height() == 0. {
            return;
        }
        self.redraw_region_f.add_rect(rect)
    }

    #[inline]
    fn clear_region(&mut self) {
        self.redraw_region.clear()
    }

    #[inline]
    fn clear_region_f(&mut self) {
        self.redraw_region_f.clear()
    }

    #[inline]
    fn redraw_region(&self) -> &Region {
        &self.redraw_region
    }

    #[inline]
    fn redraw_region_f(&self) -> &FRegion {
        &self.redraw_region_f
    }

    #[inline]
    fn rect(&self) -> Rect {
        self.rect
    }

    #[inline]
    fn set_fixed_width(&mut self, width: i32) {
        self.fixed_size = true;
        self.rect.set_width(width)
    }

    #[inline]
    fn set_fixed_height(&mut self, height: i32) {
        self.fixed_size = true;
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
    fn is_fixed_size(&self) -> bool {
        self.fixed_size
    }

    #[inline]
    fn unfixed_size(&mut self) {
        self.fixed_size = false
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
