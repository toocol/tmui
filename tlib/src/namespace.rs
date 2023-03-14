use crate::{
    prelude::{StaticType, ToValue},
    values::{FromBytes, FromValue, ToBytes},
    Type, Value,
};
use std::mem::size_of;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

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

    // Unicode Basic Latin block (0x00-0x7f)
    KeySpace = 0x20,
    KeyExclam = 0x21,
    KeyQuoteDbl = 0x22,
    KeyNumberSign = 0x23,
    KeyDollar = 0x24,
    KeyPercent = 0x25,
    KeyAmpersand = 0x26,
    KeyApostrophe = 0x27,
    KeyParenLeft = 0x28,
    KeyParenRight = 0x29,
    KeyAsterisk = 0x2a,
    KeyPlus = 0x2b,
    KeyComma = 0x2c,
    KeyMinus = 0x2d,
    KeyPeriod = 0x2e,
    KeySlash = 0x2f,
    Key0 = 0x30,
    Key1 = 0x31,
    Key2 = 0x32,
    Key3 = 0x33,
    Key4 = 0x34,
    Key5 = 0x35,
    Key6 = 0x36,
    Key7 = 0x37,
    Key8 = 0x38,
    Key9 = 0x39,
    KeyColon = 0x3a,
    KeySemicolon = 0x3b,
    KeyLess = 0x3c,
    KeyEqual = 0x3d,
    KeyGreater = 0x3e,
    KeyQuestion = 0x3f,
    KeyAt = 0x40,
    KeyA = 0x41,
    KeyB = 0x42,
    KeyC = 0x43,
    KeyD = 0x44,
    KeyE = 0x45,
    KeyF = 0x46,
    KeyG = 0x47,
    KeyH = 0x48,
    KeyI = 0x49,
    KeyJ = 0x4a,
    KeyK = 0x4b,
    KeyL = 0x4c,
    KeyM = 0x4d,
    KeyN = 0x4e,
    KeyO = 0x4f,
    KeyP = 0x50,
    KeyQ = 0x51,
    KeyR = 0x52,
    KeyS = 0x53,
    KeyT = 0x54,
    KeyU = 0x55,
    KeyV = 0x56,
    KeyW = 0x57,
    KeyX = 0x58,
    KeyY = 0x59,
    KeyZ = 0x5a,
    KeyBracketLeft = 0x5b,
    KeyBackslash = 0x5c,
    KeyBracketRight = 0x5d,
    KeyAsciiCircum = 0x5e,
    KeyUnderscore = 0x5f,
    KeyQuoteLeft = 0x60,
    KeyBraceLeft = 0x7b,
    KeyBar = 0x7c,
    KeyBraceRight = 0x7d,
    KeyAsciiTilde = 0x7e,

    KeyEscape = 0x01000000, // misc keys
    KeyTab = 0x01000001,
    KeyBacktab = 0x01000002,
    KeyBackspace = 0x01000003,
    KeyReturn = 0x01000004,
    KeyEnter = 0x01000005,
    KeyInsert = 0x01000006,
    KeyDelete = 0x01000007,
    KeyPause = 0x01000008,
    KeyPrint = 0x01000009, // print screen
    KeySysReq = 0x0100000a,
    KeyClear = 0x0100000b,
    KeyHome = 0x01000010, // cursor movement
    KeyEnd = 0x01000011,
    KeyLeft = 0x01000012,
    KeyUp = 0x01000013,
    KeyRight = 0x01000014,
    KeyDown = 0x01000015,
    KeyPageUp = 0x01000016,
    KeyPageDown = 0x01000017,
    KeyShift = 0x01000020, // modifiers
    KeyControl = 0x01000021,
    KeyMeta = 0x01000022,
    KeyAlt = 0x01000023,
    KeyCapsLock = 0x01000024,
    KeyNumLock = 0x01000025,
    KeyScrollLock = 0x01000026,
    KeyF1 = 0x01000030, // function keys
    KeyF2 = 0x01000031,
    KeyF3 = 0x01000032,
    KeyF4 = 0x01000033,
    KeyF5 = 0x01000034,
    KeyF6 = 0x01000035,
    KeyF7 = 0x01000036,
    KeyF8 = 0x01000037,
    KeyF9 = 0x01000038,
    KeyF10 = 0x01000039,
    KeyF11 = 0x0100003a,
    KeyF12 = 0x0100003b,
    KeyF13 = 0x0100003c,
    KeyF14 = 0x0100003d,
    KeyF15 = 0x0100003e,
    KeyF16 = 0x0100003f,
    KeyF17 = 0x01000040,
    KeyF18 = 0x01000041,
    KeyF19 = 0x01000042,
    KeyF20 = 0x01000043,
    KeyF21 = 0x01000044,
    KeyF22 = 0x01000045,
    KeyF23 = 0x01000046,
    KeyF24 = 0x01000047,
    KeyF25 = 0x01000048, // F25 .. F35 only on X11
    KeyF26 = 0x01000049,
    KeyF27 = 0x0100004a,
    KeyF28 = 0x0100004b,
    KeyF29 = 0x0100004c,
    KeyF30 = 0x0100004d,
    KeyF31 = 0x0100004e,
    KeyF32 = 0x0100004f,
    KeyF33 = 0x01000050,
    KeyF34 = 0x01000051,
    KeyF35 = 0x01000052,
    KeySuperL = 0x01000053, // extra keys
    KeySuperR = 0x01000054,
    KeyMenu = 0x01000055,
    KeyHyperL = 0x01000056,
    KeyHyperR = 0x01000057,
    KeyHelp = 0x01000058,
    KeyDirectionL = 0x01000059,
    KeyDirectionR = 0x01000060,
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
            Self::KeyAsciiCircum => "^".to_string(),
            Self::KeyUnderscore => "_".to_string(),
            Self::KeyQuoteLeft => "\"".to_string(),
            Self::KeyBraceLeft => "{".to_string(),
            Self::KeyBar => "|".to_string(),
            Self::KeyBraceRight => "}".to_string(),
            Self::KeyAsciiTilde => "~".to_string(),
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
            Self::KeyNumLock => "NumLock".to_string(),
            Self::KeyScrollLock => "ScrollLock".to_string(),
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
            Self::KeyF25 => "F25".to_string(),
            Self::KeyF26 => "F26".to_string(),
            Self::KeyF27 => "F27".to_string(),
            Self::KeyF28 => "F28".to_string(),
            Self::KeyF29 => "F29".to_string(),
            Self::KeyF30 => "F30".to_string(),
            Self::KeyF31 => "F31".to_string(),
            Self::KeyF32 => "F32".to_string(),
            Self::KeyF33 => "F33".to_string(),
            Self::KeyF34 => "F34".to_string(),
            Self::KeyF35 => "F35".to_string(),
            Self::KeySuperL => "SuperL".to_string(),
            Self::KeySuperR => "SuperR".to_string(),
            Self::KeyMenu => "Menu".to_string(),
            Self::KeyHyperL => "HyperL".to_string(),
            Self::KeyHyperR => "HyperR".to_string(),
            Self::KeyHelp => "Help".to_string(),
            Self::KeyDirectionL => "DirectionL".to_string(),
            Self::KeyDirectionR => "DirectionR".to_string(),
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
    GroupSwitchModifier,

    KeyboardModifierMask,
    Combination(u32),
}
impl AsNumeric<u32> for KeyboardModifier {
    fn as_numeric(&self) -> u32 {
        self.as_u32()
    }
}
impl KeyboardModifier {
    pub fn or(&self, other: KeyboardModifier) -> KeyboardModifier {
        let one = self.as_u32();
        let other = other.as_u32();
        Self::Combination(one | other)
    }

    pub fn has(&self, has: KeyboardModifier) -> bool {
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
    IBeamCursor,
    SizeVerCursor,
    SizeHorCursor,
    SizeBDiagCursor,
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
}
impl SystemCursorShape {
    pub fn as_u8(&self) -> u8 {
        match self {
            SystemCursorShape::ArrowCursor => 0,
            SystemCursorShape::UpArrowCursor => 1,
            SystemCursorShape::CrossCursor => 2,
            SystemCursorShape::WaitCursor => 3,
            SystemCursorShape::IBeamCursor => 4,
            SystemCursorShape::SizeVerCursor => 5,
            SystemCursorShape::SizeHorCursor => 6,
            SystemCursorShape::SizeBDiagCursor => 7,
            SystemCursorShape::SizeFDiagCursor => 8,
            SystemCursorShape::SizeAllCursor => 9,
            SystemCursorShape::BlankCursor => 10,
            SystemCursorShape::SplitVCursor => 11,
            SystemCursorShape::SplitHCursor => 12,
            SystemCursorShape::PointingHandCursor => 13,
            SystemCursorShape::ForbiddenCursor => 14,
            SystemCursorShape::WhatsThisCursor => 15,
            SystemCursorShape::BusyCursor => 16,
            SystemCursorShape::OpenHandCursor => 17,
            SystemCursorShape::ClosedHandCursor => 18,
            SystemCursorShape::DragCopyCursor => 19,
            SystemCursorShape::DragMoveCursor => 20,
            SystemCursorShape::DragLinkCursor => 21,
        }
    }
}
impl From<u8> for SystemCursorShape {
    fn from(n: u8) -> Self {
        match n {
            0 => SystemCursorShape::ArrowCursor,
            1 => SystemCursorShape::UpArrowCursor,
            2 => SystemCursorShape::CrossCursor,
            3 => SystemCursorShape::WaitCursor,
            4 => SystemCursorShape::IBeamCursor,
            5 => SystemCursorShape::SizeVerCursor,
            6 => SystemCursorShape::SizeHorCursor,
            7 => SystemCursorShape::SizeBDiagCursor,
            8 => SystemCursorShape::SizeFDiagCursor,
            9 => SystemCursorShape::SizeAllCursor,
            10 => SystemCursorShape::BlankCursor,
            11 => SystemCursorShape::SplitVCursor,
            12 => SystemCursorShape::SplitHCursor,
            13 => SystemCursorShape::PointingHandCursor,
            14 => SystemCursorShape::ForbiddenCursor,
            15 => SystemCursorShape::WhatsThisCursor,
            16 => SystemCursorShape::BusyCursor,
            17 => SystemCursorShape::OpenHandCursor,
            18 => SystemCursorShape::ClosedHandCursor,
            19 => SystemCursorShape::DragCopyCursor,
            20 => SystemCursorShape::DragMoveCursor,
            21 => SystemCursorShape::DragLinkCursor,
            _ => unimplemented!(),
        }
    }
}
impl AsNumeric<u8> for SystemCursorShape {
    fn as_numeric(&self) -> u8 {
        self.as_u8()
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
implements_enum_value!(MouseButton, u32);

#[cfg(test)]
mod tests {
    use crate::prelude::ToValue;

    use super::{Align, KeyCode, KeyboardModifier, SystemCursorShape, Coordinate, Orientation, BorderStyle};

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
}
