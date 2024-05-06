use crate::{
    label::Label,
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[extends(Widget)]
#[derive(Childable)]
pub struct Button {
    #[child]
    label: Box<Label>,
}

impl ObjectSubclass for Button {
    const NAME: &'static str = "Button";
}

impl ObjectImpl for Button {
    fn initialize(&mut self) {
        self.set_borders(1., 1., 1., 1.);
    }
}

impl WidgetImpl for Button {}

impl Button {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}
