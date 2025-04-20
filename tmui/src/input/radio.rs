use super::{
    ctrl::RadioControl, Input, InputBounds, InputEle, InputSignals, InputWrapper, ReflectInputEle,
};
use crate::{
    input_ele_impl,
    prelude::*,
    tlib::object::{ObjectImpl, ObjectSubclass},
    widget::WidgetImpl,
};
use std::rc::Rc;

#[extends(Widget)]
pub struct Radio<T: InputBounds> {
    input_wrapper: Rc<InputWrapper<T>>,
    radio_ctrl: Option<Rc<RadioControl<T>>>,
}

impl<T: InputBounds> InputSignals for Radio<T> {}

impl<T: InputBounds> ObjectSubclass for Radio<T> {
    const NAME: &'static str = "Radio";
}

impl<T: InputBounds> ObjectImpl for Radio<T> {
    fn construct(&mut self) {
        self.parent_construct();

        self.input_wrapper.init(self.id());
    }

    #[inline]
    fn type_register(&self, type_registry: &mut TypeRegistry) {
        type_registry.register::<Self, ReflectInputEle>()
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

    #[inline]
    fn required_handle(&mut self) -> bool {
        true
    }
}

impl<T: InputBounds> Radio<T> {
    #[inline]
    pub fn new() -> Tr<Self> {
        Self::new_alloc()
    }

    #[inline]
    pub fn bind_control(&mut self, control: Rc<RadioControl<T>>) {
        control.link_wrapper(self.input_wrapper.clone());

        self.radio_ctrl = Some(control);
    }

    #[inline]
    pub fn selected_value(&self) -> Option<T> {
        self.radio_ctrl
            .as_ref()
            .expect("The radio has not been bound to a control.")
            .value()
    }
}

input_ele_impl!(Radio<T: InputBounds>);
