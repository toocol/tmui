use std::rc::Rc;
use crate::{
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};
use super::{ctrl::RadioControl, Input, InputBounds, InputWrapper};

#[extends(Widget)]
pub struct Radio<T: InputBounds> {
    input_wrapper: Rc<InputWrapper<T>>,
    radio_ctrl: Option<Rc<RadioControl<T>>>,
}

impl<T: InputBounds> ObjectSubclass for Radio<T> {
    const NAME: &'static str = "Radio";
}

impl<T: InputBounds> ObjectImpl for Radio<T> {
    fn construct(&mut self) {
        self.parent_construct();

        self.input_wrapper.init(self.id());
    }
}

impl<T: InputBounds> WidgetImpl for Radio<T> {}

impl<T: InputBounds> Input for Radio<T> {
    type Value = T;

    #[inline]
    fn input_type(&self) -> super::InputType {
        super::InputType::Radio
    }

    #[inline]
    fn input_wrapper(&self) -> &InputWrapper<Self::Value> {
        &self.input_wrapper
    }
}

impl<T: InputBounds> Radio<T> {
    #[inline]
    pub fn new() -> Box<Self> {
        Object::new(&[])
    }

    #[inline]
    pub fn bind_control(&mut self, control: Rc<RadioControl<T>>) {
        control.link_wrapper(self.input_wrapper.clone());

        self.radio_ctrl = Some(control);
    }

    #[inline]
    pub fn selected_value(&self) -> Option<T> {
        self.radio_ctrl.as_ref().expect("The radio has not been bound to a control.").value()
    }
}
