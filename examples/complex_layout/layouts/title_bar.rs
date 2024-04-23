use tmui::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[extends(Widget, Layout(HBox))]
pub struct TitleBar {}

impl ObjectSubclass for TitleBar {
    const NAME: &'static str = "TitleBar";
}

impl ObjectImpl for TitleBar {
    fn initialize(&mut self) {
        self.set_background(Color::GREY_LIGHT);
        self.set_hexpand(true);
        self.height_request(30);
    }
}

impl WidgetImpl for TitleBar {}