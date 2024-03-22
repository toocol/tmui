pub mod checkbox;
pub mod date;
pub mod password;
pub mod radio;
pub mod text;

use std::cell::RefCell;

use tlib::prelude::*;

#[derive(Debug, Clone, Copy, Hash)]
pub enum InputType {
    Text,
    Password,
    Radio,
    Checkbox,
    Date,
}

#[reflect_trait]
pub trait Input {
    /// Get the type of input widget.
    fn input_type(&self) -> InputType;

    /// Disable the input widget, make it unable to interact.
    fn disable(&mut self);

    /// Enable the input widget, make it able to interact.
    fn enable(&mut self);

    /// Whether the widget is enabled or not.
    fn is_enable(&self) -> bool;

    /// The name of input widget.
    fn input_name(&self) -> &str;
}

pub trait InputValueBounds: Clone + Default + 'static {}
impl<T: Clone + Default + 'static> InputValueBounds for T {}

#[derive(Debug, Default)]
pub enum InputValueWrapper<T: InputValueBounds> {
    #[default]
    None,
    Init {
        name: String,
        value: RefCell<T>,
    },
}

impl<T: InputValueBounds> InputValueWrapper<T> {
    #[inline]
    pub fn init(&mut self, name: impl ToString, value: Option<T>) {
        let value = value.or(Some(T::default())).unwrap();
        *self = Self::Init {
            name: name.to_string(),
            value: RefCell::new(value),
        }
    }

    #[inline]
    pub fn set_value(&self, val: T) {
        match self {
            Self::Init { value, .. } => *value.borrow_mut() = val,
            _ => panic!("Value wrapper was not initialized."),
        }
    }

    #[inline]
    pub fn value(&self) -> T {
        match self {
            Self::Init { value, .. } => value.borrow().clone(),
            _ => panic!("Value wrapper was not initialized."),
        }
    }

    #[inline]
    pub fn name(&self) -> String {
        match self {
            Self::Init { name, .. } => name.clone(),
            _ => panic!("Value wrapper was not initialized."),
        }
    }
}
