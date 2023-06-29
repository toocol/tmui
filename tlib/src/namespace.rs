use crate::{
    prelude::{StaticType, ToValue},
    values::{FromBytes, FromValue, ToBytes},
    Type, Value,
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
            if code.to_string() == value {
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
impl Into<u32> for KeyCode {
    fn into(self) -> u32 {
        self as u32
    }
}
impl ToString for KeyCode {
    fn to_string(&self) -> String {
        match self {
            Self::Unknown => "Unknown".to_string(),
            Self::KeySpace => " ".to_string(),
            Self::KeyExclam => "!".to_string(),
            Self::KeyQuoteDbl => "@".to_string(),
            Self::KeyNumberSign => "#".to_string(),
            Self::KeyDollar => "$".to_string(),
            Self::KeyPercent => "%".to_string(),
            Self::KeyAmpersand => "&".to_string(),
            Self::KeyApostrophe => "'".to_string(),
            Self::KeyParenLeft => "(".to_string(),
            Self::KeyParenRight => ")".to_string(),
            Self::KeyAsterisk => "*".to_string(),
            Self::KeyPlus => "+".to_string(),
            Self::KeyComma => ",".to_string(),
            Self::KeyMinus => "-".to_string(),
            Self::KeyPeriod => ".".to_string(),
            Self::KeySlash => "/".to_string(),
            Self::Key0 => "0".to_string(),
            Self::Key1 => "1".to_string(),
            Self::Key2 => "2".to_string(),
            Self::Key3 => "3".to_string(),
            Self::Key4 => "4".to_string(),
            Self::Key5 => "5".to_string(),
            Self::Key6 => "6".to_string(),
            Self::Key7 => "7".to_string(),
            Self::Key8 => "8".to_string(),
            Self::Key9 => "9".to_string(),
            Self::KeyColon => ":".to_string(),
            Self::KeySemicolon => ";".to_string(),
            Self::KeyLess => "<".to_string(),
            Self::KeyEqual => "=".to_string(),
            Self::KeyGreater => ">".to_string(),
            Self::KeyQuestion => "?".to_string(),
            Self::KeyAt => "@".to_string(),
            Self::KeyA => "A".to_string(),
            Self::KeyB => "B".to_string(),
            Self::KeyC => "C".to_string(),
            Self::KeyD => "D".to_string(),
            Self::KeyE => "E".to_string(),
            Self::KeyF => "F".to_string(),
            Self::KeyG => "G".to_string(),
            Self::KeyH => "H".to_string(),
            Self::KeyI => "I".to_string(),
            Self::KeyJ => "J".to_string(),
            Self::KeyK => "K".to_string(),
            Self::KeyL => "L".to_string(),
            Self::KeyM => "M".to_string(),
            Self::KeyN => "N".to_string(),
            Self::KeyO => "O".to_string(),
            Self::KeyP => "P".to_string(),
            Self::KeyQ => "Q".to_string(),
            Self::KeyR => "R".to_string(),
            Self::KeyS => "S".to_string(),
            Self::KeyT => "T".to_string(),
            Self::KeyU => "U".to_string(),
            Self::KeyV => "V".to_string(),
            Self::KeyW => "W".to_string(),
            Self::KeyX => "X".to_string(),
            Self::KeyY => "Y".to_string(),
            Self::KeyZ => "Z".to_string(),
            Self::KeyBracketLeft => "[".to_string(),
            Self::KeyBackslash => "\\".to_string(),
            Self::KeyBracketRight => "]".to_string(),
            Self::KeyCaret => "^".to_string(),
            Self::KeyUnderscore => "_".to_string(),
            Self::KeyQuoteLeft => "\"".to_string(),
            Self::KeyBraceLeft => "{".to_string(),
            Self::KeyBar => "|".to_string(),
            Self::KeyBraceRight => "}".to_string(),
            Self::KeyTilde => "~".to_string(),
            Self::KeyEscape => "Escape".to_string(),
            Self::KeyTab => "Tab".to_string(),
            Self::KeyBacktab => "Backtab".to_string(),
            Self::KeyBackspace => "Backspace".to_string(),
            Self::KeyReturn => "Return".to_string(),
            Self::KeyEnter => "Enter".to_string(),
            Self::KeyInsert => "Insert".to_string(),
            Self::KeyDelete => "Delete".to_string(),
            Self::KeyPause => "Pause".to_string(),
            Self::KeyPrint => "Print".to_string(),
            Self::KeySysReq => "SysReq".to_string(),
            Self::KeyClear => "Clear".to_string(),
            Self::KeyHome => "Home".to_string(),
            Self::KeyEnd => "End".to_string(),
            Self::KeyLeft => "Left".to_string(),
            Self::KeyUp => "Up".to_string(),
            Self::KeyRight => "Right".to_string(),
            Self::KeyDown => "Down".to_string(),
            Self::KeyPageUp => "PageUp".to_string(),
            Self::KeyPageDown => "PageDown".to_string(),
            Self::KeyShift => "Shift".to_string(),
            Self::KeyControl => "Control".to_string(),
            Self::KeyMeta => "Meta".to_string(),
            Self::KeyAlt => "Alt".to_string(),
            Self::KeyCapsLock => "CapsLock".to_string(),
            Self::KeyScrollLock => "ScrollLock".to_string(),
            Self::KeyNumLock => "NumLock".to_string(),
            Self::KeyNumpad0 => "0".to_string(),
            Self::KeyNumpad1 => "1".to_string(),
            Self::KeyNumpad2 => "2".to_string(),
            Self::KeyNumpad3 => "3".to_string(),
            Self::KeyNumpad4 => "4".to_string(),
            Self::KeyNumpad5 => "5".to_string(),
            Self::KeyNumpad6 => "6".to_string(),
            Self::KeyNumpad7 => "7".to_string(),
            Self::KeyNumpad8 => "8".to_string(),
            Self::KeyNumpad9 => "9".to_string(),
            Self::KeyNumpadAdd => "+".to_string(),
            Self::KeyNumpadDivide => "/".to_string(),
            Self::KeyNumpadDecimal => ".".to_string(),
            Self::KeyNumpadComma => ",".to_string(),
            Self::KeyNumpadEnter => "Enter".to_string(),
            Self::KeyNumpadEquals => "=".to_string(),
            Self::KeyNumpadMultiply => "*".to_string(),
            Self::KeyNumpadSubtract => "-".to_string(),
            Self::KeyF1 => "F1".to_string(),
            Self::KeyF2 => "F2".to_string(),
            Self::KeyF3 => "F3".to_string(),
            Self::KeyF4 => "F4".to_string(),
            Self::KeyF5 => "F5".to_string(),
            Self::KeyF6 => "F6".to_string(),
            Self::KeyF7 => "F7".to_string(),
            Self::KeyF8 => "F8".to_string(),
            Self::KeyF9 => "F9".to_string(),
            Self::KeyF10 => "F10".to_string(),
            Self::KeyF11 => "F11".to_string(),
            Self::KeyF12 => "F12".to_string(),
            Self::KeyF13 => "F13".to_string(),
            Self::KeyF14 => "F14".to_string(),
            Self::KeyF15 => "F15".to_string(),
            Self::KeyF16 => "F16".to_string(),
            Self::KeyF17 => "F17".to_string(),
            Self::KeyF18 => "F18".to_string(),
            Self::KeyF19 => "F19".to_string(),
            Self::KeyF20 => "F20".to_string(),
            Self::KeyF21 => "F21".to_string(),
            Self::KeyF22 => "F22".to_string(),
            Self::KeyF23 => "F23".to_string(),
            Self::KeyF24 => "F24".to_string(),
            Self::KeySuperL => "SuperL".to_string(),
            Self::KeySuperR => "SuperR".to_string(),
            Self::KeyMenu => "Menu".to_string(),
            Self::KeyHyperL => "HyperL".to_string(),
            Self::KeyHyperR => "HyperR".to_string(),
            Self::KeyHelp => "Help".to_string(),
            Self::KeyDirectionL => "DirectionL".to_string(),
            Self::KeyDirectionR => "DirectionR".to_string(),
            Self::KeyCompose => "Compose".to_string(),
            Self::KeyAbntC1 => "AbntC1".to_string(),
            Self::KeyAbntC2 => "AbntC2".to_string(),
            Self::KeyCalculator => "Calculator".to_string(),
            Self::KeyKana => "Kana".to_string(),
            Self::KeyKanji => "Kanji".to_string(),
            Self::KeyMail => "Mail".to_string(),
            Self::KeyMediaSelect => "MediaSel".to_string(),
            Self::KeyMediaStop => "MediaStop".to_string(),
            Self::KeyMute => "Mute".to_string(),
            Self::KeyUnderLine => "_".to_string(),
            Self::KeyVolumeDown => "VolumeDown".to_string(),
            Self::KeyVolumeUp => "VolumeUp".to_string(),
        }
    }
}
impl Into<KeyCode> for winit::keyboard::KeyCode {
    fn into(self) -> KeyCode {
        match self {
            Self::Digit1 => KeyCode::Key1,
            Self::Digit2 => KeyCode::Key2,
            Self::Digit3 => KeyCode::Key3,
            Self::Digit4 => KeyCode::Key4,
            Self::Digit5 => KeyCode::Key5,
            Self::Digit6 => KeyCode::Key6,
            Self::Digit7 => KeyCode::Key7,
            Self::Digit8 => KeyCode::Key8,
            Self::Digit9 => KeyCode::Key9,
            Self::Digit0 => KeyCode::Key0,
            Self::KeyA => KeyCode::KeyA,
            Self::KeyB => KeyCode::KeyB,
            Self::KeyC => KeyCode::KeyC,
            Self::KeyD => KeyCode::KeyD,
            Self::KeyE => KeyCode::KeyE,
            Self::KeyF => KeyCode::KeyF,
            Self::KeyG => KeyCode::KeyG,
            Self::KeyH => KeyCode::KeyH,
            Self::KeyI => KeyCode::KeyI,
            Self::KeyJ => KeyCode::KeyJ,
            Self::KeyK => KeyCode::KeyK,
            Self::KeyL => KeyCode::KeyL,
            Self::KeyM => KeyCode::KeyM,
            Self::KeyN => KeyCode::KeyN,
            Self::KeyO => KeyCode::KeyO,
            Self::KeyP => KeyCode::KeyP,
            Self::KeyQ => KeyCode::KeyQ,
            Self::KeyR => KeyCode::KeyR,
            Self::KeyS => KeyCode::KeyS,
            Self::KeyT => KeyCode::KeyT,
            Self::KeyU => KeyCode::KeyU,
            Self::KeyV => KeyCode::KeyV,
            Self::KeyW => KeyCode::KeyW,
            Self::KeyX => KeyCode::KeyX,
            Self::KeyY => KeyCode::KeyY,
            Self::KeyZ => KeyCode::KeyZ,
            Self::Escape => KeyCode::KeyEscape,
            Self::F1 => KeyCode::KeyF1,
            Self::F2 => KeyCode::KeyF2,
            Self::F3 => KeyCode::KeyF3,
            Self::F4 => KeyCode::KeyF4,
            Self::F5 => KeyCode::KeyF5,
            Self::F6 => KeyCode::KeyF6,
            Self::F7 => KeyCode::KeyF7,
            Self::F8 => KeyCode::KeyF8,
            Self::F9 => KeyCode::KeyF9,
            Self::F10 => KeyCode::KeyF10,
            Self::F11 => KeyCode::KeyF11,
            Self::F12 => KeyCode::KeyF12,
            Self::F13 => KeyCode::KeyF13,
            Self::F14 => KeyCode::KeyF14,
            Self::F15 => KeyCode::KeyF15,
            Self::F16 => KeyCode::KeyF16,
            Self::F17 => KeyCode::KeyF17,
            Self::F18 => KeyCode::KeyF18,
            Self::F19 => KeyCode::KeyF19,
            Self::F20 => KeyCode::KeyF20,
            Self::F21 => KeyCode::KeyF21,
            Self::F22 => KeyCode::KeyF22,
            Self::F23 => KeyCode::KeyF23,
            Self::F24 => KeyCode::KeyF24,
            Self::PrintScreen => KeyCode::KeyPrint,
            Self::ScrollLock => KeyCode::KeyScrollLock,
            Self::Pause => KeyCode::KeyPause,
            Self::Insert => KeyCode::KeyInsert,
            Self::Home => KeyCode::KeyHome,
            Self::Delete => KeyCode::KeyBackspace,
            Self::End => KeyCode::KeyEnd,
            Self::PageDown => KeyCode::KeyPageDown,
            Self::PageUp => KeyCode::KeyPageUp,
            Self::ArrowLeft => KeyCode::KeyLeft,
            Self::ArrowUp => KeyCode::KeyUp,
            Self::ArrowRight => KeyCode::KeyRight,
            Self::ArrowDown => KeyCode::KeyDown,
            Self::Enter => KeyCode::KeyEnter,
            Self::Space => KeyCode::KeySpace,
            Self::NumLock => KeyCode::KeyNumLock,
            Self::Numpad0 => KeyCode::KeyNumpad0,
            Self::Numpad1 => KeyCode::KeyNumpad1,
            Self::Numpad2 => KeyCode::KeyNumpad2,
            Self::Numpad3 => KeyCode::KeyNumpad3,
            Self::Numpad4 => KeyCode::KeyNumpad4,
            Self::Numpad5 => KeyCode::KeyNumpad5,
            Self::Numpad6 => KeyCode::KeyNumpad6,
            Self::Numpad7 => KeyCode::KeyNumpad7,
            Self::Numpad8 => KeyCode::KeyNumpad8,
            Self::Numpad9 => KeyCode::KeyNumpad9,
            Self::NumpadAdd => KeyCode::KeyNumpadAdd,
            Self::NumpadDivide => KeyCode::KeyNumpadDivide,
            Self::NumpadDecimal => KeyCode::KeyNumpadDecimal,
            Self::NumpadComma => KeyCode::KeyNumpadComma,
            Self::NumpadEnter => KeyCode::KeyNumpadEnter,
            Self::NumpadEqual => KeyCode::KeyNumpadEquals,
            Self::NumpadMultiply => KeyCode::KeyNumpadMultiply,
            Self::NumpadSubtract => KeyCode::KeyNumpadSubtract,
            Self::Backslash => KeyCode::KeyBackslash,
            Self::Comma => KeyCode::KeyComma,
            Self::Equal => KeyCode::KeyEqual,
            Self::AltLeft => KeyCode::KeyAlt,
            Self::BracketLeft => KeyCode::KeyBracketLeft,
            Self::ControlLeft => KeyCode::KeyControl,
            Self::ShiftLeft => KeyCode::KeyShift,
            Self::Meta => KeyCode::KeyMeta,
            Self::MediaSelect => KeyCode::KeyMediaSelect,
            Self::MediaStop => KeyCode::KeyMediaStop,
            Self::Minus => KeyCode::KeyMinus,
            Self::Period => KeyCode::KeyPeriod,
            Self::AltRight => KeyCode::KeyAlt,
            Self::BracketRight => KeyCode::KeyBracketRight,
            Self::ControlRight => KeyCode::KeyControl,
            Self::ShiftRight => KeyCode::KeyShift,
            Self::Semicolon => KeyCode::KeySemicolon,
            Self::Slash => KeyCode::KeySlash,
            Self::Tab => KeyCode::KeyTab,
            _ => KeyCode::Unknown,
        }
    }
}
implements_enum_value!(KeyCode, u32);

////////////////////////////////////////////////////////////////////////////////////////////////
/// [`KeyboardModifier`]
////////////////////////////////////////////////////////////////////////////////////////////////
/// The enum to represent the keyboard modifier.
#[repr(u32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
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
        Self::Combination(one | other)
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
impl Into<u32> for KeyboardModifier {
    fn into(self) -> u32 {
        match self {
            Self::NoModifier => 0x00000000,
            Self::ShiftModifier => 0x02000000,
            Self::ControlModifier => 0x04000000,
            Self::AltModifier => 0x08000000,
            Self::MetaModifier => 0x10000000,
            Self::KeypadModifier => 0x20000000,
            Self::GroupSwitchModifier => 0x40000000,
            Self::KeyboardModifierMask => 0xfe000000,
            Self::Combination(mask) => mask,
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
    SizeAllCursor,

    BlankCursor,
    SplitVCursor,
    SplitHCursor,
    PointingHandCursor,
    ForbiddenCursor,
    WhatsThisCursor,
    BusyCursor,
    OpenHandCursor,
    ClosedHandCursor,
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
            10 => SystemCursorShape::SizeAllCursor,
            11 => SystemCursorShape::BlankCursor,
            12 => SystemCursorShape::SplitVCursor,
            13 => SystemCursorShape::SplitHCursor,
            14 => SystemCursorShape::PointingHandCursor,
            15 => SystemCursorShape::ForbiddenCursor,
            16 => SystemCursorShape::WhatsThisCursor,
            17 => SystemCursorShape::BusyCursor,
            18 => SystemCursorShape::OpenHandCursor,
            19 => SystemCursorShape::ClosedHandCursor,
            20 => SystemCursorShape::DragCopyCursor,
            21 => SystemCursorShape::DragMoveCursor,
            22 => SystemCursorShape::DragLinkCursor,
            23 => SystemCursorShape::ZoomInCursor,
            24 => SystemCursorShape::ZoomOutCursor,
            25 => SystemCursorShape::CellCursor,
            _ => unimplemented!(),
        }
    }
}
impl Into<CursorIcon> for SystemCursorShape {
    fn into(self) -> CursorIcon {
        match self {
            Self::ArrowCursor => CursorIcon::Default,
            Self::UpArrowCursor => CursorIcon::ContextMenu,
            Self::CrossCursor => CursorIcon::Crosshair,
            Self::WaitCursor => CursorIcon::Wait,
            Self::TextCursor => CursorIcon::Text,
            Self::VerticalTextCursor => CursorIcon::VerticalText,
            Self::SizeVerCursor => CursorIcon::NResize,
            Self::SizeHorCursor => CursorIcon::SResize,
            Self::SizeBDiagCursor => CursorIcon::NeResize,
            Self::SizeFDiagCursor => CursorIcon::SwResize,
            Self::SizeAllCursor => CursorIcon::NeswResize,
            Self::BlankCursor => panic!("Should use `Window::set_cursor_visible()` instead."),
            Self::SplitVCursor => CursorIcon::RowResize,
            Self::SplitHCursor => CursorIcon::ColResize,
            Self::PointingHandCursor => CursorIcon::Pointer,
            Self::ForbiddenCursor => CursorIcon::NotAllowed,
            Self::WhatsThisCursor => CursorIcon::Help,
            Self::BusyCursor => CursorIcon::Progress,
            Self::OpenHandCursor => CursorIcon::Grab,
            Self::ClosedHandCursor => CursorIcon::Grabbing,
            Self::DragCopyCursor => CursorIcon::Copy,
            Self::DragMoveCursor => CursorIcon::Move,
            Self::DragLinkCursor => CursorIcon::Alias,
            Self::ZoomInCursor => CursorIcon::ZoomIn,
            Self::ZoomOutCursor => CursorIcon::ZoomOut,
            Self::CellCursor => CursorIcon::Cell,
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
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
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
    fn as_numeric(&self) -> u32 {
        self.as_u32()
    }
}
impl MouseButton {
    pub fn or(&self, other: MouseButton) -> MouseButton {
        let one = self.as_u32();
        let other = other.as_u32();
        Self::Combination(one | other)
    }

    pub fn has(&self, has: MouseButton) -> bool {
        match self {
            Self::Combination(mask) => {
                let has = has.as_u32();
                mask & has != 0
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
impl Into<MouseButton> for winit::event::MouseButton {
    fn into(self) -> MouseButton {
        match self {
            winit::event::MouseButton::Left => MouseButton::LeftButton,
            winit::event::MouseButton::Right => MouseButton::RightButton,
            winit::event::MouseButton::Middle => MouseButton::MiddleButton,
            winit::event::MouseButton::Back => MouseButton::BackButton,
            winit::event::MouseButton::Forward => MouseButton::ForwardButton,
            winit::event::MouseButton::Other(x) => MouseButton::Other(x as u32),
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
            _ => unreachable!()
        }
    }
}
implements_enum_value!(ImageOption, u8);

#[cfg(test)]
mod tests {
    use crate::prelude::ToValue;

    use super::{
        Align, BorderStyle, Coordinate, KeyCode, KeyboardModifier, Orientation, SystemCursorShape, ExitStatus, ImageOption,
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
}
