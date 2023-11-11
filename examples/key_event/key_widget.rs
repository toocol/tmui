use std::time::Duration;

use tlib::{events::KeyEvent, timer::Timer, connect};
use tmui::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[extends(Widget)]
pub struct KeyWidget {
    timer: Timer,
}

impl ObjectSubclass for KeyWidget {
    const NAME: &'static str = "KeyWidget";
}

impl ObjectImpl for KeyWidget {
    fn construct(&mut self) {
        self.parent_construct();

        self.set_focus(true);
        self.set_mouse_tracking(true);
        self.set_vexpand(true);
        self.set_hexpand(true);
        self.set_background(Color::from_rgb(120, 120, 120));

        self.window().high_load_request(true);

        connect!(self.timer, timeout(), self, timeout());
        self.timer.start(Duration::from_secs(10));
    }
}

impl WidgetImpl for KeyWidget {
    fn on_key_pressed(&mut self, event: &KeyEvent) {
        println!("Receive key pressed: {:?}", event);
    }

    fn on_key_released(&mut self, event: &KeyEvent) {
        println!("Receive key pressed: {:?}", event);
    }

    fn on_mouse_pressed(&mut self, event: &tlib::events::MouseEvent) {
        println!("\nReceive mouse pressed event: {:?}\n", event);
    }

    fn on_mouse_move(&mut self, event: &tlib::events::MouseEvent) {
        println!("\nReceive mouse moved event: {:?}\n", event);
    }
}

impl KeyWidget {
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }

    pub fn timeout(&self) {
        self.window().high_load_request(false);
    }
}
