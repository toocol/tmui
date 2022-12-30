use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// The enum to represent the key code on keyboard.
#[repr(u32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, EnumIter)]
pub enum KeyCode {
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
    Unknown = 0x00,
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
            Self::Unknown => "Unknown".to_string(),
        }
    }
}

/// The enum to represent the keyboard modifier.
#[repr(u32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum KeyboardModifier {
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
                mask & has > 0
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
