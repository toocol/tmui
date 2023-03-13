use std::mem::size_of;

use crate::{
    namespace::{KeyCode, KeyboardModifier, MouseButton, AsNumeric},
    prelude::{StaticType, ToValue},
    values::{FromBytes, FromValue, ToBytes},
    Type, Value, implements_enum_value,
};

pub trait Event {
    fn type_(&self) -> EventType;
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EventType {
    None = 0,
    MouseButtonPress,
    MouseButtonRelease,
    MouseButtonDoubleClick,
    MouseMove,
    MouseWhell,
    MouseEnter,
    MouseLeave,
    KeyPress,
    KeyRelease,
    FocusIn,
    FocusOut,
    Resize,
    Paint,
}
impl From<u8> for EventType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::MouseButtonPress,
            2 => Self::MouseButtonRelease,
            3 => Self::MouseButtonDoubleClick,
            4 => Self::MouseMove,
            5 => Self::MouseWhell,
            6 => Self::MouseEnter,
            7 => Self::MouseLeave,
            8 => Self::KeyPress,
            9 => Self::KeyRelease,
            10 => Self::FocusIn,
            11 => Self::FocusOut,
            12 => Self::Resize,
            13 => Self::Paint,
            _ => unimplemented!(),
        }
    }
}
impl AsNumeric<u8> for EventType {
    fn as_numeric(&self) -> u8 {
        *self as u8
    }
}
implements_enum_value!(EventType, u8);

/////////////////////////////////////////////////////////////////////////////////////
/// [`KeyEvent`] Keyboard press/release events.
/////////////////////////////////////////////////////////////////////////////////////
pub struct KeyEvent {
    type_: EventType,
    key_code: KeyCode,
    text: String,
    modifier: Option<KeyboardModifier>,
}
impl KeyEvent {
    pub fn new(
        type_: EventType,
        key_code: KeyCode,
        text: String,
        modifier: Option<KeyboardModifier>,
    ) -> Self {
        let type_ = match type_ {
            EventType::KeyPress => type_,
            EventType::KeyRelease => type_,
            _ => unimplemented!(),
        };
        Self {
            type_,
            key_code,
            text,
            modifier,
        }
    }

    pub fn key_code(&self) -> KeyCode {
        self.key_code
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn modifier(&self) -> Option<KeyboardModifier> {
        self.modifier
    }
}
impl Event for KeyEvent {
    fn type_(&self) -> EventType {
        self.type_
    }
}
impl StaticType for KeyEvent {
    fn static_type() -> Type {
        Type::from_name("KeyEvent")
    }

    fn bytes_len() -> usize {
        0
    }
}

pub type Position = (i32, i32);
/////////////////////////////////////////////////////////////////////////////////////
/// [`MouseEvent`] Mouse press/release events.
/////////////////////////////////////////////////////////////////////////////////////
pub struct MouseEvent {
    type_: EventType,
    position: Position,
    mouse_button: MouseButton,
    modifier: KeyboardModifier,
}

impl MouseEvent {
    pub fn new(
        type_: EventType,
        position: Position,
        mouse_button: MouseButton,
        modifier: KeyboardModifier,
    ) -> Self {
        let type_ = match type_ {
            EventType::MouseButtonPress => type_,
            EventType::MouseButtonRelease => type_,
            EventType::MouseButtonDoubleClick => type_,
            _ => unimplemented!(),
        };

        Self {
            type_,
            position,
            mouse_button,
            modifier,
        }
    }

    pub fn position(&self) -> Position {
        self.position
    }

    pub fn mouse_button(&self) -> MouseButton {
        self.mouse_button
    }

    pub fn modifier(&self) -> KeyboardModifier {
        self.modifier
    }
}

impl Event for MouseEvent {
    fn type_(&self) -> EventType {
        self.type_
    }
}
