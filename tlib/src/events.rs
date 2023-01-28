use crate::namespace::{KeyCode, KeyboardModifier, MouseButton};

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
    pub fn new(type_: EventType, position: Position, mouse_button: MouseButton, modifier: KeyboardModifier) -> Self {
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
