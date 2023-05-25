use crate::{
    impl_as_any, implements_enum_value,
    namespace::{AsNumeric, KeyCode, KeyboardModifier, MouseButton},
    prelude::*,
    values::{FromBytes, FromValue, ToBytes},
    Type, Value,
};
use std::{any::Any, fmt::Debug, mem::size_of};

pub type Event = Box<dyn EventTrait>;
pub trait EventTrait: 'static + AsAny + Debug + Sync + Send {
    fn type_(&self) -> EventType;
}

#[inline]
pub fn to_key_event(evt: Event) -> Result<Box<KeyEvent>, Box<dyn Any>> {
    evt.as_any_boxed().downcast::<KeyEvent>()
}

#[inline]
pub fn to_mouse_event(evt: Event) -> Result<Box<MouseEvent>, Box<dyn Any>> {
    evt.as_any_boxed().downcast::<MouseEvent>()
}

#[inline]
pub fn to_focus_event(evt: Event) -> Result<Box<FocusEvent>, Box<dyn Any>> {
    evt.as_any_boxed().downcast::<FocusEvent>()
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
impl_as_any!(KeyEvent);
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

    #[inline]
    pub fn key_code(&self) -> KeyCode {
        self.key_code
    }

    #[inline]
    pub fn text(&self) -> &str {
        &self.text
    }

    #[inline]
    pub fn modifier(&self) -> KeyboardModifier {
        self.modifier
    }
}
impl EventTrait for KeyEvent {
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
    n_press: i32,
    delta: i32,
}
impl_as_any!(MouseEvent);
impl MouseEvent {
    pub fn new(
        type_: EventType,
        position: Position,
        mouse_button: MouseButton,
        modifier: KeyboardModifier,
        n_press: i32,
        delta: i32,
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
            n_press,
            delta,
        }
    }

    #[inline]
    pub fn position(&self) -> Position {
        self.position
    }

    #[inline]
    pub fn mouse_button(&self) -> MouseButton {
        self.mouse_button
    }

    #[inline]
    pub fn modifier(&self) -> KeyboardModifier {
        self.modifier
    }

    #[inline]
    pub fn n_press(&self) -> i32 {
        self.n_press
    }

    #[inline]
    pub fn delta(&self) -> i32 {
        self.delta
    }
}

impl EventTrait for MouseEvent {
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
            + i32::bytes_len() * 4
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

        bytes.append(&mut self.n_press.to_bytes());

        bytes.append(&mut self.delta.to_bytes());

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
        let y = i32::from_bytes(&data[idx..idx + i32_len], i32_len);
        idx += i32_len;

        let n_press = i32::from_bytes(&data[idx..idx + i32_len], i32_len);
        idx += i32_len;

        let delta = i32::from_bytes(&data[idx..], i32_len);

        Self {
            type_,
            position: (x, y),
            mouse_button,
            modifier,
            n_press,
            delta,
        }
    }
}
impl FromValue for MouseEvent {
    fn from_value(value: &Value) -> Self {
        Self::from_bytes(value.data(), Self::bytes_len())
    }
}

/////////////////////////////////////////////////////////////////////////////////////
/// [`FocusEvent`] Focus in/out events.
/////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct FocusEvent {
    type_: EventType,
}
impl_as_any!(FocusEvent);
impl FocusEvent {
    pub fn new(focus_in: bool) -> Self {
        Self {
            type_: if focus_in {
                EventType::FocusIn
            } else {
                EventType::FocusOut
            },
        }
    }
}
impl EventTrait for FocusEvent {
    fn type_(&self) -> EventType {
        self.type_
    }
}

impl StaticType for FocusEvent {
    fn static_type() -> Type {
        Type::from_name("FocusEvent")
    }

    fn bytes_len() -> usize {
        size_of::<bool>()
    }
}
impl ToBytes for FocusEvent {
    fn to_bytes(&self) -> Vec<u8> {
        match self.type_ {
            EventType::FocusIn => true.to_bytes(),
            EventType::FocusOut => false.to_bytes(),
            _ => unreachable!(),
        }
    }
}
impl ToValue for FocusEvent {
    fn to_value(&self) -> Value {
        Value::new(self)
    }

    fn value_type(&self) -> Type {
        Self::static_type()
    }
}
impl FromBytes for FocusEvent {
    fn from_bytes(data: &[u8], len: usize) -> Self {
        let focus_in = bool::from_bytes(data, len);
        Self::new(focus_in)
    }
}
impl FromValue for FocusEvent {
    fn from_value(value: &Value) -> Self {
        Self::from_bytes(value.data(), Self::bytes_len())
    }
}

/////////////////////////////////////////////////////////////////////////////////////
/// [`ResizeEvent`] Resize event.
/////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct ResizeEvent {
    type_: EventType,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_convert() {
        let evt: Event = Box::new(KeyEvent::new(
            EventType::KeyPress,
            KeyCode::KeyA,
            KeyCode::KeyA.to_string(),
            KeyboardModifier::AltModifier,
        ));
        let key_event = to_key_event(evt).unwrap();
        assert_eq!(key_event.type_, EventType::KeyPress);
        assert_eq!(key_event.key_code, KeyCode::KeyA);
        assert_eq!(key_event.text, KeyCode::KeyA.to_string());
        assert_eq!(key_event.modifier, KeyboardModifier::AltModifier);

        let evt: Event = Box::new(MouseEvent::new(
            EventType::MouseButtonPress,
            (234, 12),
            MouseButton::LeftButton,
            KeyboardModifier::ControlModifier.or(KeyboardModifier::ShiftModifier),
            3,
            100,
        ));
        let mouse_event = to_mouse_event(evt).unwrap();
        assert_eq!(mouse_event.type_, EventType::MouseButtonPress);
        assert_eq!(mouse_event.position, (234, 12));
        assert_eq!(mouse_event.mouse_button, MouseButton::LeftButton);
        assert_eq!(mouse_event.modifier, KeyboardModifier::ControlModifier.or(KeyboardModifier::ShiftModifier));
        assert_eq!(mouse_event.n_press, 3);
        assert_eq!(mouse_event.delta, 100);

        let evt: Event = Box::new(FocusEvent::new(true));
        let focus_event = to_focus_event(evt).unwrap();
        assert_eq!(focus_event.type_, EventType::FocusIn);
    }

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
            3,
            100,
        );
        let val = mouse_event.to_value();
        assert_eq!(mouse_event, val.get::<MouseEvent>())
    }

    #[test]
    fn test_focus_event_value() {
        let focus_event = FocusEvent::new(true);
        let val = focus_event.to_value();
        assert_eq!(focus_event, val.get::<FocusEvent>())
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
            3,
            0,
        );
        let focus_event = FocusEvent::new(false);
        let tuple = (key_event, mouse_event, focus_event);
        let val = tuple.to_value();
        assert_eq!(tuple, val.get::<(KeyEvent, MouseEvent, FocusEvent)>())
    }
}
