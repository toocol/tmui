use tlib::events::{KeyEvent, MouseEvent};

use super::{Input, InputWrapper};
use crate::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};

const TEXT_DEFAULT_WIDTH: u32 = 80;

#[extends(Widget)]
pub struct Text {
    input_wrapper: InputWrapper<String>,
    cursor_position: usize,
}

impl ObjectSubclass for Text {
    const NAME: &'static str = "Text";
}

impl ObjectImpl for Text {
    fn construct(&mut self) {
        self.parent_construct();

        self.input_wrapper.init(self.id());
        self.set_mouse_tracking(true);
    }
}

impl WidgetImpl for Text {
    #[inline]
    fn enable_focus(&self) -> bool {
        true
    }

    fn paint(&mut self, painter: &mut Painter) {
        let val_ref = self.value_ref();
        let val = val_ref.as_str();
        let mut rect = self.contents_rect(Some(Coordinate::Widget));

        if self.is_focus() {
            rect.set_width(rect.width() - 2);
            rect.set_height(rect.height() - 2);
            painter.set_line_width(2.);
            painter.draw_rect(rect);
        } else {
            rect.set_width(rect.width() - 1);
            rect.set_height(rect.height() - 1);
            painter.set_line_width(1.);
            painter.draw_rect(rect);
        }
    }

    fn font_changed(&mut self) {}

    fn on_get_focus(&mut self) {
        println!("{} => Getting focused.", self.name());
        self.update();
    }

    fn on_lose_focus(&mut self) {
        println!("{} => Losing focused.", self.name());
        self.update();
    }

    fn on_key_pressed(&mut self, event: &KeyEvent) {}

    fn on_mouse_pressed(&mut self, event: &MouseEvent) {}

    fn on_mouse_move(&mut self, event: &MouseEvent) {}
}

impl Input for Text {
    type Value = String;

    #[inline]
    fn input_type(&self) -> super::InputType {
        super::InputType::Text
    }

    #[inline]
    fn input_wrapper(&self) -> &InputWrapper<Self::Value> {
        &self.input_wrapper
    }
}

impl Text {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }
}
