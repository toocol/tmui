pub mod board;
pub mod drawing_context;
pub mod element;
pub mod figure;

#[cfg(test)]
mod tests {
    use super::element::{Element, ElementExt, ElementImpl};
    use super::figure::Point;
    use tlib::{
        object::{ObjectImpl, ObjectSubclass},
        prelude::*,
    };

    #[extends_element]
    pub struct SubElement {}

    impl ObjectSubclass for SubElement {
        const NAME: &'static str = "SubElement";

        type Type = SubElement;

        type ParentType = Element;
    }

    impl ObjectImpl for SubElement {}

    impl ElementImpl for SubElement {
        fn on_renderer(&self, cr: &super::drawing_context::DrawingContext) {
            cr.line_to(10., 10.)
        }
    }

    #[test]
    fn test_sub_element() {
        let element = Object::new::<SubElement>(&[("prop1", &&12), ("prop2", &"12")]);
        element.set_point(Point::from((100, 100)));
        element.update();
        assert_eq!(12, element.get_property("prop1").unwrap().get());
        assert_eq!("12", element.get_property("prop2").unwrap().get::<String>());
        test_is_a(element);
    }

    fn test_is_a<T: IsA<Element>>(obj: T) {
        let element = obj.as_ref();
        assert_eq!(12, element.get_property("prop1").unwrap().get());
        assert_eq!("12", element.get_property("prop2").unwrap().get::<String>());

        let element = obj.downcast_ref::<SubElement>().unwrap();
        assert_eq!((100, 100), element.point().into());
        assert!(element.invalidate());
    }
}
