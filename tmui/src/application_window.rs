use crate::{
    graphics::{figure::Color, painter::Painter},
    prelude::*,
    widget::WidgetImpl,
};
use skia_safe::Font;
use tlib::object::{ObjectImpl, ObjectSubclass};
use log::debug;

#[extends_widget]
#[derive(Default)]
pub struct ApplicationWindow {}

impl ObjectSubclass for ApplicationWindow {
    const NAME: &'static str = "ApplicationWindow";

    type Type = ApplicationWindow;

    type ParentType = Object;
}

impl ObjectImpl for ApplicationWindow {
    fn construct(&mut self) {
        self.parent_construct();
        debug!(
            "`ApplicationWindow` construct: static_type: {}",
            Self::static_type().name()
        )
    }
}

impl WidgetImpl for ApplicationWindow {
    fn paint(&mut self, mut _painter: Painter) {}
}

impl ApplicationWindow {
    pub fn new(width: i32, height: i32) -> ApplicationWindow {
        Object::new(&[("width", &width), ("height", &height)])
    }
}
