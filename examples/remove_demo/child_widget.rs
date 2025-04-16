use tmui::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[extends(Widget)]
pub struct ChildWidget {}

impl ObjectSubclass for ChildWidget {
    const NAME: &'static str = "ChildWidget";
}

impl ObjectImpl for ChildWidget {
    fn construct(&mut self) {
        self.parent_construct();

        self.width_request(200);
        self.height_request(200);
        self.set_background(Color::GREY_LIGHT);

        let mut child: Box<Widget> = Object::new(&[]);
        child.set_halign(Align::Center);
        child.set_valign(Align::Center);
        child.set_background(Color::YELLOW);
        child.width_request(100);
        child.height_request(100);

        let mut child_c: Box<Widget> = Object::new(&[]);
        child_c.set_halign(Align::Center);
        child_c.set_valign(Align::Center);
        child_c.set_background(Color::GREEN);
        child_c.width_request(50);
        child_c.height_request(50);
        child.child(child_c);

        self.child(child);
    }
}

impl WidgetImpl for ChildWidget {}

impl ChildWidget {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}
