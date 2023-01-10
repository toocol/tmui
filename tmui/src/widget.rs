use crate::{
    graphics::{drawing_context::DrawingContext, element::ElementImpl},
    prelude::*,
};
use tlib::object::{IsSubclassable, ObjectImpl, ObjectSubclass};

#[extends_element]
#[derive(Default)]
pub struct Widget {}

impl ObjectSubclass for Widget {
    const NAME: &'static str = "Widget";

    type Type = Widget;
    type ParentType = Object;
}

pub trait WidgetAcquire: WidgetImpl {}

////////////////////////////////////// WidgetExt //////////////////////////////////////
/// The extended actions of [`Widget`], impl by proc-macro [`extends_widget`] automaticly.
pub trait WidgetExt: ObjectOperation + ObjectImpl {}

impl WidgetExt for Widget {}

////////////////////////////////////// WidgetImpl //////////////////////////////////////
/// Every struct modified by proc-macro [`extends_widget`] should impl this trait manually.
pub trait WidgetImpl: WidgetExt {}

////////////////////////////////////// Widget Implements //////////////////////////////////////
impl IsSubclassable for Widget {}

impl ObjectImpl for Widget {
    fn construct(&self) {
        self.parent_construct();

        println!("`Widget` construct")
    }
}

impl ElementImpl for Widget {
    fn on_renderer(&self, _cr: &DrawingContext) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use tlib::object::{ObjectSubclass, ObjectImpl};

    use crate::prelude::*;

    use super::WidgetImpl;

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

    #[test]
    fn test_sub_widget() {
        let widget: SubWidget = Object::new(&[("width", &&120), ("height", &&80)]);
        assert_eq!(120, widget.get_property("width").unwrap().get::<i32>());
        assert_eq!(80, widget.get_property("height").unwrap().get::<i32>());
    }
}