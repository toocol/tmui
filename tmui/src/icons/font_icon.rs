use crate::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[extends(Widget)]
pub struct FontIcon {
    code: char
}

impl ObjectSubclass for FontIcon {
    const NAME: &'static str = "FontIcon";
}

impl ObjectImpl for FontIcon {}

impl WidgetImpl for FontIcon {}

impl FontIcon {
    #[inline]
    pub fn new(code: char) -> Box<Self> {
        let mut icon: Box<Self> = Object::new(&[]);
        icon.code = code;
        icon
    }
}