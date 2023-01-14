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
    fn construct(&self) {
        self.parent_construct();
        debug!(
            "`ApplicationWindow` construct: static_type: {}",
            Self::static_type().name()
        )
    }
}

impl WidgetImpl for ApplicationWindow {
    fn paint(&mut self, mut painter: Painter) {
        painter.set_antialiasing();
        painter.fill_rect(self.rect().clone(), Color::BLUE);
        painter.set_color(Color::WHITE);

        let mut font = Font::default();
        font.set_size(30.);
        painter.set_font(font);
        painter.draw_text("Hello World", (30, 40));
    }
}

impl ApplicationWindow {
    pub fn new(width: i32, height: i32) -> ApplicationWindow {
        Object::new(&[("width", &width), ("height", &height)])
    }
}
