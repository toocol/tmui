use tlib::events::KeyEvent;
use tmui::{
   prelude::*,
   tlib::object::{ObjectImpl, ObjectSubclass},
   widget::WidgetImpl,
};

#[extends(Widget)]
pub struct KeyWidget {}

impl ObjectSubclass for KeyWidget {
   const NAME: &'static str = "KeyWidget";
}

impl ObjectImpl for KeyWidget {
    fn construct(&mut self) {
        self.parent_construct();

        self.set_focus(true);
        self.set_vexpand(true);
        self.set_hexpand(true);
        self.set_background(Color::from_rgb(120, 120, 120));
    }
}

impl WidgetImpl for KeyWidget {
    fn on_key_pressed(&mut self, event: &KeyEvent) {
        println!("Receive key pressed: {:?}", event);
    }

    fn on_key_released(&mut self, event: &KeyEvent) {
        println!("Receive key pressed: {:?}", event);
    }
}

impl KeyWidget {
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}