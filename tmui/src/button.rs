use crate::{
    graphics::painter::Painter,
    label::Label,
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

#[extends(Widget)]
#[derive(Childable)]
pub struct Button {
    #[child]
    label: Box<Label>,
}

impl ObjectSubclass for Button {
    const NAME: &'static str = "Button";
}

impl ObjectImpl for Button {}

impl WidgetImpl for Button {
    fn paint(&mut self, mut painter: Painter) {
        let rect = self.contents_rect(Some(Coordinate::Widget));
        painter.set_color(Color::BLACK);
        painter.set_line_width(1.);
        painter.draw_rect(rect)
    }
}

impl Button {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}
