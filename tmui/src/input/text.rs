use crate::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

use super::InputValueWrapper;

#[extends(Widget)]
pub struct Text {
    value_wrapper: InputValueWrapper<String>,
}

impl ObjectSubclass for Text {
    const NAME: &'static str = "Text";
}

impl ObjectImpl for Text {}

impl WidgetImpl for Text {}

impl Text {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}