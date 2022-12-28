#[repr(u32)]
pub enum KeyboardModifier {
    NoModifier = 0x00000000,
    ShiftModifier = 0x02000000,
    ControlModifier = 0x04000000,
    AltModifier = 0x08000000,
    MetaModifier = 0x10000000,
    KeypadModifier = 0x20000000,
    GroupSwitchModifier = 0x40000000,

    KeyboardModifierMask = 0xfe000000,
}
impl Into<u32> for KeyboardModifier {
    fn into(self) -> u32 {
        match self {
            Self::NoModifier => Self::NoModifier as u32,
            Self::ShiftModifier => Self::ShiftModifier as u32,
            Self::ControlModifier => Self::ControlModifier as u32,
            Self::AltModifier => Self::AltModifier as u32,
            Self::MetaModifier => Self::MetaModifier as u32,
            Self::KeypadModifier => Self::KeypadModifier as u32,
            Self::GroupSwitchModifier => Self::GroupSwitchModifier as u32,
            Self::KeyboardModifierMask => Self::KeyboardModifierMask as u32,
        }
    }
}