use std::mem::size_of;

use crate::{
    implements_enum_value,
    namespace::{AsNumeric, KeyCode, KeyboardModifier, MouseButton},
    prelude::{StaticType, ToValue},
    values::{FromBytes, FromValue, ToBytes},
    Type, Value,
};

pub trait Event {
    fn type_(&self) -> EventType;
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum EventType {
    #[default]
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
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct KeyEvent {
    type_: EventType,
    key_code: KeyCode,
    text: String,
    modifier: KeyboardModifier,
}
impl KeyEvent {
    pub fn new(
        type_: EventType,
        key_code: KeyCode,
        text: String,
        modifier: KeyboardModifier,
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

    pub fn modifier(&self) -> KeyboardModifier {
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

    fn dyn_bytes_len(&self) -> usize {
        EventType::bytes_len()
            + KeyCode::bytes_len()
            + KeyboardModifier::bytes_len()
            + self.text.dyn_bytes_len()
    }
}
impl ToBytes for KeyEvent {
    fn to_bytes(&self) -> Vec<u8> {
        let event_type_len = EventType::bytes_len();
        let key_code_len = KeyCode::bytes_len();
        let modifier_len = KeyboardModifier::bytes_len();
        let mut bytes = vec![0u8; event_type_len + key_code_len + modifier_len];

        let mut idx = 0;
        let type_ = self.type_.to_bytes();
        bytes[idx..idx + event_type_len].copy_from_slice(&type_);
        idx += event_type_len;

        let code = self.key_code.to_bytes();
        bytes[idx..idx + key_code_len].copy_from_slice(&code);
        idx += key_code_len;

        let modifier = self.modifier.to_bytes();
        bytes[idx..idx + modifier_len].copy_from_slice(&modifier);

        let mut text = self.text.as_bytes().to_vec();
        bytes.append(&mut text);
        bytes.push(0);
        bytes
    }
}
impl ToValue for KeyEvent {
    fn to_value(&self) -> Value {
        Value::new(self)
    }

    fn value_type(&self) -> Type {
        Self::static_type()
    }
}
impl FromBytes for KeyEvent {
    fn from_bytes(data: &[u8], _: usize) -> Self {
        let type_len = EventType::bytes_len();
        let code_len = KeyCode::bytes_len();
        let modifier_len = KeyCode::bytes_len();
        let mut idx = 0;

        let type_bytes = &data[idx..idx + type_len];
        let type_ = EventType::from_bytes(type_bytes, type_len);
        idx += type_len;

        let code_bytes = &data[idx..idx + code_len];
        let key_code = KeyCode::from_bytes(code_bytes, code_len);
        idx += code_len;

        let modifier_bytes = &data[idx..idx + modifier_len];
        let modifier = KeyboardModifier::from_bytes(modifier_bytes, modifier_len);
        idx += modifier_len;

        let text_bytes = &data[idx..];
        let text = String::from_bytes(text_bytes, 0);

        Self {
            type_,
            key_code,
            text,
            modifier,
        }
    }
}
impl FromValue for KeyEvent {
    fn from_value(value: &Value) -> Self {
        Self::from_bytes(value.data(), Self::bytes_len())
    }
}

pub type Position = (i32, i32);
/////////////////////////////////////////////////////////////////////////////////////
/// [`MouseEvent`] Mouse press/release events.
/////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
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

impl StaticType for MouseEvent {
    fn static_type() -> Type {
        Type::from_name("MouseEvent")
    }

    fn bytes_len() -> usize {
        EventType::bytes_len()
            + MouseButton::bytes_len()
            + KeyboardModifier::bytes_len()
            + i32::bytes_len() * 2
    }
}
impl ToBytes for MouseEvent {
    fn to_bytes(&self) -> Vec<u8> {
        let type_len = EventType::bytes_len();
        let button_len = MouseButton::bytes_len();
        let modifier_len = KeyboardModifier::bytes_len();
        let mut bytes = vec![0u8; type_len + button_len + modifier_len];
        let mut idx = 0;

        let type_ = self.type_.to_bytes();
        bytes[idx..idx + type_len].copy_from_slice(&type_);
        idx += type_len;

        let button = self.mouse_button.to_bytes();
        bytes[idx..idx + button_len].copy_from_slice(&button);
        idx += button_len;

        let modifier = self.modifier.to_bytes();
        bytes[idx..idx + modifier_len].copy_from_slice(&modifier);

        let mut position_x = self.position.0.to_bytes();
        let mut position_y = self.position.1.to_bytes();
        bytes.append(&mut position_x);
        bytes.append(&mut position_y);

        bytes
    }
}
impl ToValue for MouseEvent {
    fn to_value(&self) -> Value {
        Value::new(self)
    }

    fn value_type(&self) -> Type {
        Self::static_type()
    }
}
impl FromBytes for MouseEvent {
    fn from_bytes(data: &[u8], _: usize) -> Self {
        let type_len = EventType::bytes_len();
        let button_len = MouseButton::bytes_len();
        let modifier_len = KeyboardModifier::bytes_len();
        let mut idx = 0;

        let type_bytes = &data[idx..idx + type_len];
        let type_ = EventType::from_bytes(type_bytes, type_len);
        idx += type_len;

        let button_bytes = &data[idx..idx + button_len];
        let mouse_button = MouseButton::from_bytes(button_bytes, button_len);
        idx += button_len;

        let modifier_bytes = &data[idx..idx + modifier_len];
        let modifier = KeyboardModifier::from_bytes(modifier_bytes, modifier_len);
        idx += modifier_len;

        let i32_len = i32::bytes_len();
        let x = i32::from_bytes(&data[idx..idx + i32_len], i32_len);
        idx += i32_len;
        let y = i32::from_bytes(&data[idx..], i32_len);

        Self {
            type_,
            position: (x, y),
            mouse_button,
            modifier,
        }
    }
}
impl FromValue for MouseEvent {
    fn from_value(value: &Value) -> Self {
        Self::from_bytes(value.data(), Self::bytes_len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_event_value() {
        let key_event = KeyEvent::new(
            EventType::KeyPress,
            KeyCode::KeyA,
            KeyCode::KeyA.to_string(),
            KeyboardModifier::AltModifier,
        );
        let val = key_event.to_value();
        assert_eq!(key_event, val.get::<KeyEvent>())
    }

    #[test]
    fn test_mouse_event_value() {
        let mouse_event = MouseEvent::new(
            EventType::MouseButtonPress,
            (234, 12),
            MouseButton::LeftButton,
            KeyboardModifier::ControlModifier.or(KeyboardModifier::ShiftModifier),
        );
        let val = mouse_event.to_value();
        assert_eq!(mouse_event, val.get::<MouseEvent>())
    }

    #[test]
    fn test_event_value_tuple() {
        let key_event = KeyEvent::new(
            EventType::KeyPress,
            KeyCode::KeyA,
            KeyCode::KeyA.to_string(),
            KeyboardModifier::AltModifier,
        );
        let mouse_event = MouseEvent::new(
            EventType::MouseButtonPress,
            (234, 12),
            MouseButton::LeftButton,
            KeyboardModifier::ControlModifier.or(KeyboardModifier::ShiftModifier),
        );
        let tuple = (key_event, mouse_event);
        let val = tuple.to_value();
        assert_eq!(tuple, val.get::<(KeyEvent, MouseEvent)>())
    }
}
