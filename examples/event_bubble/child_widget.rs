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

        self.width_request(100);
        self.height_request(40);
        self.set_halign(Align::Center);
        self.set_valign(Align::Center);
        self.set_mouse_tracking(true);
        self.set_margin_right(5);
    }
}

impl WidgetImpl for ChildWidget {
    fn enable_focus(&self) -> bool {
        true
    }

    fn on_mouse_pressed(&mut self, event: &tlib::events::MouseEvent) {
        println!("Child => mouse pressed {:?}", event)
    }

    fn on_mouse_released(&mut self, event: &tlib::events::MouseEvent) {
        println!("Child => mouse released {:?}", event)
    }

    fn on_mouse_move(&mut self, event: &tlib::events::MouseEvent) {
        println!("Child => mouse move {:?}", event)
    }

    fn on_mouse_wheel(&mut self, event: &tlib::events::MouseEvent) {
        println!("Child => mouse wheel {:?}", event)
    }

    fn on_key_pressed(&mut self, event: &tlib::events::KeyEvent) {
        println!("Child => key pressed {:?}", event)
    }

    fn on_key_released(&mut self,event: &tlib::events::KeyEvent) {
        println!("Child => key released {:?}", event)
    }
}