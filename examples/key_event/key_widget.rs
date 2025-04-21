use std::time::Duration;

use tlib::{connect, events::KeyEvent, timer::Timer};
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
        self.set_mouse_tracking(false);
        self.set_vexpand(true);
        self.set_hexpand(true);
        self.set_hscale(0.7);
        self.set_vscale(0.6);
        self.set_background(Color::GREY_MEDIUM);

        self.set_halign(Align::End);
        self.set_valign(Align::End);

        self.window().high_load_request(true);

        connect!(self.timer, timeout(), self, timeout());
        connect!(self, size_changed(), self, on_size_changed(Size));
        connect!(self, geometry_changed(), self, on_geometry_changed(Rect));
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
    pub fn new() -> Tr<Self> {
        Self::new_alloc()
    }

    pub fn timeout(&self) {
        self.window().high_load_request(false);
    }

    pub fn on_size_changed(&self, size: Size) {
        println!("size changed: {:?}", size)
    }

    pub fn on_geometry_changed(&self, rect: Rect) {
        println!("geometry changed: {:?}", rect)
    }
}
