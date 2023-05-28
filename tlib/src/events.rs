use winit::event::Ime;
use crate::{
    figure::{Point, Size},
    impl_as_any, implements_enum_value,
    namespace::{AsNumeric, KeyCode, KeyboardModifier, MouseButton},
    prelude::*,
    values::{FromBytes, FromValue, ToBytes},
    Type, Value,
};
use std::{any::Any, fmt::Debug, mem::size_of, path::PathBuf};

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

#[inline]
pub fn to_resize_event(evt: Event) -> Result<Box<ResizeEvent>, Box<dyn Any>> {
    evt.as_any_boxed().downcast::<ResizeEvent>()
}

#[inline]
pub fn to_moved_event(evt: Event) -> Result<Box<MovedEvent>, Box<dyn Any>> {
    evt.as_any_boxed().downcast::<MovedEvent>()
}

#[inline]
pub fn to_file_event(evt: Event) -> Result<Box<FileEvent>, Box<dyn Any>> {
    evt.as_any_boxed().downcast::<FileEvent>()
}

#[inline]
pub fn to_receive_character_event(evt: Event) -> Result<Box<ReceiveCharacterEvent>, Box<dyn Any>> {
    evt.as_any_boxed().downcast::<ReceiveCharacterEvent>()
}

#[inline]
pub fn to_input_method_event(evt: Event) -> Result<Box<InputMethodEvent>, Box<dyn Any>> {
    evt.as_any_boxed().downcast::<InputMethodEvent>()
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
    Moved,
    DroppedFile,
    HoveredFile,
    HoveredFileCancelled,
    ReceivedCharacter,
    InputMethod,
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
            13 => Self::Moved,
            14 => Self::DroppedFile,
            15 => Self::HoveredFile,
            16 => Self::HoveredFileCancelled,
            17 => Self::ReceivedCharacter,
            18 => Self::InputMethod,
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
    #[inline]
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
    // x-horizontal, y-vertical
    delta: Point,
}
impl_as_any!(MouseEvent);
impl MouseEvent {
    pub fn new(
        type_: EventType,
        position: Position,
        mouse_button: MouseButton,
        modifier: KeyboardModifier,
        n_press: i32,
        delta: Point,
    ) -> Self {
        let type_ = match type_ {
            EventType::MouseButtonPress => type_,
            EventType::MouseButtonRelease => type_,
            EventType::MouseButtonDoubleClick => type_,
            EventType::MouseEnter => type_,
            EventType::MouseLeave => type_,
            EventType::MouseMove => type_,
            EventType::MouseWhell => type_,
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
    pub fn set_position(&mut self, pos: (i32, i32)) {
        self.position = pos
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
    pub fn delta(&self) -> Point {
        self.delta
    }
}

impl EventTrait for MouseEvent {
    #[inline]
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
            + i32::bytes_len() * 3
            + Point::bytes_len()
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

        let delta = Point::from_bytes(&data[idx..], i32_len);

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
    #[inline]
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
    size: Size,
}
impl_as_any!(ResizeEvent);
impl ResizeEvent {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            type_: EventType::Resize,
            size: (width, height).into(),
        }
    }

    #[inline]
    pub fn size(&self) -> Size {
        self.size
    }

    #[inline]
    pub fn width(&self) -> i32 {
        self.size.width()
    }

    #[inline]
    pub fn height(&self) -> i32 {
        self.size.height()
    }
}
impl EventTrait for ResizeEvent {
    #[inline]
    fn type_(&self) -> EventType {
        self.type_
    }
}

impl StaticType for ResizeEvent {
    fn static_type() -> Type {
        Type::from_name("ResizeEvent")
    }

    fn bytes_len() -> usize {
        EventType::bytes_len() + Size::bytes_len()
    }
}
impl ToBytes for ResizeEvent {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = self.type_.to_bytes();

        bytes.append(&mut self.size.to_bytes());

        bytes
    }
}
impl ToValue for ResizeEvent {
    fn to_value(&self) -> Value {
        Value::new(self)
    }

    fn value_type(&self) -> Type {
        Self::static_type()
    }
}
impl FromBytes for ResizeEvent {
    fn from_bytes(data: &[u8], _len: usize) -> Self {
        let mut idx = 0usize;
        let ty_len = EventType::bytes_len();
        let ty = EventType::from_bytes(&data[idx..idx + ty_len], ty_len);
        idx += ty_len;

        let size_len = Size::bytes_len();
        let size = Size::from_bytes(&data[idx..idx + size_len], size_len);

        Self { type_: ty, size }
    }
}
impl FromValue for ResizeEvent {
    fn from_value(value: &Value) -> Self {
        Self::from_bytes(value.data(), Self::bytes_len())
    }
}

/////////////////////////////////////////////////////////////////////////////////////
/// [`MovedEvent`] Indicate the window moved.
/////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct MovedEvent {
    type_: EventType,
    position: Point,
}
impl_as_any!(MovedEvent);
impl MovedEvent {
    #[inline]
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            type_: EventType::Moved,
            position: (x, y).into(),
        }
    }

    #[inline]
    pub fn position(&self) -> Point {
        self.position
    }

    #[inline]
    pub fn x(&self) -> i32 {
        self.position.x()
    }

    #[inline]
    pub fn y(&self) -> i32 {
        self.position.y()
    }
}
impl EventTrait for MovedEvent {
    #[inline]
    fn type_(&self) -> EventType {
        self.type_
    }
}

impl StaticType for MovedEvent {
    fn static_type() -> Type {
        Type::from_name("MovedEvent")
    }

    fn bytes_len() -> usize {
        EventType::bytes_len() + Point::bytes_len()
    }
}
impl ToBytes for MovedEvent {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = self.type_.to_bytes();

        bytes.append(&mut self.position.to_bytes());

        bytes
    }
}
impl ToValue for MovedEvent {
    fn to_value(&self) -> Value {
        Value::new(self)
    }

    fn value_type(&self) -> Type {
        Self::static_type()
    }
}
impl FromBytes for MovedEvent {
    fn from_bytes(data: &[u8], _len: usize) -> Self {
        let mut idx = 0usize;

        let ty_len = EventType::bytes_len();
        let type_ = EventType::from_bytes(&data[idx..idx + ty_len], ty_len);
        idx += ty_len;

        let pos_len = Point::bytes_len();
        let position = Point::from_bytes(&data[idx..idx + pos_len], pos_len);

        Self { type_, position }
    }
}
impl FromValue for MovedEvent {
    fn from_value(value: &Value) -> Self {
        Self::from_bytes(value.data(), Self::bytes_len())
    }
}

/////////////////////////////////////////////////////////////////////////////////////
/// [`FileEvent`] Indicate the window has dropped in([`DroppedFile`](EventType::DroppedFile))
/// or horvered([`HoveredFile`](EventType::HoveredFile))/hovered canceled([`HoveredFileCancelled`](EventType::HoveredFileCancelled)).
/////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct FileEvent {
    type_: EventType,
    path: Option<PathBuf>,
}
impl_as_any!(FileEvent);
impl FileEvent {
    #[inline]
    pub fn dropped(path: PathBuf) -> Self {
        Self {
            type_: EventType::DroppedFile,
            path: Some(path),
        }
    }

    #[inline]
    pub fn hovered(path: PathBuf) -> Self {
        Self {
            type_: EventType::HoveredFile,
            path: Some(path),
        }
    }

    #[inline]
    pub fn hovered_cancel() -> Self {
        Self {
            type_: EventType::HoveredFileCancelled,
            path: None,
        }
    }

    #[inline]
    pub fn path(&self) -> Option<&PathBuf> {
        self.path.as_ref()
    }
}
impl EventTrait for FileEvent {
    fn type_(&self) -> EventType {
        self.type_
    }
}

impl StaticType for FileEvent {
    fn static_type() -> Type {
        Type::from_name("FileEvent")
    }

    fn bytes_len() -> usize {
        EventType::bytes_len() + PathBuf::bytes_len()
    }

    fn dyn_bytes_len(&self) -> usize {
        let path_len = if self.path.is_none() {
            0
        } else {
            if self.path.is_none() {
                0
            } else {
                if let Some(str) = self.path().unwrap().as_os_str().to_str() {
                    // Path string will end with '\0':
                    str.len() + 1
                } else {
                    0
                }
            }
        };
        EventType::bytes_len() + path_len
    }
}
impl ToBytes for FileEvent {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = self.type_.to_bytes();

        if let Some(ref path) = self.path {
            bytes.append(&mut path.to_bytes());
        }

        bytes
    }
}
impl ToValue for FileEvent {
    fn to_value(&self) -> Value {
        Value::new(self)
    }

    fn value_type(&self) -> Type {
        Self::static_type()
    }
}
impl FromBytes for FileEvent {
    fn from_bytes(data: &[u8], _len: usize) -> Self {
        let mut idx = 0usize;

        let ty_len = EventType::bytes_len();
        let type_ = EventType::from_bytes(&data[idx..idx + ty_len], ty_len);
        let path = if data.len() == EventType::bytes_len() {
            None
        } else {
            idx += ty_len;
            Some(PathBuf::from_bytes(&data[idx..], 0))
        };

        Self { type_, path }
    }
}
impl FromValue for FileEvent {
    fn from_value(value: &Value) -> Self {
        Self::from_bytes(value.data(), Self::bytes_len())
    }
}

/////////////////////////////////////////////////////////////////////////////////////
/// [`ReceiveCharacterEvent`] window receive the unicode character event.
/////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct ReceiveCharacterEvent {
    type_: EventType,
    c: char,
}
impl_as_any!(ReceiveCharacterEvent);
impl ReceiveCharacterEvent {
    #[inline]
    pub fn new(c: char) -> Self {
        Self {
            type_: EventType::ReceivedCharacter,
            c,
        }
    }

    #[inline]
    pub fn character(&self) -> char {
        self.c
    }
}
impl EventTrait for ReceiveCharacterEvent {
    fn type_(&self) -> EventType {
        self.type_
    }
}

impl StaticType for ReceiveCharacterEvent {
    fn static_type() -> Type {
        Type::from_name("ReceiveCharacterEvent")
    }

    fn bytes_len() -> usize {
        EventType::bytes_len() + char::bytes_len()
    }
}
impl ToBytes for ReceiveCharacterEvent {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = self.type_.to_bytes();

        bytes.append(&mut self.c.to_bytes());

        bytes
    }
}
impl ToValue for ReceiveCharacterEvent {
    fn to_value(&self) -> Value {
        Value::new(self)
    }

    fn value_type(&self) -> Type {
        Self::static_type()
    }
}
impl FromBytes for ReceiveCharacterEvent {
    fn from_bytes(data: &[u8], _len: usize) -> Self {
        let mut idx = 0usize;

        let ty_len = EventType::bytes_len();
        let type_ = EventType::from_bytes(&data[idx..idx + ty_len], ty_len);
        idx += ty_len;

        let char_len = char::bytes_len();
        let c = char::from_bytes(&data[idx..], char_len);

        Self { type_, c }
    }
}
impl FromValue for ReceiveCharacterEvent {
    fn from_value(value: &Value) -> Self {
        Self::from_bytes(value.data(), Self::bytes_len())
    }
}

/////////////////////////////////////////////////////////////////////////////////////
/// [`InputMethodEvent`] window receive the input method event.
/////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct InputMethodEvent {
    type_: EventType,
    ime: Ime,
}
impl_as_any!(InputMethodEvent);
impl InputMethodEvent {
    #[inline]
    pub fn new(ime: Ime) -> Self {
        Self {
            type_: EventType::InputMethod,
            ime,
        }
    }

    #[inline]
    pub fn ime(&self) -> &Ime {
        &self.ime
    }
}
impl EventTrait for InputMethodEvent {
    #[inline]
    fn type_(&self) -> EventType {
        self.type_
    }
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
            Point::new(100, 0),
        ));
        let mouse_event = to_mouse_event(evt).unwrap();
        assert_eq!(mouse_event.type_, EventType::MouseButtonPress);
        assert_eq!(mouse_event.position, (234, 12));
        assert_eq!(mouse_event.mouse_button, MouseButton::LeftButton);
        assert_eq!(
            mouse_event.modifier,
            KeyboardModifier::ControlModifier.or(KeyboardModifier::ShiftModifier)
        );
        assert_eq!(mouse_event.n_press, 3);
        assert_eq!(mouse_event.delta, Point::new(100, 0));

        let evt: Event = Box::new(FocusEvent::new(true));
        let focus_event = to_focus_event(evt).unwrap();
        assert_eq!(focus_event.type_, EventType::FocusIn);

        let evt: Event = Box::new(ResizeEvent::new(225, 150));
        let resize_evt = to_resize_event(evt).unwrap();
        assert_eq!(resize_evt.type_, EventType::Resize);
        assert_eq!(resize_evt.width(), 225);
        assert_eq!(resize_evt.height(), 150);

        let evt: Event = Box::new(MovedEvent::new(290, 15));
        let moved_evt = to_moved_event(evt).unwrap();
        assert_eq!(moved_evt.type_, EventType::Moved);
        assert_eq!(moved_evt.x(), 290);
        assert_eq!(moved_evt.y(), 15);

        let path = "C:\\Windows\\System".into();
        let evt: Event = Box::new(FileEvent::dropped(path));
        let file_evt = to_file_event(evt).unwrap();
        let path: Option<PathBuf> = Some("C:\\Windows\\System".into());
        assert_eq!(file_evt.type_, EventType::DroppedFile);
        assert_eq!(file_evt.path, path);

        let evt: Event = Box::new(ReceiveCharacterEvent::new('好'));
        let receive_char_evt = to_receive_character_event(evt).unwrap();
        assert_eq!(receive_char_evt.type_, EventType::ReceivedCharacter);
        assert_eq!(receive_char_evt.c, '好');

        let ime = Ime::Preedit("a b".to_string(), Some((3, 3)));
        let evt: Event = Box::new(InputMethodEvent::new(ime.clone()));
        let ime_event = to_input_method_event(evt).unwrap();
        assert_eq!(ime_event.type_, EventType::InputMethod);
        assert_eq!(ime_event.ime, ime);
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
            Point::new(100, 0),
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
    fn test_resize_event_value() {
        let resize_evt = ResizeEvent::new(125, 225);
        let val = resize_evt.to_value();
        assert_eq!(resize_evt, val.get::<ResizeEvent>())
    }

    #[test]
    fn test_moved_event_value() {
        let moved_event = MovedEvent::new(290, 15);
        let val = moved_event.to_value();
        assert_eq!(moved_event, val.get::<MovedEvent>())
    }

    #[test]
    fn test_file_event_value() {
        let path = "C:\\Windows\\System".into();
        let file_event = FileEvent::dropped(path);
        let val = file_event.to_value();
        assert_eq!(file_event, val.get::<FileEvent>());

        let file_event = FileEvent::hovered_cancel();
        let val = file_event.to_value();
        assert_eq!(file_event, val.get::<FileEvent>());
    }

    #[test]
    fn test_receive_character_value() {
        let receive_char_evt = ReceiveCharacterEvent::new('好');
        let val = receive_char_evt.to_value();
        assert_eq!(receive_char_evt, val.get::<ReceiveCharacterEvent>())
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
            Point::new(100, 0),
        );
        let path = "C:\\Windows\\System".into();
        let file_event = FileEvent::dropped(path);
        let hover_cancel_evt = FileEvent::hovered_cancel();
        let focus_event = FocusEvent::new(false);
        let resize_evt = ResizeEvent::new(125, 225);
        let moved_event = MovedEvent::new(290, 15);
        let receive_char_evt = ReceiveCharacterEvent::new('好');
        let tuple = (
            key_event,
            mouse_event,
            file_event,
            hover_cancel_evt,
            focus_event,
            resize_evt,
            moved_event,
            receive_char_evt,
        );
        let val = tuple.to_value();
        assert_eq!(
            tuple,
            val.get::<(
                KeyEvent,
                MouseEvent,
                FileEvent,
                FileEvent,
                FocusEvent,
                ResizeEvent,
                MovedEvent,
                ReceiveCharacterEvent
            )>()
        )
    }
}
