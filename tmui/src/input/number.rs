use crate::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[extends(Widget)]
pub struct Number {}

impl ObjectSubclass for Number {
    const NAME: &'static str = "Number";
}

impl ObjectImpl for Number {
    fn initialize(&mut self) {}
}

impl WidgetImpl for Number {}

impl Number {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}
