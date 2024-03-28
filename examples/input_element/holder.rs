use tlib::run_after;
use tmui::{
    input::text::Text,
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[extends(Widget, Layout(VBox))]
#[derive(Childrenable)]
#[run_after]
pub struct Holder {
    #[children]
    text1: Box<Text>,

    #[children]
    text2: Box<Text>,

    #[children]
    text3: Box<Text>,
}

impl ObjectSubclass for Holder {
    const NAME: &'static str = "Holder";
}

impl ObjectImpl for Holder {
    fn initialize(&mut self) {
        // self.text1.set_background(Color::RED);
        self.text1.width_request(200);
        self.text1.height_request(25);
        self.text1.set_margin_left(20);
        self.text1.set_margin_top(10);
        self.text1.set_hexpand(true);
        // self.text1.set_vexpand(true);

        // self.text2.set_background(Color::BLUE);
        // self.text2.width_request(200);
        // self.text2.height_request(25);
        self.text2.set_margin_left(20);
        self.text2.set_margin_top(10);
        // self.text2.set_vexpand(true);

        // self.text3.width_request(200);
        // self.text3.height_request(25);
        self.text3.set_margin_left(20);
        self.text3.set_margin_top(10);
        // self.text3.set_vexpand(true);

        self.set_vexpand(true);
        self.set_spacing(30);
        self.set_hexpand(true);
        self.set_background(Color::GREY_LIGHT);
    }
}

impl WidgetImpl for Holder {
    fn run_after(&mut self) {
        self.parent_run_after();

        self.text1.set_focus(true);
    }
}

impl Holder {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}
