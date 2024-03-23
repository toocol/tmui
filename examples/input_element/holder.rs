use tmui::{
    input::text::Text,
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[extends(Widget, Layout(VBox))]
#[derive(Childrenable)]
pub struct Holder {
    #[children]
    text1: Box<Text>,

    #[children]
    text2: Box<Text>,
}

impl ObjectSubclass for Holder {
    const NAME: &'static str = "Holder";
}

impl ObjectImpl for Holder {
    fn initialize(&mut self) {
        self.text1.set_background(Color::RED);
        self.text1.width_request(200);
        self.text1.height_request(40);

        self.text2.set_background(Color::BLUE);
        self.text2.width_request(200);
        self.text2.height_request(40);

        self.set_vexpand(true);
        self.set_hexpand(true);
    }
}

impl WidgetImpl for Holder {}

impl Holder {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}
