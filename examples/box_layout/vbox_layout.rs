use derivative::Derivative;
use tlib::object::ObjectSubclass;
use tmui::prelude::*;

use crate::hbox_layout::HBoxLayout;

#[extends(Widget, Layout(VBox))]
#[derive(Childrenable)]
pub struct VBoxLayout {
    #[children]
    hbox_1: Box<HBoxLayout>,

    #[children]
    hbox_2: Box<HBoxLayout>,

    #[children]
    hbox_3: Box<HBoxLayout>,
}

impl ObjectSubclass for VBoxLayout {
    const NAME: &'static str = "VBoxLayout";
}

impl ObjectImpl for VBoxLayout {
    fn construct(&mut self) {
        self.parent_construct();

        self.set_homogeneous(true);

        self.width_request(200);
        self.set_background(Color::CYAN);

        self.hbox_1.set_background(Color::from_rgb(200, 200, 200));
        self.hbox_2.set_background(Color::from_rgb(100, 220, 200));
        self.hbox_3.set_background(Color::from_rgb(220, 100, 220));
    }
}

impl WidgetImpl for VBoxLayout {}

impl VBoxLayout {
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}
