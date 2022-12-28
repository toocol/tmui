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
        let other = self.as_u32();
        Self::Combination(one | other)
    }

    pub fn has(&self, has: KeyboardModifier) -> bool {
        match self {
            Self::Combination(mask) => {
                let has = has.as_u32();
                mask & has > 0
            },
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