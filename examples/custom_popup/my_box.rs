use tmui::{
   prelude::*,
   tlib::object::{ObjectImpl, ObjectSubclass},
   widget::WidgetImpl,
};
use crate::my_widget::MyWidget;

#[extends(Widget, Layout(VBox))]
#[derive(Childrenable)]
pub struct MyBox {
    #[children]
    content_label: Box<Widget>,

    #[children]
    bottom_bar: Box<MyWidget>,
}

impl ObjectSubclass for MyBox {
   const NAME: &'static str = "MyBox";
}

impl ObjectImpl for MyBox {
    fn construct(&mut self) {
        self.parent_construct();

        self.set_homogeneous(true);

        self.set_hexpand(true);
        self.set_vexpand(true);

        self.content_label.set_vexpand(true);
        self.content_label.set_hexpand(true);
        self.content_label.set_background(Color::RED);

        self.bottom_bar.height_request(25);
        self.bottom_bar.set_hexpand(true);
        self.bottom_bar.set_background(Color::BLUE);
    }
}

impl WidgetImpl for MyBox {
}

impl MyBox {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}