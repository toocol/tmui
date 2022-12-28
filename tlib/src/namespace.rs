#[repr(u32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum KeyboardModifier {
    NoModifier = 0x00000000,
    ShiftModifier = 0x02000000,
    ControlModifier = 0x04000000,
    AltModifier = 0x08000000,
    MetaModifier = 0x10000000,
    KeypadModifier = 0x20000000,
    GroupSwitchModifier = 0x40000000,

    KeyboardModifierMask = 0xfe000000,
    Combination(u32),
}
impl KeyboardModifier {
    pub fn or(self, other: KeyboardModifier) -> KeyboardModifier {
        let one: u32 = self.into();
        let other: u32 = other.into();
        Self::Combination(one | other)
    }

    pub fn has(&self, has: KeyboardModifier) -> bool {
        match self {
            Self::Combination(mask) => {
                let has: u32 = has.into();
                mask & has > 0
            },
            _ => *self == has,
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