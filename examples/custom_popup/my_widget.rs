use tlib::events::MouseEvent;
use tmui::{
   prelude::*,
   tlib::object::{ObjectImpl, ObjectSubclass},
   widget::WidgetImpl,
};

use crate::custom_popup::CustomPopup;

#[extends(Widget)]
#[popupable]
pub struct MyWidget {}

impl ObjectSubclass for MyWidget {
   const NAME: &'static str = "MyWidget";
}

impl ObjectImpl for MyWidget {
    fn construct(&mut self) {
        self.parent_construct();

        self.add_popup(CustomPopup::new());
    }
}

impl WidgetImpl for MyWidget {
    fn on_mouse_pressed(&mut self, event: &MouseEvent) {
        println!("MyWidget mouse pressed.");

        if self.get_popup_ref().unwrap().visible() {
            self.hide_popup();
        } else {
            self.show_popup(self.map_to_global(&event.position().into()));
        }
    }
}