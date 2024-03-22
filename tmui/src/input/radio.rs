use crate::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

use super::{InputValueBounds, InputValueWrapper};

#[extends(Widget)]
pub struct Radio<T: InputValueBounds> {
    value_wrapper: InputValueWrapper<T>
}

impl<T: InputValueBounds> ObjectSubclass for Radio<T> {
    const NAME: &'static str = "Radio";
}

impl<T: InputValueBounds> ObjectImpl for Radio<T> {}

impl<T: InputValueBounds> WidgetImpl for Radio<T> {}

impl<T: InputValueBounds> Radio<T> {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}