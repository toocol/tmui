use super::{board::Board, drawing_context::DrawingContext};
use crate::{application_window::current_window_id, widget::WidgetImpl};
use tlib::{
    figure::{CoordRect, CoordRegion, FRect, Rect},
    object::{ObjectImpl, ObjectSubclass},
    prelude::*,
    signal, signals,
};

/// Basic drawing element super type for basic graphics such as triangle, rectangle....
#[extends(Object)]
pub struct Element {
    window_id: ObjectId,
    rect: FRect,
    redraw_region: CoordRegion,
    styles_redraw_region: CoordRegion,
}

impl ObjectSubclass for Element {
    const NAME: &'static str = "Element";
}

impl ObjectImpl for Element {
    fn construct(&mut self) {
        self.parent_construct();

        self.set_property("invalidate", (true, false).to_value());
        Board::notify_update();

        let try_window_id = current_window_id();
        if try_window_id != 0 {
            self.window_id = try_window_id
        }
    }
}

pub trait ElementSignals: ActionExt {
    signals! {
        ElementSignals:

        /// Emit when element was calling function [`update()`],[`force_update()`]..etc.
        invalidated();
    }
}
impl<T: ElementImpl> ElementSignals for T {}

pub trait ElementPropsAcquire {
    /// Get the property reference of Element.
    fn element_props(&self) -> &Element;

    /// Get the mutable property reference of Element.
    fn element_props_mut(&mut self) -> &mut Element;
}
impl ElementPropsAcquire for Element {
    #[inline]
    fn element_props(&self) -> &Element {
        self
    }

    #[inline]
    fn element_props_mut(&mut self) -> &mut Element {
        self
    }
}

/// Elentment extend operation, impl this trait by proc-marcos `extends_element` automaticly.
pub trait ElementExt {
    /// Get the name of widget.
    fn name(&self) -> String;

    /// Set the application window id which the element belongs to.
    fn set_window_id(&mut self, id: ObjectId);

    /// Get the application window id which the element belongs to.
    fn window_id(&self) -> ObjectId;

    /// Mark element's invalidate field to true, and element will be redrawed in next frame.
    ///
    /// This function will propagate to widget's children.
    fn update(&mut self);

    /// Mark element's invalidate field to true, and element will be redrawed in next frame.
    ///
    /// @param `propagate`: propagate to widget's children or not.
    fn update_with_propagate(&mut self, propagate: bool);

    /// Mark element's invalidate field to true, and element will be redrawed immediately.
    fn force_update(&mut self);

    /// Specified the rects to redraw.
    /// This will result in clipping the drawing area of the widget.(after styles render)
    fn update_rect(&mut self, rect: CoordRect);

    /// Specified the styles rects to redraw.
    /// This will result in clipping the drawing area of the widget.(before styles render)
    fn update_styles_rect(&mut self, rect: CoordRect);

    /// Specified the region to redraw.
    /// @return false if region is empty.
    fn update_region(&mut self, region: &CoordRegion) -> bool;

    /// Specified the styles region to redraw;
    /// @return false if region is empty.
    fn update_styles_region(&mut self, region: &CoordRegion) -> bool;

    /// Cleaer the redraw region.
    fn clear_regions(&mut self);

    /// Get the redraw region. <br>
    fn redraw_region(&self) -> &CoordRegion;

    /// Get the styles redraw region. <br>
    fn styles_redraw_region(&self) -> &CoordRegion;

    /// Get the geometry rect of element which contains element's size and position.
    fn rect(&self) -> Rect;

    /// Get the geometry rect of element which contains element's size and position.
    fn rect_f(&self) -> FRect;

    /// Set the width of element.
    fn set_fixed_width(&mut self, width: i32);

    /// Set the height of element.
    fn set_fixed_height(&mut self, height: i32);

    /// Set the x position of element.
    fn set_fixed_x(&mut self, x: i32);

    /// Set the y position of element.
    fn set_fixed_y(&mut self, y: i32);

    /// The element was invalidated or not.
    fn invalidate(&self) -> bool;

    /// Set the property `invalidate` of element to `false`.
    fn validate(&mut self);
}

impl<T: ElementImpl> ElementExt for T {
    #[inline]
    fn name(&self) -> String {
        self.get_property("name").unwrap().get::<String>()
    }

    #[inline]
    fn set_window_id(&mut self, id: ObjectId) {
        self.element_props_mut().window_id = id
    }

    #[inline]
    fn window_id(&self) -> ObjectId {
        self.element_props().window_id
    }

    #[inline]
    fn update(&mut self) {
        element_update(self, true);
        self.clear_regions();
    }

    #[inline]
    fn update_with_propagate(&mut self, propagate: bool) {
        element_update(self, propagate);
    }

    #[inline]
    fn force_update(&mut self) {
        element_update(self, true);
        Board::force_update();
    }

    #[inline]
    fn update_rect(&mut self, rect: CoordRect) {
        self.element_props_mut().redraw_region.add_rect(rect);
        element_update(self, false);
    }

    #[inline]
    fn update_styles_rect(&mut self, rect: CoordRect) {
        self.element_props_mut().styles_redraw_region.add_rect(rect);
        element_update(self, false);
    }

    #[inline]
    fn update_region(&mut self, region: &CoordRegion) -> bool {
        if region.is_empty() {
            return false;
        }
        self.element_props_mut().redraw_region.add_region(region);
        element_update(self, false);
        true
    }

    #[inline]
    fn update_styles_region(&mut self, region: &CoordRegion) -> bool {
        if region.is_empty() {
            return false;
        }
        self.element_props_mut()
            .styles_redraw_region
            .add_region(region);
        element_update(self, false);
        true
    }

    #[inline]
    fn clear_regions(&mut self) {
        self.element_props_mut().redraw_region.clear();
        self.element_props_mut().styles_redraw_region.clear();
    }

    #[inline]
    fn redraw_region(&self) -> &CoordRegion {
        &self.element_props().redraw_region
    }

    #[inline]
    fn styles_redraw_region(&self) -> &CoordRegion {
        &self.element_props().styles_redraw_region
    }

    #[inline]
    fn rect(&self) -> Rect {
        self.element_props().rect.into()
    }

    #[inline]
    fn rect_f(&self) -> FRect {
        self.element_props().rect
    }

    #[inline]
    fn set_fixed_width(&mut self, width: i32) {
        self.element_props_mut().rect.set_width(width as f32)
    }

    #[inline]
    fn set_fixed_height(&mut self, height: i32) {
        self.element_props_mut().rect.set_height(height as f32)
    }

    #[inline]
    fn set_fixed_x(&mut self, x: i32) {
        self.element_props_mut().rect.set_x(x as f32)
    }

    #[inline]
    fn set_fixed_y(&mut self, y: i32) {
        self.element_props_mut().rect.set_y(y as f32)
    }

    #[inline]
    fn invalidate(&self) -> bool {
        match self.get_property("invalidate") {
            Some(val) => val.get::<(bool, bool)>().0,
            None => true,
        }
    }

    #[inline]
    fn validate(&mut self) {
        self.set_property("invalidate", (false, false).to_value());
    }
}

#[inline]
fn element_update(el: &mut impl ElementImpl, propagate: bool) {
    if !el.invalidate() {
        el.set_property("invalidate", (true, propagate).to_value());
        Board::notify_update();
        emit!(el.invalidated());
    }
}

/// Every Element's subclass should impl this trait manually, and implements `on_renderer` function. <br>
/// Each subclass which impl [`WidgetImpl`] will impl this trait automatically.
#[reflect_trait]
pub trait ElementImpl:
    ElementExt
    + ElementPropsAcquire
    + ObjectImpl
    + ObjectOperation
    + SuperType
    + ElementSignals
    + 'static
{
    #[inline]
    fn before_renderer(&mut self) {}

    fn on_renderer(&mut self, cr: &DrawingContext);

    #[inline]
    fn after_renderer(&mut self) {}
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
        let element = Object::new::<Element>(&[("prop1", &&12), ("prop2", &"12")]);
        assert_eq!(12, element.get_property("prop1").unwrap().get::<i32>());
        assert_eq!("12", element.get_property("prop2").unwrap().get::<String>());
        test_is_a(element);
    }

    fn test_is_a<T: IsA<Element>>(obj: Box<T>) {
        let element = obj.downcast_ref::<Element>().unwrap();
        assert_eq!(12, element.get_property("prop1").unwrap().get::<i32>());
        assert_eq!("12", element.get_property("prop2").unwrap().get::<String>());

        let _ = obj.downcast_ref::<Element>().unwrap();
    }
}
