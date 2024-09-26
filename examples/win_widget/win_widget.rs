use tlib::win_widget;
use tmui::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[extends(Widget)]
#[win_widget]
pub struct MyWinWidget {}

impl ObjectSubclass for MyWinWidget {
    const NAME: &'static str = "MyWinWidget";
}

impl ObjectImpl for MyWinWidget {
    fn initialize(&mut self) {
        self.set_hexpand(true);
        self.set_vexpand(true);

        self.set_background(Color::BLUE);
    }
}

impl WidgetImpl for MyWinWidget {}

impl MyWinWidget {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}
