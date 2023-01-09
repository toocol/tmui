use std::cell::Cell;
use super::{drawing_context::DrawingContext, figure::Rect};
use tlib::{
    object::{IsSubclassable, ObjectImpl, ObjectSubclass},
    prelude::*,
};

/// Basic drawing element super type for basic graphics such as triangle, rectangle....
#[extends_object]
#[derive(Default)]
pub struct Element {
    invalidate: Cell<bool>,
    rect: Cell<Rect>,
}

/// Mark `Element` as is subclassable.
impl IsSubclassable for Element {}

impl ObjectSubclass for Element {
    const NAME: &'static str = "Element";

    type Type = Element;

    type ParentType = tlib::Object;
}

impl ObjectImpl for Element {
    fn construct(&self) {
        self.parent_construct();

        println!("`Element` construct")
    }
}

/// Elentment extend operation, impl this trait by proc-marcos `extends_element` automaticly.
pub trait ElementExt {
    fn update(&self);

    fn force_update(&self);

    fn rect(&self) -> Rect;

    fn set_rect(&self, point: Rect);

    fn invalidate(&self) -> bool;

    fn validate(&self);
}

impl ElementExt for Element {
    /// Mark element's invalidate field to true, and element will be redrawed in next frame.
    fn update(&self) {
        self.invalidate.set(true);
    }

    /// Mark element's invalidate field to true, and element will be redrawed immediately.
    fn force_update(&self) {
        self.invalidate.set(true);
        // TODO: firgue out how to invoke `Board`'s `invalidate_visual` obligatory.
    }

    fn rect(&self) -> Rect {
        self.rect.get()
    }

    fn set_rect(&self, rect: Rect) {
        self.rect.set(rect)
    }

    fn invalidate(&self) -> bool {
        self.invalidate.get()
    }

    fn validate(&self) {
        self.invalidate.set(false)
    }
}

/// Every Element's subclass should impl this trait manually, 
/// and implements `on_renderer` function.
pub trait ElementImpl: ElementExt {
    fn on_renderer(&self, cr: &DrawingContext);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element() {
        let element = Object::new::<Element>(&[("prop1", &&12), ("prop2", &"12")]);
        element.set_rect(Rect::from((100, 100, 50, 50)));
        element.update();
        assert_eq!(12, element.get_property("prop1").unwrap().get());
        assert_eq!("12", element.get_property("prop2").unwrap().get::<String>());
        test_is_a(element);
    }

    fn test_is_a<T: IsA<Element>>(obj: T) {
        let element = obj.as_ref();
        assert_eq!(12, element.get_property("prop1").unwrap().get());
        assert_eq!("12", element.get_property("prop2").unwrap().get::<String>());

        let element = obj.downcast_ref::<Element>().unwrap();
        assert_eq!((100, 100, 50, 50), element.rect().into());
        assert!(element.invalidate());
    }
}
