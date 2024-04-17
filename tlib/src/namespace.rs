use crate::{
    prelude::{StaticType, ToValue}, typedef::{SkiaBlendMode, WinitKeyCode, WinitMouseButton}, values::{FromBytes, FromValue, ToBytes}, Type, Value
};
use std::mem::size_of;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use winit::window::CursorIcon;

/// Use macro rules to implement [`StaticType`], [`ToBytes`], [`ToValue`], [`FromBytes`], [`FromValue`] automatically. <br>
/// The enum use this macro should implements [`AsNumeric`], [`From<T>`].
/// ### Example:
/// ```
/// use tlib::{
///     prelude::{StaticType, ToValue},
///     values::{FromBytes, FromValue, ToBytes},
///     Type, Value,
/// };
/// use tlib::implements_enum_value;
/// use tlib::namespace::AsNumeric;
/// use std::mem::size_of;
///
/// #[repr(u8)]
/// #[derive(Clone, Copy)]
/// pub enum Enum {
///     One = 0,
///     Two,
/// }
/// impl AsNumeric<u8> for Enum {
///     fn as_numeric(&self) -> u8 {
///         *self as u8
///     }
/// }
/// impl From<u8> for Enum {
///     fn from(value: u8) -> Self {
///         match value {
///             0 => Self::One,
///             1 => Self::Two,
///             _ => unimplemented!(),
///         }
///     }
/// }
/// implements_enum_value!(Enum, u8);
/// ```
#[macro_export]
macro_rules! implements_enum_value {
    ($name:ident, $repr:ident) => {
        impl StaticType for $name {
            fn static_type() -> Type {
                Type::from_name(stringify!($name))
            }

            fn bytes_len() -> usize {
                size_of::<$repr>()
            }
        }

        impl ToBytes for $name {
            fn to_bytes(&self) -> Vec<u8> {
                self.as_numeric().to_bytes()
            }
        }

        impl ToValue for $name {
            fn to_value(&self) -> Value {
                Value::new(self)
            }

            fn value_type(&self) -> Type {
                Self::static_type()
            }
        }

        impl FromBytes for $name {
            fn from_bytes(data: &[u8], len: usize) -> Self {
                Self::from($repr::from_bytes(data, len))
            }
        }

        impl FromValue for $name {
            fn from_value(value: &Value) -> Self {
                Self::from_bytes(value.data(), Self::bytes_len())
            }
        }
    };
}

pub trait AsNumeric<T: ToBytes> {
    fn as_numeric(&self) -> T;
}

////////////////////////////////////////////////////////////////////////////////////////////////
/// [`KeyCode`]
////////////////////////////////////////////////////////////////////////////////////////////////
/// The enum to represent the key code on keyboard.
#[repr(u32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, EnumIter, Default)]
pub enum KeyCode {
    #[default]
    Unknown = 0x00,

    // Unicode Basic Latin block
    KeySpace,
    KeyExclam,
    KeyQuoteDbl,
    KeyNumberSign,
    KeyDollar,
    KeyPercent,
    KeyAmpersand,
    KeyApostrophe,
    KeyParenLeft,
    KeyParenRight,
    KeyAsterisk,
    KeyPlus,
    KeyComma,
    KeyMinus,
    KeyPeriod,
    KeySlash,

    // The number keys over the letters.
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,

    KeyColon,
    KeySemicolon,
    KeyLess,
    KeyEqual,
    KeyGreater,
    KeyQuestion,
    KeyAt,
    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
    KeyG,
    KeyH,
    KeyI,
    KeyJ,
    KeyK,
    KeyL,
    KeyM,
    KeyN,
    KeyO,
    KeyP,
    KeyQ,
    KeyR,
    KeyS,
    KeyT,
    KeyU,
    KeyV,
    KeyW,
    KeyX,
    KeyY,
    KeyZ,
    KeyBracketLeft,
    KeyBackslash,
    KeyBracketRight,
    KeyCaret,
    KeyUnderscore,
    KeyQuoteLeft,
    KeyBraceLeft,
    KeyBar,
    KeyBraceRight,
    KeyTilde,
    KeyCompose,

    KeyEscape, // misc keys
    KeyTab,
    KeyBacktab,
    KeyBackspace,
    KeyReturn,
    KeyEnter,
    KeyInsert,
    KeyDelete,
    KeyPause,
    KeyPrint, // print screen
    KeySysReq,
    KeyClear,
    KeyHome, // cursor movement
    KeyEnd,
    KeyLeft,
    KeyUp,
    KeyRight,
    KeyDown,
    KeyPageUp,
    KeyPageDown,
    KeyShift, // modifiers
    KeyControl,
    KeyMeta,
    KeyAlt,
    KeyCapsLock,
    KeyScrollLock,

    // Numpad
    KeyNumLock,
    KeyNumpad0,
    KeyNumpad1,
    KeyNumpad2,
    KeyNumpad3,
    KeyNumpad4,
    KeyNumpad5,
    KeyNumpad6,
    KeyNumpad7,
    KeyNumpad8,
    KeyNumpad9,
    KeyNumpadAdd,
    KeyNumpadDivide,
    KeyNumpadDecimal,
    KeyNumpadComma,
    KeyNumpadEnter,
    KeyNumpadEquals,
    KeyNumpadMultiply,
    KeyNumpadSubtract,

    KeyF1, // function keys
    KeyF2,
    KeyF3,
    KeyF4,
    KeyF5,
    KeyF6,
    KeyF7,
    KeyF8,
    KeyF9,
    KeyF10,
    KeyF11,
    KeyF12,
    KeyF13,
    KeyF14,
    KeyF15,
    KeyF16,
    KeyF17,
    KeyF18,
    KeyF19,
    KeyF20,
    KeyF21,
    KeyF22,
    KeyF23,
    KeyF24,

    KeySuperL, // extra keys
    KeySuperR,
    KeyMenu,
    KeyHyperL,
    KeyHyperR,
    KeyHelp,
    KeyDirectionL,
    KeyDirectionR,
    KeyAbntC1,
    KeyAbntC2,
    KeyCalculator,
    KeyKana,
    KeyKanji,
    KeyMail,
    KeyMediaSelect,
    KeyMediaStop,
    KeyMute,
    KeyUnderLine,
    KeyVolumeDown,
    KeyVolumeUp,
}
impl AsNumeric<u32> for KeyCode {
    fn as_numeric(&self) -> u32 {
        *self as u32
    }
}
impl From<u32> for KeyCode {
    fn from(value: u32) -> Self {
        for code in Self::iter() {
            if code as u32 == value {
                return code;
            }
        }
        Self::Unknown
    }
}
impl From<String> for KeyCode {
    fn from(value: String) -> Self {
        for code in Self::iter() {
            if code.name() == value {
                return code;
            }
        }
        Self::Unknown
    }
}
impl From<&str> for KeyCode {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}
impl From<&String> for KeyCode {
    fn from(value: &String) -> Self {
        Self::from(value.to_string())
    }
}
impl From<KeyCode> for u32 {
    #[inline]
    fn from(value: KeyCode) -> Self {
        value as u32
    }
}
impl KeyCode {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Unknown => "Unknown",
            Self::KeySpace => "Space",
            Self::KeyExclam => "!",
            Self::KeyQuoteDbl => "@",
            Self::KeyNumberSign => "#",
            Self::KeyDollar => "$",
            Self::KeyPercent => "%",
            Self::KeyAmpersand => "&",
            Self::KeyApostrophe => "'",
            Self::KeyParenLeft => "(",
            Self::KeyParenRight => ")",
            Self::KeyAsterisk => "*",
            Self::KeyPlus => "+",
            Self::KeyComma => ",",
            Self::KeyMinus => "-",
            Self::KeyPeriod => ".",
            Self::KeySlash => "/",
            Self::Key0 => "0",
            Self::Key1 => "1",
            Self::Key2 => "2",
            Self::Key3 => "3",
            Self::Key4 => "4",
            Self::Key5 => "5",
            Self::Key6 => "6",
            Self::Key7 => "7",
            Self::Key8 => "8",
            Self::Key9 => "9",
            Self::KeyColon => ":",
            Self::KeySemicolon => ";",
            Self::KeyLess => "<",
            Self::KeyEqual => "=",
            Self::KeyGreater => ">",
            Self::KeyQuestion => "?",
            Self::KeyAt => "@",
            Self::KeyA => "A",
            Self::KeyB => "B",
            Self::KeyC => "C",
            Self::KeyD => "D",
            Self::KeyE => "E",
            Self::KeyF => "F",
            Self::KeyG => "G",
            Self::KeyH => "H",
            Self::KeyI => "I",
            Self::KeyJ => "J",
            Self::KeyK => "K",
            Self::KeyL => "L",
            Self::KeyM => "M",
            Self::KeyN => "N",
            Self::KeyO => "O",
            Self::KeyP => "P",
            Self::KeyQ => "Q",
            Self::KeyR => "R",
            Self::KeyS => "S",
            Self::KeyT => "T",
            Self::KeyU => "U",
            Self::KeyV => "V",
            Self::KeyW => "W",
            Self::KeyX => "X",
            Self::KeyY => "Y",
            Self::KeyZ => "Z",
            Self::KeyBracketLeft => "[",
            Self::KeyBackslash => "\\",
            Self::KeyBracketRight => "]",
            Self::KeyCaret => "^",
            Self::KeyUnderscore => "_",
            Self::KeyQuoteLeft => "\"",
            Self::KeyBraceLeft => "{",
            Self::KeyBar => "|",
            Self::KeyBraceRight => "}",
            Self::KeyTilde => "~",
            Self::KeyEscape => "Escape",
            Self::KeyTab => "Tab",
            Self::KeyBacktab => "Backtab",
            Self::KeyBackspace => "Backspace",
            Self::KeyReturn => "Return",
            Self::KeyEnter => "Enter",
            Self::KeyInsert => "Insert",
            Self::KeyDelete => "Delete",
            Self::KeyPause => "Pause",
            Self::KeyPrint => "Print",
            Self::KeySysReq => "SysReq",
            Self::KeyClear => "Clear",
            Self::KeyHome => "Home",
            Self::KeyEnd => "End",
            Self::KeyLeft => "Left",
            Self::KeyUp => "Up",
            Self::KeyRight => "Right",
            Self::KeyDown => "Down",
            Self::KeyPageUp => "PageUp",
            Self::KeyPageDown => "PageDown",
            Self::KeyShift => "Shift",
            Self::KeyControl => "Control",
            Self::KeyMeta => "Meta",
            Self::KeyAlt => "Alt",
            Self::KeyCapsLock => "CapsLock",
            Self::KeyScrollLock => "ScrollLock",
            Self::KeyNumLock => "NumLock",
            Self::KeyNumpad0 => "0",
            Self::KeyNumpad1 => "1",
            Self::KeyNumpad2 => "2",
            Self::KeyNumpad3 => "3",
            Self::KeyNumpad4 => "4",
            Self::KeyNumpad5 => "5",
            Self::KeyNumpad6 => "6",
            Self::KeyNumpad7 => "7",
            Self::KeyNumpad8 => "8",
            Self::KeyNumpad9 => "9",
            Self::KeyNumpadAdd => "+",
            Self::KeyNumpadDivide => "/",
            Self::KeyNumpadDecimal => ".",
            Self::KeyNumpadComma => ",",
            Self::KeyNumpadEnter => "Enter",
            Self::KeyNumpadEquals => "=",
            Self::KeyNumpadMultiply => "*",
            Self::KeyNumpadSubtract => "-",
            Self::KeyF1 => "F1",
            Self::KeyF2 => "F2",
            Self::KeyF3 => "F3",
            Self::KeyF4 => "F4",
            Self::KeyF5 => "F5",
            Self::KeyF6 => "F6",
            Self::KeyF7 => "F7",
            Self::KeyF8 => "F8",
            Self::KeyF9 => "F9",
            Self::KeyF10 => "F10",
            Self::KeyF11 => "F11",
            Self::KeyF12 => "F12",
            Self::KeyF13 => "F13",
            Self::KeyF14 => "F14",
            Self::KeyF15 => "F15",
            Self::KeyF16 => "F16",
            Self::KeyF17 => "F17",
            Self::KeyF18 => "F18",
            Self::KeyF19 => "F19",
            Self::KeyF20 => "F20",
            Self::KeyF21 => "F21",
            Self::KeyF22 => "F22",
            Self::KeyF23 => "F23",
            Self::KeyF24 => "F24",
            Self::KeySuperL => "SuperL",
            Self::KeySuperR => "SuperR",
            Self::KeyMenu => "Menu",
            Self::KeyHyperL => "HyperL",
            Self::KeyHyperR => "HyperR",
            Self::KeyHelp => "Help",
            Self::KeyDirectionL => "DirectionL",
            Self::KeyDirectionR => "DirectionR",
            Self::KeyCompose => "Compose",
            Self::KeyAbntC1 => "AbntC1",
            Self::KeyAbntC2 => "AbntC2",
            Self::KeyCalculator => "Calculator",
            Self::KeyKana => "Kana",
            Self::KeyKanji => "Kanji",
            Self::KeyMail => "Mail",
            Self::KeyMediaSelect => "MediaSel",
            Self::KeyMediaStop => "MediaStop",
            Self::KeyMute => "Mute",
            Self::KeyUnderLine => "_",
            Self::KeyVolumeDown => "VolumeDown",
            Self::KeyVolumeUp => "VolumeUp",
        }
    }
}
impl From<WinitKeyCode> for KeyCode {
    fn from(val: WinitKeyCode) -> Self {
        match val {
            WinitKeyCode::Digit1 => KeyCode::Key1,
            WinitKeyCode::Digit2 => KeyCode::Key2,
            WinitKeyCode::Digit3 => KeyCode::Key3,
            WinitKeyCode::Digit4 => KeyCode::Key4,
            WinitKeyCode::Digit5 => KeyCode::Key5,
            WinitKeyCode::Digit6 => KeyCode::Key6,
            WinitKeyCode::Digit7 => KeyCode::Key7,
            WinitKeyCode::Digit8 => KeyCode::Key8,
            WinitKeyCode::Digit9 => KeyCode::Key9,
            WinitKeyCode::Digit0 => KeyCode::Key0,
            WinitKeyCode::KeyA => KeyCode::KeyA,
            WinitKeyCode::KeyB => KeyCode::KeyB,
            WinitKeyCode::KeyC => KeyCode::KeyC,
            WinitKeyCode::KeyD => KeyCode::KeyD,
            WinitKeyCode::KeyE => KeyCode::KeyE,
            WinitKeyCode::KeyF => KeyCode::KeyF,
            WinitKeyCode::KeyG => KeyCode::KeyG,
            WinitKeyCode::KeyH => KeyCode::KeyH,
            WinitKeyCode::KeyI => KeyCode::KeyI,
            WinitKeyCode::KeyJ => KeyCode::KeyJ,
            WinitKeyCode::KeyK => KeyCode::KeyK,
            WinitKeyCode::KeyL => KeyCode::KeyL,
            WinitKeyCode::KeyM => KeyCode::KeyM,
            WinitKeyCode::KeyN => KeyCode::KeyN,
            WinitKeyCode::KeyO => KeyCode::KeyO,
            WinitKeyCode::KeyP => KeyCode::KeyP,
            WinitKeyCode::KeyQ => KeyCode::KeyQ,
            WinitKeyCode::KeyR => KeyCode::KeyR,
            WinitKeyCode::KeyS => KeyCode::KeyS,
            WinitKeyCode::KeyT => KeyCode::KeyT,
            WinitKeyCode::KeyU => KeyCode::KeyU,
            WinitKeyCode::KeyV => KeyCode::KeyV,
            WinitKeyCode::KeyW => KeyCode::KeyW,
            WinitKeyCode::KeyX => KeyCode::KeyX,
            WinitKeyCode::KeyY => KeyCode::KeyY,
            WinitKeyCode::KeyZ => KeyCode::KeyZ,
            WinitKeyCode::Escape => KeyCode::KeyEscape,
            WinitKeyCode::F1 => KeyCode::KeyF1,
            WinitKeyCode::F2 => KeyCode::KeyF2,
            WinitKeyCode::F3 => KeyCode::KeyF3,
            WinitKeyCode::F4 => KeyCode::KeyF4,
            WinitKeyCode::F5 => KeyCode::KeyF5,
            WinitKeyCode::F6 => KeyCode::KeyF6,
            WinitKeyCode::F7 => KeyCode::KeyF7,
            WinitKeyCode::F8 => KeyCode::KeyF8,
            WinitKeyCode::F9 => KeyCode::KeyF9,
            WinitKeyCode::F10 => KeyCode::KeyF10,
            WinitKeyCode::F11 => KeyCode::KeyF11,
            WinitKeyCode::F12 => KeyCode::KeyF12,
            WinitKeyCode::F13 => KeyCode::KeyF13,
            WinitKeyCode::F14 => KeyCode::KeyF14,
            WinitKeyCode::F15 => KeyCode::KeyF15,
            WinitKeyCode::F16 => KeyCode::KeyF16,
            WinitKeyCode::F17 => KeyCode::KeyF17,
            WinitKeyCode::F18 => KeyCode::KeyF18,
            WinitKeyCode::F19 => KeyCode::KeyF19,
            WinitKeyCode::F20 => KeyCode::KeyF20,
            WinitKeyCode::F21 => KeyCode::KeyF21,
            WinitKeyCode::F22 => KeyCode::KeyF22,
            WinitKeyCode::F23 => KeyCode::KeyF23,
            WinitKeyCode::F24 => KeyCode::KeyF24,
            WinitKeyCode::PrintScreen => KeyCode::KeyPrint,
            WinitKeyCode::ScrollLock => KeyCode::KeyScrollLock,
            WinitKeyCode::Pause => KeyCode::KeyPause,
            WinitKeyCode::Insert => KeyCode::KeyInsert,
            WinitKeyCode::Home => KeyCode::KeyHome,
            WinitKeyCode::Delete => KeyCode::KeyDelete,
            WinitKeyCode::Backspace => KeyCode::KeyBackspace,
            WinitKeyCode::End => KeyCode::KeyEnd,
            WinitKeyCode::PageDown => KeyCode::KeyPageDown,
            WinitKeyCode::PageUp => KeyCode::KeyPageUp,
            WinitKeyCode::ArrowLeft => KeyCode::KeyLeft,
            WinitKeyCode::ArrowUp => KeyCode::KeyUp,
            WinitKeyCode::ArrowRight => KeyCode::KeyRight,
            WinitKeyCode::ArrowDown => KeyCode::KeyDown,
            WinitKeyCode::Enter => KeyCode::KeyEnter,
            WinitKeyCode::Space => KeyCode::KeySpace,
            WinitKeyCode::NumLock => KeyCode::KeyNumLock,
            WinitKeyCode::Numpad0 => KeyCode::KeyNumpad0,
            WinitKeyCode::Numpad1 => KeyCode::KeyNumpad1,
            WinitKeyCode::Numpad2 => KeyCode::KeyNumpad2,
            WinitKeyCode::Numpad3 => KeyCode::KeyNumpad3,
            WinitKeyCode::Numpad4 => KeyCode::KeyNumpad4,
            WinitKeyCode::Numpad5 => KeyCode::KeyNumpad5,
            WinitKeyCode::Numpad6 => KeyCode::KeyNumpad6,
            WinitKeyCode::Numpad7 => KeyCode::KeyNumpad7,
            WinitKeyCode::Numpad8 => KeyCode::KeyNumpad8,
            WinitKeyCode::Numpad9 => KeyCode::KeyNumpad9,
            WinitKeyCode::NumpadAdd => KeyCode::KeyNumpadAdd,
            WinitKeyCode::NumpadDivide => KeyCode::KeyNumpadDivide,
            WinitKeyCode::NumpadDecimal => KeyCode::KeyNumpadDecimal,
            WinitKeyCode::NumpadComma => KeyCode::KeyNumpadComma,
            WinitKeyCode::NumpadEnter => KeyCode::KeyNumpadEnter,
            WinitKeyCode::NumpadEqual => KeyCode::KeyNumpadEquals,
            WinitKeyCode::NumpadMultiply => KeyCode::KeyNumpadMultiply,
            WinitKeyCode::NumpadSubtract => KeyCode::KeyNumpadSubtract,
            WinitKeyCode::Backslash => KeyCode::KeyBackslash,
            WinitKeyCode::Comma => KeyCode::KeyComma,
            WinitKeyCode::Equal => KeyCode::KeyEqual,
            WinitKeyCode::AltLeft => KeyCode::KeyAlt,
            WinitKeyCode::BracketLeft => KeyCode::KeyBracketLeft,
            WinitKeyCode::ControlLeft => KeyCode::KeyControl,
            WinitKeyCode::ShiftLeft => KeyCode::KeyShift,
            WinitKeyCode::Meta => KeyCode::KeyMeta,
            WinitKeyCode::MediaSelect => KeyCode::KeyMediaSelect,
            WinitKeyCode::MediaStop => KeyCode::KeyMediaStop,
            WinitKeyCode::Minus => KeyCode::KeyMinus,
            WinitKeyCode::Period => KeyCode::KeyPeriod,
            WinitKeyCode::AltRight => KeyCode::KeyAlt,
            WinitKeyCode::BracketRight => KeyCode::KeyBracketRight,
            WinitKeyCode::ControlRight => KeyCode::KeyControl,
            WinitKeyCode::ShiftRight => KeyCode::KeyShift,
            WinitKeyCode::Semicolon => KeyCode::KeySemicolon,
            WinitKeyCode::Slash => KeyCode::KeySlash,
            WinitKeyCode::Tab => KeyCode::KeyTab,
            _ => KeyCode::Unknown,
        }
    }
}
impl PartialOrd for KeyCode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for KeyCode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_numeric().cmp(&other.as_numeric())
    }
}
implements_enum_value!(KeyCode, u32);

////////////////////////////////////////////////////////////////////////////////////////////////
/// [`KeyboardModifier`]
////////////////////////////////////////////////////////////////////////////////////////////////
/// The enum to represent the keyboard modifier.
#[repr(u32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, EnumIter)]
pub enum KeyboardModifier {
    #[default]
    NoModifier,
    ShiftModifier,
    ControlModifier,
    AltModifier,
    MetaModifier,
    KeypadModifier,

    // x11 only.
    GroupSwitchModifier,

    KeyboardModifierMask,
    Combination(u32),
}
impl AsNumeric<u32> for KeyboardModifier {
    #[inline]
    fn as_numeric(&self) -> u32 {
        self.as_u32()
    }
}
impl KeyboardModifier {
    #[inline]
    pub fn or(&self, other: KeyboardModifier) -> KeyboardModifier {
        let one = self.as_u32();
        let other = other.as_u32();
        let after = one | other;
        if one == 0 {
            for m in Self::iter() {
                if m.as_u32() == after {
                    return m;
                }
            }
        }
        Self::Combination(after)
    }

    pub fn remove(&self, remove: KeyboardModifier) -> KeyboardModifier {
        match self {
            Self::Combination(data) => {
                let other = remove.as_u32();
                let after_remove = *data & !(*data & other);

                for b in Self::iter() {
                    if after_remove == b.as_u32() {
                        return b;
                    }
                }
                Self::Combination(after_remove)
            }
            _ => {
                if *self == remove {
                    Self::NoModifier
                } else {
                    *self
                }
            }
        }
    }

    #[inline]
    pub fn and(&self, other: KeyboardModifier) -> KeyboardModifier {
        let one = self.as_u32();
        let other = other.as_u32();
        Self::Combination(one & other)
    }

    #[inline]
    pub fn has(&self, has: KeyboardModifier) -> bool {
        match self {
            Self::Combination(mask) => {
                let has = has.as_u32();
                mask & has != 0
            }
            _ => *self == has,
        }
    }

    #[inline]
    pub fn shift(&self) -> bool {
        self.has(KeyboardModifier::ShiftModifier)
    }

    #[inline]
    pub fn ctrl(&self) -> bool {
        self.has(KeyboardModifier::ControlModifier)
    }

    #[inline]
    pub fn alt(&self) -> bool {
        self.has(KeyboardModifier::AltModifier)
    }

    #[inline]
    pub fn meta(&self) -> bool {
        self.has(KeyboardModifier::MetaModifier)
    }

    #[inline]
    pub fn as_u32(&self) -> u32 {
        match self {
            Self::NoModifier => 0x00000000,
            Self::ShiftModifier => 0x02000000,
            Self::ControlModifier => 0x04000000,
            Self::AltModifier => 0x08000000,
            Self::MetaModifier => 0x10000000,
            Self::KeypadModifier => 0x20000000,
            Self::GroupSwitchModifier => 0x40000000,
            Self::KeyboardModifierMask => 0xfe000000,
            Self::Combination(mask) => *mask,
        }
    }
}
impl From<KeyboardModifier> for u32 {
    fn from(val: KeyboardModifier) -> Self {
        match val {
            KeyboardModifier::NoModifier => 0x00000000,
            KeyboardModifier::ShiftModifier => 0x02000000,
            KeyboardModifier::ControlModifier => 0x04000000,
            KeyboardModifier::AltModifier => 0x08000000,
            KeyboardModifier::MetaModifier => 0x10000000,
            KeyboardModifier::KeypadModifier => 0x20000000,
            KeyboardModifier::GroupSwitchModifier => 0x40000000,
            KeyboardModifier::KeyboardModifierMask => 0xfe000000,
            KeyboardModifier::Combination(mask) => mask,
        }
    }
}
impl From<u32> for KeyboardModifier {
    fn from(value: u32) -> Self {
        match value {
            0x00000000 => Self::NoModifier,
            0x02000000 => Self::ShiftModifier,
            0x04000000 => Self::ControlModifier,
            0x08000000 => Self::AltModifier,
            0x10000000 => Self::MetaModifier,
            0x20000000 => Self::KeypadModifier,
            0x40000000 => Self::GroupSwitchModifier,
            0xfe000000 => Self::KeyboardModifierMask,
            _ => Self::Combination(value),
        }
    }
}
implements_enum_value!(KeyboardModifier, u32);

////////////////////////////////////////////////////////////////////////////////////////////////
/// [`Align`]
////////////////////////////////////////////////////////////////////////////////////////////////
#[repr(u8)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum Align {
    #[default]
    Start = 1,
    Center,
    End,
}
impl AsNumeric<u8> for Align {
    fn as_numeric(&self) -> u8 {
        self.as_u8()
    }
}
impl Align {
    pub fn as_u8(&self) -> u8 {
        match self {
            Self::Start => 1,
            Self::Center => 2,
            Self::End => 3,
        }
    }
}
impl From<u8> for Align {
    fn from(x: u8) -> Self {
        match x {
            1 => Self::Start,
            2 => Self::Center,
            3 => Self::End,
            _ => unimplemented!(),
        }
    }
}
implements_enum_value!(Align, u8);

////////////////////////////////////////////////////////////////////////////////////////////////
/// [`Coordinate`]
////////////////////////////////////////////////////////////////////////////////////////////////
#[repr(u8)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum Coordinate {
    #[default]
    World = 0,
    Widget,
}
impl AsNumeric<u8> for Coordinate {
    fn as_numeric(&self) -> u8 {
        *self as u8
    }
}
impl From<u8> for Coordinate {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::World,
            1 => Self::Widget,
            _ => unimplemented!(),
        }
    }
}
implements_enum_value!(Coordinate, u8);

#[repr(u8)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum Orientation {
    #[default]
    Horizontal = 0,
    Vertical,
}
impl AsNumeric<u8> for Orientation {
    fn as_numeric(&self) -> u8 {
        *self as u8
    }
}
impl From<u8> for Orientation {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Horizontal,
            1 => Self::Vertical,
            _ => unimplemented!(),
        }
    }
}
implements_enum_value!(Orientation, u8);

////////////////////////////////////////////////////////////////////////////////////////////////
/// [`Coordinate`]
////////////////////////////////////////////////////////////////////////////////////////////////
#[repr(u8)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum BorderStyle {
    #[default]
    Solid = 0,
    Dotted,
    Double,
    Dashed,
}
impl AsNumeric<u8> for BorderStyle {
    fn as_numeric(&self) -> u8 {
        *self as u8
    }
}
impl From<u8> for BorderStyle {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Solid,
            1 => Self::Dotted,
            2 => Self::Double,
            3 => Self::Dashed,
            _ => unimplemented!(),
        }
    }
}
implements_enum_value!(BorderStyle, u8);

////////////////////////////////////////////////////////////////////////////////////////////////
/// [`SystemCursorShape`]
////////////////////////////////////////////////////////////////////////////////////////////////
#[repr(u8)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum SystemCursorShape {
    #[default]
    ArrowCursor = 0,
    UpArrowCursor,
    CrossCursor,
    WaitCursor,
    TextCursor,
    VerticalTextCursor,

    // Cursors for adjusting window size elements.
    SizeVerCursor,
    SizeHorCursor,
    // right-top left-bottom
    SizeBDiagCursor,
    // left-top right-bottom
    SizeFDiagCursor,

    BlankCursor,
    SplitVCursor,
    SplitHCursor,
    PointingHandCursor,
    ForbiddenCursor,
    WhatsThisCursor,
    BusyCursor,
    GrabCursor,
    GrabbingCursor,
    DragCopyCursor,
    DragMoveCursor,
    DragLinkCursor,

    ZoomInCursor,
    ZoomOutCursor,

    CellCursor,
}
impl From<u8> for SystemCursorShape {
    fn from(n: u8) -> Self {
        match n {
            0 => SystemCursorShape::ArrowCursor,
            1 => SystemCursorShape::UpArrowCursor,
            2 => SystemCursorShape::CrossCursor,
            3 => SystemCursorShape::WaitCursor,
            4 => SystemCursorShape::TextCursor,
            5 => SystemCursorShape::VerticalTextCursor,
            6 => SystemCursorShape::SizeVerCursor,
            7 => SystemCursorShape::SizeHorCursor,
            8 => SystemCursorShape::SizeBDiagCursor,
            9 => SystemCursorShape::SizeFDiagCursor,
            10 => SystemCursorShape::BlankCursor,
            11 => SystemCursorShape::SplitVCursor,
            12 => SystemCursorShape::SplitHCursor,
            13 => SystemCursorShape::PointingHandCursor,
            14 => SystemCursorShape::ForbiddenCursor,
            15 => SystemCursorShape::WhatsThisCursor,
            16 => SystemCursorShape::BusyCursor,
            17 => SystemCursorShape::GrabCursor,
            18 => SystemCursorShape::GrabbingCursor,
            19 => SystemCursorShape::DragCopyCursor,
            20 => SystemCursorShape::DragMoveCursor,
            21 => SystemCursorShape::DragLinkCursor,
            22 => SystemCursorShape::ZoomInCursor,
            23 => SystemCursorShape::ZoomOutCursor,
            24 => SystemCursorShape::CellCursor,
            _ => unimplemented!(),
        }
    }
}
impl From<SystemCursorShape> for CursorIcon {
    fn from(val: SystemCursorShape) -> Self {
        match val {
            SystemCursorShape::ArrowCursor => CursorIcon::Default,
            SystemCursorShape::UpArrowCursor => CursorIcon::ContextMenu,
            SystemCursorShape::CrossCursor => CursorIcon::Crosshair,
            SystemCursorShape::WaitCursor => CursorIcon::Wait,
            SystemCursorShape::TextCursor => CursorIcon::Text,
            SystemCursorShape::VerticalTextCursor => CursorIcon::VerticalText,
            SystemCursorShape::SizeVerCursor => CursorIcon::NsResize,
            SystemCursorShape::SizeHorCursor => CursorIcon::EwResize,
            SystemCursorShape::SizeBDiagCursor => CursorIcon::NeswResize,
            SystemCursorShape::SizeFDiagCursor => CursorIcon::NwseResize,
            SystemCursorShape::BlankCursor => panic!("Should use `Window::set_cursor_visible()` instead."),
            SystemCursorShape::SplitVCursor => CursorIcon::RowResize,
            SystemCursorShape::SplitHCursor => CursorIcon::ColResize,
            SystemCursorShape::PointingHandCursor => CursorIcon::Pointer,
            SystemCursorShape::ForbiddenCursor => CursorIcon::NotAllowed,
            SystemCursorShape::WhatsThisCursor => CursorIcon::Help,
            SystemCursorShape::BusyCursor => CursorIcon::Progress,
            SystemCursorShape::GrabCursor => CursorIcon::Grab,
            SystemCursorShape::GrabbingCursor => CursorIcon::Grabbing,
            SystemCursorShape::DragCopyCursor => CursorIcon::Copy,
            SystemCursorShape::DragMoveCursor => CursorIcon::Move,
            SystemCursorShape::DragLinkCursor => CursorIcon::Alias,
            SystemCursorShape::ZoomInCursor => CursorIcon::ZoomIn,
            SystemCursorShape::ZoomOutCursor => CursorIcon::ZoomOut,
            SystemCursorShape::CellCursor => CursorIcon::Cell,
        }
    }
}
impl AsNumeric<u8> for SystemCursorShape {
    fn as_numeric(&self) -> u8 {
        *self as u8
    }
}
implements_enum_value!(SystemCursorShape, u8);

////////////////////////////////////////////////////////////////////////////////////////////////
/// [`Coordinate`]
////////////////////////////////////////////////////////////////////////////////////////////////
#[repr(u32)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, EnumIter)]
pub enum MouseButton {
    #[default]
    NoButton,
    LeftButton,
    RightButton,
    MiddleButton,
    BackButton,
    ForwardButton,
    Other(u32),
    Combination(u32),
}
impl AsNumeric<u32> for MouseButton {
    #[inline]
    fn as_numeric(&self) -> u32 {
        self.as_u32()
    }
}
impl MouseButton {
    pub fn or(&self, other: MouseButton) -> MouseButton {
        let one = self.as_u32();
        let other = other.as_u32();
        let after = one | other;

        if one == 0 {
            for b in Self::iter() {
                if after == b.as_u32() {
                    return b;
                }
            }
        }
        Self::Combination(after)
    }

    pub fn remove(&self, remove: MouseButton) -> MouseButton {
        match self {
            Self::Combination(data) => {
                let other = remove.as_u32();
                let after_remove = *data & !(*data & other);

                for b in Self::iter() {
                    if after_remove == b.as_u32() {
                        return b;
                    }
                }
                Self::Combination(after_remove)
            }
            _ => {
                if *self == remove {
                    Self::NoButton
                } else {
                    *self
                }
            }
        }
    }

    #[inline]
    pub fn has(&self, has: MouseButton) -> bool {
        match self {
            Self::Combination(data) => {
                let has = has.as_u32();
                data & has != 0
            }
            _ => *self == has,
        }
    }

    pub fn as_u32(&self) -> u32 {
        match self {
            MouseButton::NoButton => 0x00000000,
            MouseButton::LeftButton => 0x00000001,
            MouseButton::RightButton => 0x00000002,
            MouseButton::MiddleButton => 0x00000004,
            MouseButton::BackButton => 0x00000008,
            MouseButton::ForwardButton => 0x00000010,
            MouseButton::Other(x) => *x,
            MouseButton::Combination(x) => *x,
        }
    }
}
impl From<u32> for MouseButton {
    fn from(x: u32) -> Self {
        match x {
            0x00000000 => MouseButton::NoButton,
            0x00000001 => MouseButton::LeftButton,
            0x00000002 => MouseButton::RightButton,
            0x00000004 => MouseButton::MiddleButton,
            _ => MouseButton::Combination(x),
        }
    }
}
impl From<WinitMouseButton> for MouseButton {
    fn from(val: WinitMouseButton) -> Self {
        match val {
            WinitMouseButton::Left => MouseButton::LeftButton,
            WinitMouseButton::Right => MouseButton::RightButton,
            WinitMouseButton::Middle => MouseButton::MiddleButton,
            WinitMouseButton::Back => MouseButton::BackButton,
            WinitMouseButton::Forward => MouseButton::ForwardButton,
            WinitMouseButton::Other(x) => MouseButton::Other(x as u32),
        }
    }
}
implements_enum_value!(MouseButton, u32);

////////////////////////////////////////////////////////////////////////////////////////////////
/// [`ExitStatus`]
////////////////////////////////////////////////////////////////////////////////////////////////
#[repr(u8)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum ExitStatus {
    #[default]
    NormalExit = 0,
    CrashExit,
}
impl AsNumeric<u8> for ExitStatus {
    fn as_numeric(&self) -> u8 {
        match self {
            Self::NormalExit => 0,
            Self::CrashExit => 1,
        }
    }
}
impl From<u8> for ExitStatus {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::NormalExit,
            1 => Self::CrashExit,
            _ => unimplemented!(),
        }
    }
}
implements_enum_value!(ExitStatus, u8);

////////////////////////////////////////////////////////////////////////////////////////////////
/// [`ImageOption`]
////////////////////////////////////////////////////////////////////////////////////////////////
#[repr(u8)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum ImageOption {
    #[default]
    Fill = 0,
    Adapt,
    Tile,
    Stretch,
    Center,
}
impl AsNumeric<u8> for ImageOption {
    #[inline]
    fn as_numeric(&self) -> u8 {
        *self as u8
    }
}
impl From<u8> for ImageOption {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Fill,
            1 => Self::Adapt,
            2 => Self::Tile,
            3 => Self::Stretch,
            4 => Self::Center,
            _ => unreachable!(),
        }
    }
}
implements_enum_value!(ImageOption, u8);

////////////////////////////////////////////////////////////////////////////////////////////////
/// [`BlendMode`]
////////////////////////////////////////////////////////////////////////////////////////////////
#[repr(u8)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, EnumIter)]
pub enum BlendMode {
    Clear = 0,
    Src,
    Dst,
    #[default]
    SrcOver,
    DstOver,
    ScrIn,
    DstIn,
    SrcOut,
    DstOut,
    SrcATop,
    DstATop,
    Xor,
    Plus,
    Modulate,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
    Multiply,
    Hue,
    Saturation,
    Color,
    Luminosity,
}
impl AsNumeric<u8> for BlendMode {
    #[inline]
    fn as_numeric(&self) -> u8 {
        *self as u8
    }
}
impl From<u8> for BlendMode {
    fn from(value: u8) -> Self {
        for mode in Self::iter() {
            if mode.as_numeric() == value {
                return mode;
            }
        }
        unreachable!()
    }
}
implements_enum_value!(BlendMode, u8);
impl From<BlendMode> for SkiaBlendMode {
    fn from(val: BlendMode) -> Self {
        match val {
            BlendMode::Clear => SkiaBlendMode::Clear,
            BlendMode::Src => SkiaBlendMode::Src,
            BlendMode::Dst => SkiaBlendMode::Dst,
            BlendMode::SrcOver => SkiaBlendMode::SrcOver,
            BlendMode::DstOver => SkiaBlendMode::DstOver,
            BlendMode::ScrIn => SkiaBlendMode::SrcIn,
            BlendMode::DstIn => SkiaBlendMode::DstIn,
            BlendMode::SrcOut => SkiaBlendMode::SrcOut,
            BlendMode::DstOut => SkiaBlendMode::DstOut,
            BlendMode::SrcATop => SkiaBlendMode::SrcATop,
            BlendMode::DstATop => SkiaBlendMode::DstATop,
            BlendMode::Xor => SkiaBlendMode::Xor,
            BlendMode::Plus => SkiaBlendMode::Plus,
            BlendMode::Modulate => SkiaBlendMode::Modulate,
            BlendMode::Screen => SkiaBlendMode::Screen,
            BlendMode::Overlay => SkiaBlendMode::Overlay,
            BlendMode::Darken => SkiaBlendMode::Darken,
            BlendMode::Lighten => SkiaBlendMode::Lighten,
            BlendMode::ColorDodge => SkiaBlendMode::ColorDodge,
            BlendMode::ColorBurn => SkiaBlendMode::ColorBurn,
            BlendMode::HardLight => SkiaBlendMode::HardLight,
            BlendMode::SoftLight => SkiaBlendMode::SoftLight,
            BlendMode::Difference => SkiaBlendMode::Difference,
            BlendMode::Exclusion => SkiaBlendMode::Exclusion,
            BlendMode::Multiply => SkiaBlendMode::Multiply,
            BlendMode::Hue => SkiaBlendMode::Hue,
            BlendMode::Saturation => SkiaBlendMode::Saturation,
            BlendMode::Color => SkiaBlendMode::Color,
            BlendMode::Luminosity => SkiaBlendMode::Luminosity,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::ToValue;

    use super::{
        Align, BorderStyle, Coordinate, ExitStatus, ImageOption, KeyCode, KeyboardModifier,
        MouseButton, Orientation, SystemCursorShape,
    };

    #[test]
    fn test_key_code_value() {
        let code = KeyCode::KeyBraceLeft;
        let val = code.to_value();
        assert_eq!(code, val.get::<KeyCode>())
    }

    #[test]
    fn test_keyboard_modifier_value() {
        let modifier = KeyboardModifier::MetaModifier;
        let val = modifier.to_value();
        assert_eq!(modifier, val.get::<KeyboardModifier>())
    }

    #[test]
    fn test_align_value() {
        let val = Align::Center.to_value();
        assert_eq!(val.get::<Align>(), Align::Center);
    }

    #[test]
    fn test_coordinate_value() {
        let coord = Coordinate::World;
        let val = coord.to_value();
        assert_eq!(val.get::<Coordinate>(), coord);
    }

    #[test]
    fn test_orientation_value() {
        let ori = Orientation::Vertical;
        let val = ori.to_value();
        assert_eq!(val.get::<Orientation>(), ori);
    }

    #[test]
    fn test_border_style_value() {
        let style = BorderStyle::Double;
        let val = style.to_value();
        assert_eq!(style, val.get::<BorderStyle>());
    }

    #[test]
    fn test_system_cursor_shape() {
        let val = SystemCursorShape::CrossCursor.to_value();
        assert_eq!(
            val.get::<SystemCursorShape>(),
            SystemCursorShape::CrossCursor
        )
    }

    #[test]
    fn test_exit_status() {
        let val = ExitStatus::CrashExit.to_value();
        assert_eq!(ExitStatus::CrashExit, val.get::<ExitStatus>())
    }

    #[test]
    fn test_image_option() {
        let val = ImageOption::Stretch.to_value();
        assert_eq!(ImageOption::Stretch, val.get::<ImageOption>())
    }

    #[test]
    fn test_key_code_ord() {
        let code = KeyCode::KeyC;
        assert!(code >= KeyCode::KeyA && code <= KeyCode::KeyZ);
    }

    #[test]
    fn test_modifier_remove() {
        let mut m = KeyboardModifier::AltModifier;
        m = m.or(KeyboardModifier::ShiftModifier);
        let rec_alt_shift = m;
        m = m.or(KeyboardModifier::ControlModifier);

        m = m.remove(KeyboardModifier::ControlModifier);
        assert_eq!(m, rec_alt_shift);

        m = m.remove(KeyboardModifier::ShiftModifier);
        assert_eq!(m, KeyboardModifier::AltModifier);

        m = m.remove(KeyboardModifier::AltModifier);
        assert_eq!(m, KeyboardModifier::NoModifier);
    }

    #[test]
    fn test_mouse_button_remove() {
        let mut button = MouseButton::LeftButton;
        button = button.or(MouseButton::MiddleButton);
        let rec_left_middle = button;
        button = button.or(MouseButton::RightButton);

        button = button.remove(MouseButton::RightButton);
        assert_eq!(button, rec_left_middle);

        button = button.remove(MouseButton::MiddleButton);
        assert_eq!(button, MouseButton::LeftButton);

        button = button.remove(MouseButton::LeftButton);
        assert_eq!(button, MouseButton::NoButton);
    }
}
