pub mod checkbox;
pub mod ctrl;
pub mod date;
pub mod number;
pub mod password;
pub mod radio;
pub mod select;
pub mod text;

use std::cell::{Cell, Ref, RefCell, RefMut};

use log::warn;
use tlib::{object::ObjectId, prelude::*, signal, signals};

#[derive(Debug, Clone, Copy, Hash)]
pub enum InputType {
    Text,
    Password,
    Radio,
    Checkbox,
    Date,
    Select,
    Number,
}

/// All the input widget should implement this trait.
/// Provide some common function related to input.
pub trait Input: InputSignals {
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
        self.input_wrapper().disable();

        emit!(self.available_changed(), false)
    }

    /// Enable the input widget, make it able to interact.
    #[inline]
    fn enable(&mut self) {
        self.input_wrapper().enable();

        emit!(self.available_changed(), true)
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
        if !self.check_value(&val) {
            return;
        }
        self.input_wrapper().set_value(val);

        emit!(self.value_changed())
    }

    /// @return
    ///  - true:  Check success, normal execution of the value setting process.
    ///  - false: Check failed, value setting is ignored.
    #[inline]
    #[allow(unused_variables)]
    fn check_value(&mut self, val: &Self::Value) -> bool {
        true
    }

    #[inline]
    fn set_required(&self, required: bool) {
        self.input_wrapper().set_required(required)
    }

    #[inline]
    fn is_required(&self) -> bool {
        self.input_wrapper().is_required()
    }

    /// Check the value of input element,
    /// different actions will be taken based on different components.
    ///
    /// @return </br>
    /// true : Check passed </br>
    /// false: Check failed
    fn check_required(&mut self) -> bool {
        if self.is_required() {
            self.required_handle()
        } else {
            true
        }
    }

    fn required_handle(&mut self) -> bool;
}

pub trait InputBounds: Clone + Default + 'static {}
impl<T: Clone + Default + 'static> InputBounds for T {}

#[derive(Debug, Derivative)]
#[derivative(Default)]
pub struct InputWrapper<T: InputBounds> {
    id: Cell<ObjectId>,
    initialized: Cell<bool>,
    #[derivative(Default(value = "Cell::new(true)"))]
    enable: Cell<bool>,
    required: Cell<bool>,
    value: RefCell<T>,
}

pub trait InputSignals: ActionExt {
    signals! {
        InputSignals:

        /// Emit when input elements' value has changed.
        value_changed();

        /// Emit when input elements were enabled/disabled.
        ///
        /// @param [`bool`]
        available_changed();
    }
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
    }

    #[inline]
    pub fn id(&self) -> ObjectId {
        self.check_init();
        self.id.get()
    }

    #[inline]
    pub fn set_value(&self, val: T) {
        *self.value.borrow_mut() = val;
    }

    #[inline]
    pub fn value(&self) -> T {
        self.value.borrow().clone()
    }

    #[inline]
    pub fn value_ref(&self) -> Ref<T> {
        self.value.borrow()
    }

    #[inline]
    pub fn value_mut(&self) -> RefMut<T> {
        self.value.borrow_mut()
    }

    #[inline]
    pub fn enable(&self) {
        self.enable.set(true);
    }

    #[inline]
    pub fn disable(&self) {
        self.enable.set(false);
    }

    #[inline]
    pub fn is_enable(&self) -> bool {
        self.enable.get()
    }

    #[inline]
    pub fn set_required(&self, required: bool) {
        self.required.set(required)
    }

    #[inline]
    pub fn is_required(&self) -> bool {
        self.required.get()
    }
}
