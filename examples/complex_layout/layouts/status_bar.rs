use tmui::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[extends(Widget)]
pub struct StatusBar {}

impl ObjectSubclass for StatusBar {
    const NAME: &'static str = "StatusBar";
}

impl ObjectImpl for StatusBar {
    fn initialize(&mut self) {
        self.set_background(Color::GREY_LIGHT);

        self.set_hexpand(true);
        self.height_request(20);
    }
}

impl WidgetImpl for StatusBar {}