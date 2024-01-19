use tmui::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

use crate::child_widget::ChildWidget;

#[extends(Widget, Layout(HBox))]
#[derive(Childrenable)]
pub struct MyWidget {
    #[children]
    w1: Box<ChildWidget>,

    #[children]
    w2: Box<ChildWidget>,
}

impl ObjectSubclass for MyWidget {
    const NAME: &'static str = "MyWidget";
}

impl ObjectImpl for MyWidget {
    fn construct(&mut self) {
        self.parent_construct();

        self.set_hexpand(true);
        self.set_vexpand(true);
        self.set_background(Color::CYAN);

        self.enable_bubble(EventBubble::MOUSE_PRESSED);
        self.enable_bubble(EventBubble::MOUSE_RELEASED);
        self.enable_bubble(EventBubble::MOUSE_WHEEL);
        self.enable_bubble(EventBubble::MOUSE_MOVE);
        self.enable_bubble(EventBubble::KEY_PRESSED);
        self.enable_bubble(EventBubble::KEY_RELEASED);
    }
}

impl WidgetImpl for MyWidget {
    fn on_mouse_pressed(&mut self, event: &tlib::events::MouseEvent) {
        println!("Parent bubble => mouse pressed {:?}", event)
    }

    fn on_mouse_released(&mut self, event: &tlib::events::MouseEvent) {
        println!("Parent bubble => mouse released {:?}", event)
    }

    fn on_mouse_move(&mut self, event: &tlib::events::MouseEvent) {
        println!("Parent bubble => mouse move {:?}", event)
    }

    fn on_mouse_wheel(&mut self, event: &tlib::events::MouseEvent) {
        println!("Parent bubble => mouse wheel {:?}", event)
    }

    fn on_key_pressed(&mut self, event: &tlib::events::KeyEvent) {
        println!("Parent bubble => key pressed {:?}", event)
    }

    fn on_key_released(&mut self,event: &tlib::events::KeyEvent) {
        println!("Parent bubble => key released {:?}", event)
    }
}

impl MyWidget {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}
