use crate::prelude::{KeyCode, KeyboardModifier};

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
/// Keyboard press/release events.
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