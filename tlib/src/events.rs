use crate::{
    figure::{Point, Size},
    global::to_static,
    impl_as_any, implements_enum_value,
    namespace::{AsNumeric, KeyCode, KeyboardModifier, MouseButton},
    prelude::*,
    values::{FromBytes, FromValue, ToBytes},
    Type, Value, payload::PayloadWeight,
};
use std::{any::Any, fmt::Debug, mem::size_of, path::PathBuf};
use winit::event::Ime;

pub type Event = Box<dyn EventTrait>;
pub trait EventTrait: 'static + PayloadWeight + AsAny + Debug + Sync + Send {
    fn type_(&self) -> EventType;
}

#[inline]
pub fn downcast_event<T: EventTrait>(evt: Event) -> Result<Box<T>, Box<dyn Any>> {
    evt.as_any_boxed().downcast::<T>()
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum EventType {
    #[default]
    None = 0,
    MouseButtonPress,
    MouseButtonRelease,
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
            3 => Self::MouseMove,
            4 => Self::MouseWhell,
            5 => Self::MouseEnter,
            6 => Self::MouseLeave,
            7 => Self::KeyPress,
            8 => Self::KeyRelease,
            9 => Self::FocusIn,
            10 => Self::FocusOut,
            11 => Self::Resize,
            12 => Self::Moved,
            13 => Self::DroppedFile,
            14 => Self::HoveredFile,
            15 => Self::HoveredFileCancelled,
            16 => Self::ReceivedCharacter,
            17 => Self::InputMethod,
            _ => unimplemented!(),
        }
    }
}
impl AsNumeric<u8> for EventType {
    #[inline]
    fn as_numeric(&self) -> u8 {
        *self as u8
    }
}
implements_enum_value!(EventType, u8);

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum DeltaType {
    #[default]
    Pixel = 0,
    Line,
}
impl From<u8> for DeltaType {
    #[inline]
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Pixel,
            1 => Self::Line,
            _ => unimplemented!(),
        }
    }
}
impl AsNumeric<u8> for DeltaType {
    #[inline]
    fn as_numeric(&self) -> u8 {
        *self as u8
    }
}
implements_enum_value!(DeltaType, u8);

/////////////////////////////////////////////////////////////////////////////////////
/// [`KeyEvent`] Keyboard press/release events.
/////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct KeyEvent {
    type_: EventType,
    key_code: KeyCode,
    modifier: KeyboardModifier,
    text: &'static str,
}
impl_as_any!(KeyEvent);
impl KeyEvent {
    #[inline]
    pub fn new(
        type_: EventType,
        key_code: KeyCode,
        modifier: KeyboardModifier,
        text: &'static str,
    ) -> Self {
        let type_ = match type_ {
            EventType::KeyPress => type_,
            EventType::KeyRelease => type_,
            _ => unimplemented!(),
        };
        Self {
            type_,
            key_code,
            modifier,
            text,
        }
    }

    #[inline]
    pub fn key_code(&self) -> KeyCode {
        self.key_code
    }

    #[inline]
    pub fn name(&self) -> &str {
        self.key_code.name()
    }

    #[inline]
    pub fn text(&self) -> &str {
        self.text
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
impl PayloadWeight for KeyEvent {
    #[inline]
    fn payload_wieght(&self) -> f32 {
        1.
    }
}
impl StaticType for KeyEvent {
    #[inline]
    fn static_type() -> Type {
        Type::from_name("KeyEvent")
    }

    #[inline]
    fn bytes_len() -> usize {
        0
    }

    #[inline]
    fn dyn_bytes_len(&self) -> usize {
        EventType::bytes_len()
            + KeyCode::bytes_len()
            + KeyboardModifier::bytes_len()
            + self.text.as_bytes().len()
            + 1
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

        let mut text = self.text.to_bytes();
        bytes.append(&mut text);

        bytes
    }
}
impl ToValue for KeyEvent {
    #[inline]
    fn to_value(&self) -> Value {
        Value::new(self)
    }

    #[inline]
    fn value_type(&self) -> Type {
        Self::static_type()
    }
}
impl FromBytes for KeyEvent {
    fn from_bytes(data: &[u8], _: usize) -> Self {
        let type_len = EventType::bytes_len();
        let code_len = KeyCode::bytes_len();
        let modifier_len = KeyboardModifier::bytes_len();
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
        let text = to_static(String::from_bytes(text_bytes, 0));

        Self {
            type_,
            key_code,
            modifier,
            text,
        }
    }
}
impl FromValue for KeyEvent {
    #[inline]
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
    delta_type: DeltaType,
}
impl_as_any!(MouseEvent);
impl MouseEvent {
    #[inline]
    pub fn new(
        type_: EventType,
        position: Position,
        mouse_button: MouseButton,
        modifier: KeyboardModifier,
        n_press: i32,
        delta: Point,
        delta_type: DeltaType,
    ) -> Self {
        let type_ = match type_ {
            EventType::MouseButtonPress => type_,
            EventType::MouseButtonRelease => type_,
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
            delta_type,
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

    #[inline]
    pub fn delta_type(&self) -> DeltaType {
        self.delta_type
    }
}

impl EventTrait for MouseEvent {
    #[inline]
    fn type_(&self) -> EventType {
        self.type_
    }
}

impl PayloadWeight for MouseEvent {
    #[inline]
    fn payload_wieght(&self) -> f32 {
        match self.type_ {
            EventType::MouseMove => 0.2,
            _ => 1.,
        }
    }
}

impl StaticType for MouseEvent {
    #[inline]
    fn static_type() -> Type {
        Type::from_name("MouseEvent")
    }

    #[inline]
    fn bytes_len() -> usize {
        EventType::bytes_len()
            + MouseButton::bytes_len()
            + KeyboardModifier::bytes_len()
            + i32::bytes_len() * 3
            + Point::bytes_len()
            + DeltaType::bytes_len()
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

        bytes.append(&mut self.delta_type.to_bytes());

        bytes
    }
}
impl ToValue for MouseEvent {
    #[inline]
    fn to_value(&self) -> Value {
        Value::new(self)
    }

    #[inline]
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

        let point_len = Point::bytes_len();
        let delta = Point::from_bytes(&data[idx..], point_len);
        idx += point_len;

        let delta_type = DeltaType::from_bytes(&data[idx..], DeltaType::bytes_len());

        Self {
            type_,
            position: (x, y),
            mouse_button,
            modifier,
            n_press,
            delta,
            delta_type,
        }
    }
}
impl FromValue for MouseEvent {
    #[inline]
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
    #[inline]
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

impl PayloadWeight for FocusEvent {
    #[inline]
    fn payload_wieght(&self) -> f32 {
        1.
    }
}

impl StaticType for FocusEvent {
    #[inline]
    fn static_type() -> Type {
        Type::from_name("FocusEvent")
    }

    #[inline]
    fn bytes_len() -> usize {
        size_of::<bool>()
    }
}
impl ToBytes for FocusEvent {
    #[inline]
    fn to_bytes(&self) -> Vec<u8> {
        match self.type_ {
            EventType::FocusIn => true.to_bytes(),
            EventType::FocusOut => false.to_bytes(),
            _ => unreachable!(),
        }
    }
}
impl ToValue for FocusEvent {
    #[inline]
    fn to_value(&self) -> Value {
        Value::new(self)
    }

    #[inline]
    fn value_type(&self) -> Type {
        Self::static_type()
    }
}
impl FromBytes for FocusEvent {
    #[inline]
    fn from_bytes(data: &[u8], len: usize) -> Self {
        let focus_in = bool::from_bytes(data, len);
        Self::new(focus_in)
    }
}
impl FromValue for FocusEvent {
    #[inline]
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
    #[inline]
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

impl PayloadWeight for ResizeEvent {
    #[inline]
    fn payload_wieght(&self) -> f32 {
        10.
    }
}

impl StaticType for ResizeEvent {
    #[inline]
    fn static_type() -> Type {
        Type::from_name("ResizeEvent")
    }

    #[inline]
    fn bytes_len() -> usize {
        EventType::bytes_len() + Size::bytes_len()
    }
}
impl ToBytes for ResizeEvent {
    #[inline]
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = self.type_.to_bytes();

        bytes.append(&mut self.size.to_bytes());

        bytes
    }
}
impl ToValue for ResizeEvent {
    #[inline]
    fn to_value(&self) -> Value {
        Value::new(self)
    }

    #[inline]
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
    #[inline]
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

impl PayloadWeight for MovedEvent {
    #[inline]
    fn payload_wieght(&self) -> f32 {
        0.1
    }
}

impl StaticType for MovedEvent {
    #[inline]
    fn static_type() -> Type {
        Type::from_name("MovedEvent")
    }

    #[inline]
    fn bytes_len() -> usize {
        EventType::bytes_len() + Point::bytes_len()
    }
}
impl ToBytes for MovedEvent {
    #[inline]
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = self.type_.to_bytes();

        bytes.append(&mut self.position.to_bytes());

        bytes
    }
}
impl ToValue for MovedEvent {
    #[inline]
    fn to_value(&self) -> Value {
        Value::new(self)
    }

    #[inline]
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
    #[inline]
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
    #[inline]
    fn type_(&self) -> EventType {
        self.type_
    }
}

impl PayloadWeight for FileEvent {
    #[inline]
    fn payload_wieght(&self) -> f32 {
        1.
    }
}

impl StaticType for FileEvent {
    #[inline]
    fn static_type() -> Type {
        Type::from_name("FileEvent")
    }

    #[inline]
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
    #[inline]
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = self.type_.to_bytes();

        if let Some(ref path) = self.path {
            bytes.append(&mut path.to_bytes());
        }

        bytes
    }
}
impl ToValue for FileEvent {
    #[inline]
    fn to_value(&self) -> Value {
        Value::new(self)
    }

    #[inline]
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
    #[inline]
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
    #[inline]
    fn type_(&self) -> EventType {
        self.type_
    }
}

impl PayloadWeight for ReceiveCharacterEvent {
    #[inline]
    fn payload_wieght(&self) -> f32 {
        1.
    }
}

impl StaticType for ReceiveCharacterEvent {
    #[inline]
    fn static_type() -> Type {
        Type::from_name("ReceiveCharacterEvent")
    }

    #[inline]
    fn bytes_len() -> usize {
        EventType::bytes_len() + char::bytes_len()
    }
}
impl ToBytes for ReceiveCharacterEvent {
    #[inline]
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = self.type_.to_bytes();

        bytes.append(&mut self.c.to_bytes());

        bytes
    }
}
impl ToValue for ReceiveCharacterEvent {
    #[inline]
    fn to_value(&self) -> Value {
        Value::new(self)
    }

    #[inline]
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
    #[inline]
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

impl PayloadWeight for InputMethodEvent {
    #[inline]
    fn payload_wieght(&self) -> f32 {
        1.
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
            KeyboardModifier::AltModifier,
            "a",
        ));
        let key_event = downcast_event::<KeyEvent>(evt).unwrap();
        assert_eq!(key_event.type_, EventType::KeyPress);
        assert_eq!(key_event.key_code, KeyCode::KeyA);
        assert_eq!(key_event.name(), KeyCode::KeyA.name());
        assert_eq!(key_event.text(), "a");
        assert_eq!(key_event.modifier, KeyboardModifier::AltModifier);

        let evt: Event = Box::new(MouseEvent::new(
            EventType::MouseButtonPress,
            (234, 12),
            MouseButton::LeftButton,
            KeyboardModifier::ControlModifier.or(KeyboardModifier::ShiftModifier),
            3,
            Point::new(100, 0),
            DeltaType::Line,
        ));
        let mouse_event = downcast_event::<MouseEvent>(evt).unwrap();
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
        let focus_event = downcast_event::<FocusEvent>(evt).unwrap();
        assert_eq!(focus_event.type_, EventType::FocusIn);

        let evt: Event = Box::new(ResizeEvent::new(225, 150));
        let resize_evt = downcast_event::<ResizeEvent>(evt).unwrap();
        assert_eq!(resize_evt.type_, EventType::Resize);
        assert_eq!(resize_evt.width(), 225);
        assert_eq!(resize_evt.height(), 150);

        let evt: Event = Box::new(MovedEvent::new(290, 15));
        let moved_evt = downcast_event::<MovedEvent>(evt).unwrap();
        assert_eq!(moved_evt.type_, EventType::Moved);
        assert_eq!(moved_evt.x(), 290);
        assert_eq!(moved_evt.y(), 15);

        let path = "C:\\Windows\\System".into();
        let evt: Event = Box::new(FileEvent::dropped(path));
        let file_evt = downcast_event::<FileEvent>(evt).unwrap();
        let path: Option<PathBuf> = Some("C:\\Windows\\System".into());
        assert_eq!(file_evt.type_, EventType::DroppedFile);
        assert_eq!(file_evt.path, path);

        let evt: Event = Box::new(ReceiveCharacterEvent::new('好'));
        let receive_char_evt = downcast_event::<ReceiveCharacterEvent>(evt).unwrap();
        assert_eq!(receive_char_evt.type_, EventType::ReceivedCharacter);
        assert_eq!(receive_char_evt.c, '好');

        let ime = Ime::Preedit("a b".to_string(), Some((3, 3)));
        let evt: Event = Box::new(InputMethodEvent::new(ime.clone()));
        let ime_event = downcast_event::<InputMethodEvent>(evt).unwrap();
        assert_eq!(ime_event.type_, EventType::InputMethod);
        assert_eq!(ime_event.ime, ime);
    }

    #[test]
    fn test_key_event_value() {
        let key_event = KeyEvent::new(
            EventType::KeyPress,
            KeyCode::KeyA,
            KeyboardModifier::AltModifier,
            "a",
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
            DeltaType::default(),
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
            KeyboardModifier::AltModifier,
            "a",
        );
        let mouse_event = MouseEvent::new(
            EventType::MouseButtonPress,
            (234, 12),
            MouseButton::LeftButton,
            KeyboardModifier::ControlModifier.or(KeyboardModifier::ShiftModifier),
            3,
            Point::new(100, 0),
            DeltaType::default(),
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
