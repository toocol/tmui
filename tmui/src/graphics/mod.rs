pub mod bitmap;
pub mod board;
pub mod drawing_context;
pub mod element;
pub mod figure;
pub mod painter;

#[cfg(test)]
mod tests {
    use super::element::{Element, ElementAcquire, ElementExt, ElementImpl};
    use super::figure::Rect;
    use crate::prelude::*;
    use tlib::object::{ObjectImpl, ObjectSubclass};

    #[extends(Element)]
    #[derive(Default)]
    pub struct SubElement {}

    impl ObjectSubclass for SubElement {
        const NAME: &'static str = "SubElement";

        type Type = SubElement;

        type ParentType = Element;
    }

    impl ObjectImpl for SubElement {
        fn construct(&mut self) {
            self.parent_construct();

            println!("`SubElement` construct.")
        }
    }

    impl ElementImpl for SubElement {
        fn on_renderer(&mut self, _cr: &super::drawing_context::DrawingContext) {}
    }

    #[test]
    fn test_sub_element() {
        let mut element = Object::new::<SubElement>(&[("prop1", &&12), ("prop2", &"12")]);
        element.update();
        assert_eq!(12, element.get_property("prop1").unwrap().get::<i32>());
        assert_eq!("12", element.get_property("prop2").unwrap().get::<String>());
        test_is_a(element);
    }

    fn test_is_a<T: IsA<Element>>(obj: T) {
        let element = obj.as_ref();
        assert_eq!(12, element.get_property("prop1").unwrap().get::<i32>());
        assert_eq!("12", element.get_property("prop2").unwrap().get::<String>());

        let element = obj.downcast_ref::<SubElement>().unwrap();
        assert!(element.invalidate());
    }
}
