use tmui::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[extends(Widget, Layout(Stack))]
pub struct StackWidget {}

impl ObjectSubclass for StackWidget {
    const NAME: &'static str = "StackWidget";
}

impl ObjectImpl for StackWidget {
    fn construct(&mut self) {
        self.parent_construct();

        self.width_request(100);
        self.height_request(100);

        let mut w1: Box<Widget> = Object::new(&[]);
        let mut w2: Box<Widget> = Object::new(&[]);

        w1.set_vexpand(true);
        w1.set_hexpand(true);
        w1.set_background(Color::RED);

        w2.set_vexpand(true);
        w2.set_hexpand(true);
        w2.set_background(Color::BLUE);

        self.add_child(w1);
        self.add_child(w2);
        self.switch();
    }
}

impl WidgetImpl for StackWidget {}
