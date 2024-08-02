use crate::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[extends(Popup, internal = true)]
pub struct InputDialog {}

impl ObjectSubclass for InputDialog {
    const NAME: &'static str = "InputDialog";
}

impl ObjectImpl for InputDialog {}

impl WidgetImpl for InputDialog {}

impl PopupImpl for InputDialog {}

impl InputDialog {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}