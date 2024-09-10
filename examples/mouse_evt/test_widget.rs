use tlib::events::MouseEvent;
use tmui::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[extends(Widget)]
#[derive(Childable)]
pub struct TestWidget {
    #[child]
    child: Box<Widget>,
}

impl ObjectSubclass for TestWidget {
    const NAME: &'static str = "TestWidget";
}

impl ObjectImpl for TestWidget {
    fn initialize(&mut self) {
        self.set_halign(Align::Center);
        self.set_valign(Align::Center);
        self.set_hexpand(true);
        self.set_vexpand(true);
        self.set_vscale(0.6);
        self.set_hscale(0.6);
        self.set_background(Color::GREY_LIGHT);

        self.child.set_halign(Align::Center);
        self.child.set_valign(Align::Center);
        self.child.set_hexpand(true);
        self.child.set_vexpand(true);
        self.child.set_vscale(0.6);
        self.child.set_hscale(0.6);
        self.child.set_background(Color::GREY_MEDIUM);
    }
}

impl WidgetImpl for TestWidget {
    fn on_mouse_enter(&mut self, _: &MouseEvent) {
        println!("Mouse enter.")
    }

    fn on_mouse_leave(&mut self, _: &MouseEvent) {
        println!("Mouse leave.")
    }

    fn on_mouse_over(&mut self, _: &MouseEvent) {
        println!("Mouse over.")
    }

    fn on_mouse_out(&mut self, _: &MouseEvent) {
        println!("Mouse out.")
    }
}

impl TestWidget {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}
