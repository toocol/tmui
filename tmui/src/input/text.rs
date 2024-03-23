use crate::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};
use super::{Input, InputWrapper};

const TEXT_DEFAULT_WIDTH: u32 = 80;

#[extends(Widget)]
pub struct Text {
    input_wrapper: InputWrapper<String>,
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
    }
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
