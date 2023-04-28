use super::{drawing_context::DrawingContext, figure::Rect};
use tlib::{
    object::{IsSubclassable, ObjectImpl, ObjectSubclass},
    prelude::*,
};

/// Basic drawing element super type for basic graphics such as triangle, rectangle....
#[extends(Object)]
#[derive(Default)]
pub struct Element {
    window_id: u16,
    rect: Rect,
}

/// Mark `Element` as is subclassable.
impl IsSubclassable for Element {}

impl ObjectSubclass for Element {
    const NAME: &'static str = "Element";

    type Type = Element;

    type ParentType = tlib::Object;
}

impl ObjectImpl for Element {
    fn construct(&mut self) {
        self.parent_construct();
        self.update();
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
}

impl ElementExt for Element {
    fn set_window_id(&mut self, id: u16) {
        self.window_id = id
    }

    fn window_id(&self) -> u16 {
        self.window_id
    }

    fn update(&mut self) {
        self.set_property("invalidate", true.to_value());
    }

    fn force_update(&mut self) {
        self.set_property("invalidate", true.to_value());
        // TODO: firgue out how to invoke `Board`'s `invalidate_visual` obligatory.
    }

    fn rect(&self) -> Rect {
        self.rect
    }

    fn set_fixed_width(&mut self, width: i32) {
        self.rect.set_width(width)
    }

    fn set_fixed_height(&mut self, height: i32) {
        self.rect.set_height(height)
    }

    fn set_fixed_x(&mut self, x: i32) {
        self.rect.set_x(x)
    }

    fn set_fixed_y(&mut self, y: i32) {
        self.rect.set_y(y)
    }

    fn invalidate(&self) -> bool {
        match self.get_property("invalidate") {
            Some(invalidate) => invalidate.get::<bool>(),
            None => false
        }
    }

    fn validate(&mut self) {
        self.set_property("invalidate", false.to_value());
    }
}

/// Every Element's subclass should impl this trait manually, and implements `on_renderer` function. <br>
/// Each subclass which impl [`WidgetImpl`] will impl this trait automatically.
#[reflect_trait]
pub trait ElementImpl: ElementExt + ObjectImpl + ParentType + 'static {
    fn on_renderer(&mut self, cr: &DrawingContext);
}

pub trait ElementAcquire: ElementImpl + Default {}

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

    fn test_is_a<T: IsA<Element>>(obj: T) {
        let element = obj.as_ref();
        assert_eq!(12, element.get_property("prop1").unwrap().get::<i32>());
        assert_eq!("12", element.get_property("prop2").unwrap().get::<String>());

        let element = obj.downcast_ref::<Element>().unwrap();
        assert!(element.invalidate());
    }
}
