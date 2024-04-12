use crate::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[extends(Widget)]
pub struct SvgIcon {

}

impl ObjectSubclass for SvgIcon {
    const NAME: &'static str = "SvgIcon";
}

impl ObjectImpl for SvgIcon {}

impl WidgetImpl for SvgIcon {}

impl SvgIcon {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}