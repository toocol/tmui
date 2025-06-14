use super::{board::Board, drawing_context::DrawingContext};
use crate::{application_window::window_id, popup::PopupImpl, widget::WidgetImpl};
use log::error;
use tlib::{
    figure::{FRect, Rect},
    object::{ObjectImpl, ObjectSubclass},
    prelude::*,
    signal, signals,
};

pub(crate) const UPD_VALIDATE: u8 = 0;
pub(crate) const UPD_PARTIAL_INVALIDATE: u8 = 1;
pub(crate) const UPD_FULLY_INVALIDATE: u8 = 2;

/// Basic drawing element super type for basic graphics such as triangle, rectangle....
#[extends(Object)]
pub struct Element {
    window_id: ObjectId,
    rect: FRect,
}

impl ObjectSubclass for Element {
    const NAME: &'static str = "Element";
}

impl ObjectImpl for Element {
    fn construct(&mut self) {
        self.parent_construct();

        self.set_property("invalidate", (true, false).to_value());
        Board::notify_update();

        let try_window_id = window_id();
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
    ///
    /// #### Notice:
    /// This function will clear all the `styles_redraw_region` and `redraw_region`.
    fn update(&mut self);

    /// Mark element's invalidate field to true, and element will be redrawed in next frame.
    ///
    /// @param `propagate`: propagate to widget's children or not.
    fn update_with_propagate(&mut self, propagate: bool);

    /// Mark element's invalidate field to true, and element will be redrawed immediately.
    fn force_update(&mut self);

    /// Get the geometry rect of element which contains element's size and position.
    fn rect(&self) -> Rect;

    /// Get the geometry rect of element which contains element's size and position.
    fn rect_f(&self) -> FRect;

    /// The element was invalidated or not.
    fn invalidate(&self) -> bool;

    /// Get the code represent the update status of element.
    fn update_code(&self) -> u8;

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
        let id = self.element_props().window_id;
        if id == 0 {
            error!("Window id of element {} was `0`. Please call `self.parent_construct()` in function `construct()`.", self.name())
        }

        id
    }

    #[inline]
    fn update(&mut self) {
        element_update(self, UPD_FULLY_INVALIDATE, true);
        self.when_update();
    }

    #[inline]
    fn update_with_propagate(&mut self, propagate: bool) {
        element_update(self, UPD_FULLY_INVALIDATE, propagate);
    }

    #[inline]
    fn force_update(&mut self) {
        element_update(self, UPD_FULLY_INVALIDATE, true);
        Board::force_update();
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
    fn invalidate(&self) -> bool {
        match self.get_property("invalidate") {
            Some(val) => val.get::<(u8, bool)>().0 != UPD_VALIDATE,
            None => true,
        }
    }

    #[inline]
    fn update_code(&self) -> u8 {
        match self.get_property("invalidate") {
            Some(val) => val.get::<(u8, bool)>().0,
            None => UPD_VALIDATE,
        }
    }

    #[inline]
    fn validate(&mut self) {
        self.set_property("invalidate", (UPD_VALIDATE, false).to_value());
    }
}

pub(crate) trait ElementInner {
    /// Set the width of element.
    fn set_fixed_width(&mut self, width: i32);

    /// Set the height of element.
    fn set_fixed_height(&mut self, height: i32);

    /// Set the x position of element.
    fn set_fixed_x(&mut self, x: i32);

    /// Set the y position of element.
    fn set_fixed_y(&mut self, y: i32);
}
macro_rules! element_inner_impl {
    () => {
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
    };
}

impl<T: ElementImpl> ElementInner for T {
    element_inner_impl!();
}
impl ElementInner for dyn ElementImpl {
    element_inner_impl!();
}
impl ElementInner for dyn WidgetImpl {
    element_inner_impl!();
}
impl ElementInner for dyn PopupImpl {
    element_inner_impl!();
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

    #[inline]
    fn when_update(&mut self) {}
}

#[inline]
pub(crate) fn element_update(el: &mut impl ElementImpl, mut upd_code: u8, propagate: bool) {
    let code = el.update_code();
    if code == UPD_FULLY_INVALIDATE {
        upd_code = UPD_FULLY_INVALIDATE;
    }
    el.set_property("invalidate", (upd_code, propagate).to_value());

    if code == UPD_VALIDATE {
        Board::notify_update();
        emit!(el, invalidated());
    }
}

pub trait ElementAcquire: ElementImpl + Default {}

/// The hierarchy of widget on the z-axis, the higher the numerical value,
/// the higher the widget position
pub(crate) const TOP_Z_INDEX: u64 = 1000000000000;
pub trait HierachyZ {
    fn z_index(&self) -> u64;

    fn set_z_index(&mut self, z_index: u64);
}
macro_rules! hierarchy_z_impl {
    () => {
        #[inline]
        fn z_index(&self) -> u64 {
            match self.get_property("z_index") {
                Some(val) => val.get(),
                None => 0,
            }
        }

        #[inline]
        fn set_z_index(&mut self, z_index: u64) {
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

pub(crate) trait RenderOrder {
    fn get_render_order(&self) -> usize;

    fn set_render_order(&mut self, render_order: usize);
}
macro_rules! render_order_impl {
    () => {
        #[inline]
        fn get_render_order(&self) -> usize {
            match self.get_property("render_order") {
                Some(val) => val.get(),
                None => 0,
            }
        }

        #[inline]
        fn set_render_order(&mut self, order: usize) {
            self.set_property("render_order", order.to_value())
        }
    };
}
impl<T: ElementImpl> RenderOrder for T {
    render_order_impl!();
}
impl RenderOrder for dyn ElementImpl {
    render_order_impl!();
}
impl RenderOrder for dyn WidgetImpl {
    render_order_impl!();
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
