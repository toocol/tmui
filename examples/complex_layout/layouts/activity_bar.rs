use tmui::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[extends(Widget, Layout(VBox))]
pub struct ActivityBar {}

impl ObjectSubclass for ActivityBar {
    const NAME: &'static str = "ActivityBar";
}

impl ObjectImpl for ActivityBar {
    fn initialize(&mut self) {
        self.width_request(30);
        self.set_vexpand(true);
        self.set_background(Color::GREY_DARK);
    }
}

impl WidgetImpl for ActivityBar {}
