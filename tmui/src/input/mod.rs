pub mod checkbox;
pub mod ctrl;
pub mod date;
pub mod password;
pub mod radio;
pub mod text;

use std::cell::{Cell, Ref, RefCell};

use log::warn;
use tlib::object::ObjectId;

#[derive(Debug, Clone, Copy, Hash)]
pub enum InputType {
    Text,
    Password,
    Radio,
    Checkbox,
    Date,
}

/// All the input widget should implement this trait.
/// Provide some common function related to input.
pub trait Input {
    /// The associated type of the input value.
    type Value: InputBounds;

    /**
     * Functions need to be rewritten.
     */
    /// Get the type of input widget.
    fn input_type(&self) -> InputType;

    /// Get the immutable reference of input wrapper.
    fn input_wrapper(&self) -> &InputWrapper<Self::Value>;

    /**
     * Functions have already defined:
     */
    /// Disable the input widget, make it unable to interact.
    #[inline]
    fn disable(&mut self) {
        self.input_wrapper().disable()
    }

    /// Enable the input widget, make it able to interact.
    #[inline]
    fn enable(&mut self) {
        self.input_wrapper().enable()
    }

    /// Whether the widget is enabled or not.
    #[inline]
    fn is_enable(&self) -> bool {
        self.input_wrapper().is_enable()
    }

    /// Get the value of the input widget.
    #[inline]
    fn value(&self) -> Self::Value {
        self.input_wrapper().value()
    }

    /// Get the reference of the value of the input widget.
    #[inline]
    fn value_ref(&self) -> Ref<Self::Value> {
        self.input_wrapper().value_ref()
    }

    /// Set the value of the input widget.
    #[inline]
    fn set_value(&mut self, val: Self::Value) {
        self.input_wrapper().set_value(val)
    }
}

pub trait InputBounds: Clone + Default + 'static {}
impl<T: Clone + Default + 'static> InputBounds for T {}

#[derive(Debug, Default)]
pub struct InputWrapper<T: InputBounds> {
    id: Cell<ObjectId>,
    initialized: Cell<bool>,
    enable: Cell<bool>,
    value: RefCell<T>,
}

impl<T: InputBounds> InputWrapper<T> {
    #[inline]
    fn check_init(&self) {
        if !self.initialized.get() {
            panic!("Input wrapper has not initialized.")
        }
    }

    #[inline]
    pub fn init(&self, id: ObjectId) {
        if self.initialized.get() {
            warn!("Input wrapper can only initialize once.");
            return;
        }

        self.id.set(id);
        self.initialized.set(true);
        self.enable.set(true);
    }

    #[inline]
    pub fn id(&self) -> ObjectId {
        self.check_init();
        self.id.get()
    }

    #[inline]
    pub fn set_value(&self, val: T) {
        self.check_init();
        *self.value.borrow_mut() = val;
    }

    #[inline]
    pub fn value(&self) -> T {
        self.check_init();
        self.value.borrow().clone()
    }

    #[inline]
    pub fn value_ref(&self) -> Ref<T> {
        self.value.borrow()
    }

    #[inline]
    pub fn enable(&self) {
        self.check_init();
        self.enable.set(true);
    }

    #[inline]
    pub fn disable(&self) {
        self.check_init();
        self.enable.set(false);
    }

    #[inline]
    pub fn is_enable(&self) -> bool {
        self.check_init();
        self.enable.get()
    }
}
