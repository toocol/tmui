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
        self.set_border_color(Color::grey_with(96));

        self.set_background(Color::grey_with(235));
    }
}

impl WidgetImpl for Button {}

impl Button {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}
